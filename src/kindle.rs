use crate::annotation::{Annotation, Section};
use std::{collections::BTreeMap, fs, io::Read, path::PathBuf};

pub fn parse(clippings: PathBuf) -> anyhow::Result<String> {
    let mut res = String::new();
    let mut file = fs::File::open(clippings)?;
    let mut buffer = String::new();
    let mut output: BTreeMap<String, Vec<Annotation>> = BTreeMap::new();
    file.read_to_string(&mut buffer)?;
    for annotation in buffer.split("==========\r\n") {
        if annotation.is_empty() {
            continue;
        }
        let (annotation, title) = parse_annotation(annotation)?;
        let entry = output.entry(title).or_insert_with(Vec::new);
        entry.push(annotation);
    }
    for (title, annotations) in output.into_iter() {
        let mut section = Section { title, annotations };
        section.remove_old();
        if section.has_annotation() {
            res.push_str(&section.to_md());
            res.push('\n');
        }
        section.mark_as_old();
    }
    Ok(res)
}

fn parse_annotation(input: &str) -> anyhow::Result<(Annotation, String)> {
    let mut lines = input.lines();
    let title = lines
        .next()
        .map(|x| x.trim().trim_start_matches('\u{feff}'))
        .unwrap_or("");
    let tidied_note = lines.map(|s| if s.starts_with("- ") { "" } else { s }).collect();
    Ok((
        Annotation {
            text: tidied_note,
            note: String::new(),
            tags: Vec::new(),
        },
        title.to_owned(),
    ))
}
