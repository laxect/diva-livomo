use crate::annotation::{collect, Annotation, Section};
use hypothesis_rs::{
    annotations::{Annotation as HypoAnn, SearchQuery, Selector},
    Hypothesis,
};
use secrecy::ExposeSecret;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap};

impl Annotation for HypoAnn {
    fn source(&self) -> Cow<str> {
        (&self.uri).into()
    }

    fn id(&self) -> Cow<str> {
        (&self.id).into()
    }

    fn origin_text(&self) -> Cow<str> {
        for target in &self.target {
            for selector in &target.selector {
                if let Selector::TextQuoteSelector(quote) = selector {
                    return Cow::Borrowed(&quote.exact);
                }
            }
        }
        "".into()
    }

    fn annotation(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    fn section_annotation(&self) -> Option<Cow<'static, str>> {
        Some((self.uri.clone()).into())
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct HypothesisConfig {
    user: String,
}

async fn list_all(user: &str, token: &str) -> anyhow::Result<Vec<Section<'static, HypoAnn>>> {
    let mut buffer: HashMap<String, Vec<HypoAnn>> = HashMap::new();

    let api = Hypothesis::new(user, token)?;
    let mut query = SearchQuery::builder().user(format!("acct:{user}@hypothes.is")).build()?;
    let resp = api.search_annotations_return_all(&mut query).await?;

    for anno in resp.into_iter() {
        if anno.is_empty() {
            continue;
        }
        let title = anno.document.as_ref().map(|doc| doc.title.concat());
        if let Some(title) = title {
            let entry = buffer.entry(title).or_default();
            entry.push(anno);
        }
    }

    Ok(collect(buffer))
}

pub fn print(token: secrecy::Secret<String>) -> anyhow::Result<Vec<Cow<'static, str>>> {
    let HypothesisConfig { user } = crate::config::CONFIG
        .hypothesis
        .as_ref()
        .expect("No hypothes.is access token configured!");
    let runtime = tokio::runtime::Runtime::new()?;
    let secs = runtime.block_on(list_all(user, token.expose_secret()))?;
    let mut res = Vec::new();
    for mut sec in secs.into_iter() {
        sec.remove_old();
        if sec.has_annotation() {
            res.append(&mut sec.to_md_frags());
        }
        sec.mark_as_old();
    }
    Ok(res)
}
