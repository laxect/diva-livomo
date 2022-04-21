use std::{borrow::Cow, collections::HashMap};

use crate::diff;
use blake3::{Hash, Hasher};
use serde::Deserialize;

pub trait Annotation {
    fn source(&self) -> Cow<str>;

    /// The unified id of annotation
    fn id(&self) -> Cow<str>;

    fn hash_id(&self) -> Hash {
        let id = self.id();
        let mut hasher = Hasher::new();
        hasher.update(id.as_bytes());
        hasher.finalize()
    }

    fn section_annotation(&self) -> Option<Cow<'static, str>> {
        None
    }

    fn origin_text(&self) -> Cow<str>;

    fn annotation(&self) -> Cow<str>;

    fn to_md(&self) -> Cow<'static, str> {
        let origin_text = self.origin_text();
        let origin_text = origin_text.trim_start_matches('\n').trim_end_matches('\n');
        let annotation = self.annotation();
        let annotation = annotation.trim_start_matches('\n').trim_end_matches('\n');

        let mut md = String::new();
        if !origin_text.is_empty() {
            md.push_str("> ");
            md.push_str(origin_text);
            md.push('\n');
        }
        if !origin_text.is_empty() && !annotation.is_empty() {
            md.push('\n');
        }
        if !annotation.is_empty() {
            md.push_str(origin_text);
            md.push('\n');
        }
        md.into()
    }

    fn is_empty(&self) -> bool {
        self.origin_text().is_empty() && self.annotation().is_empty()
    }
}

#[derive(Deserialize, Debug)]
pub struct Section<'a, T>
where
    T: Annotation,
{
    pub title: Cow<'a, str>,
    pub section_annotations: Option<Cow<'a, str>>,
    pub annotations: Vec<T>,
}

pub fn collect<T: Annotation>(buffer: HashMap<String, Vec<T>>) -> Vec<Section<'static, T>> {
    let mut res = Vec::new();
    for (title, annotations) in buffer.into_iter() {
        let section_annotations = annotations.first().and_then(Annotation::section_annotation).unwrap_or_else(|| "".into());
        let section_annotations = trim_section_annotations(section_annotations);
        res.push(Section {
            title: title.into(),
            annotations,
            section_annotations,
        });
    }
    res
}

pub fn trim_section_annotations(anno: Cow<str>) -> Option<Cow<str>> {
    if anno.is_empty() {
        return None;
    }
    let anno = anno.into_owned();
    let mut anno: Cow<str> = anno.trim_matches('\n').to_owned().into();
    anno.to_mut().push('\n');
    Some(anno)
}

impl<'a, T: Annotation> Section<'a, T> {
    pub fn has_annotation(&self) -> bool {
        !self.annotations.is_empty()
    }

    pub fn to_md_frags(&self) -> Vec<Cow<'a, str>> {
        let mut res = vec![format!("# {}\n", self.title).into()];
        if let Some(ref sanno) = self.section_annotations {
            res.push(sanno.clone());
        }
        for annotation in &self.annotations {
            res.push(annotation.to_md());
        }
        res
    }

    pub fn remove_old(&mut self) {
        self.annotations.retain(|ann| diff::is_new(ann.hash_id().as_bytes()));
    }

    pub fn mark_as_old(&self) {
        self.annotations.iter().for_each(|ann| {
            if let Err(e) = diff::add_key(ann.hash_id().as_bytes()) {
                log::error!("add key failed. {}", e);
            }
        });
    }
}
