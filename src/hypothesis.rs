use crate::annotation::{Annotation, Section};
use hypothesis::{
    annotations::{Annotation as HypoAnn, SearchQuery, Selector},
    Hypothesis,
};
use serde::Deserialize;
use std::{cmp::Eq, collections::HashMap};

#[derive(Deserialize, Clone)]
pub(crate) struct HypothesisConfig {
    user: String,
    token: String,
}

#[derive(Hash, Eq, PartialEq)]
struct ID(String, String);

fn parse_hypothesis(input: Vec<HypoAnn>, buffer: &mut HashMap<ID, Vec<Annotation>>) {
    for item in input.into_iter() {
        let uri = item.uri;
        let note = item.text;
        let tags = item.tags;
        let title = item.document.map(|doc| doc.title.concat()).unwrap_or_default();
        let mut text = String::new();
        'find: for target in item.target.into_iter() {
            for selector in target.selector.into_iter() {
                if let Selector::TextQuoteSelector(quote) = selector {
                    text = quote.exact;
                    break 'find;
                }
            }
        }
        let ann = Annotation { text, note, tags };
        let id = ID(uri, title);
        let entry = buffer.entry(id).or_insert_with(Vec::new);
        entry.push(ann);
    }
}

fn collect(buffer: HashMap<ID, Vec<Annotation>>) -> Vec<Section> {
    let mut res = Vec::new();
    for (ID(_uri, title), annotations) in buffer.into_iter() {
        res.push(Section { title, annotations });
    }
    res
}

async fn list_all(user: &str, token: &str) -> anyhow::Result<Vec<Section>> {
    let mut buffer = HashMap::new();
    let api = Hypothesis::new(user, token)?;
    let mut query = SearchQuery::builder()
        .user(format!("acct:{}@hypothes.is", user))
        .build()?;
    let resp = api.search_annotations_return_all(&mut query).await?;
    let mut end = resp.last().map(|ann| ann.updated.to_rfc3339());
    parse_hypothesis(resp, &mut buffer);
    while let Some(end_time) = end {
        let mut query = SearchQuery::builder()
            .user(format!("acct:{}@hypothes.is", user))
            .search_after(end_time)
            .build()?;
        let resp = api.search_annotations_return_all(&mut query).await?;
        end = resp.last().map(|ann| ann.updated.to_rfc3339());
        parse_hypothesis(resp, &mut buffer);
    }
    Ok(collect(buffer))
}

pub fn print() -> anyhow::Result<String> {
    let HypothesisConfig { user, token } = crate::config::CONFIG
        .hypothesis
        .as_ref()
        .expect("No hypothes.is access token configured!");
    let runtime = tokio::runtime::Runtime::new()?;
    let secs = runtime.block_on(list_all(user, token))?;
    let mut res = String::new();
    for mut sec in secs.into_iter() {
        sec.remove_old();
        if sec.has_annotation() {
            res.push_str(&sec.to_md());
            res.push('\n');
        }
        sec.mark_as_old();
    }
    Ok(res)
}
