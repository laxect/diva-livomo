use crate::annotation::{Annotation, Section};
use serde::Deserialize;
use serde_json::from_reader;
use std::{borrow::Cow, fs, path::PathBuf};

const FOLIATE: &str = "com.github.johnfactotum.Foliate";

fn foliate_dir() -> PathBuf {
    dirs::data_local_dir().expect("XDG DATA Not setting").join(FOLIATE)
}

#[derive(Deserialize)]
struct FoliateAnnotation {
    title: Option<Cow<'static, str>>,
    text: Cow<'static, str>,
    note: Cow<'static, str>,
}

impl Annotation for FoliateAnnotation {
    fn source(&self) -> Cow<str> {
        self.title.as_ref().unwrap_or(&("".into())).clone()
    }

    fn id(&self) -> Cow<str> {
        [self.source(), self.origin_text()].concat().into()
    }

    fn origin_text(&self) -> Cow<str> {
        self.text.clone()
    }

    fn annotation(&self) -> Cow<str> {
        self.note.clone()
    }
}

#[derive(Deserialize)]
struct FoliateMetadata {
    title: String,
}

#[derive(Deserialize)]
struct Foliate {
    metadata: FoliateMetadata,
    #[serde(default)]
    annotations: Vec<FoliateAnnotation>,
}

impl From<Foliate> for Section<'static, FoliateAnnotation> {
    fn from(fo: Foliate) -> Self {
        Self {
            title: fo.metadata.title.into(),
            annotations: fo.annotations,
            section_annotations: None,
        }
    }
}

fn load() -> anyhow::Result<Vec<Section<'static, FoliateAnnotation>>> {
    let mut res = Vec::new();
    for entry in fs::read_dir(foliate_dir())? {
        let entry = entry?;
        if entry
            .path()
            .extension()
            .map(|oss| oss.to_string_lossy().to_string())
            .eq(&Some("json".to_string()))
        {
            log::info!("load {}", entry.path().to_string_lossy());
            let piece_file = fs::File::open(entry.path())?;
            let piece: Foliate = from_reader(piece_file)?;
            res.push(piece.into());
        }
    }
    Ok(res)
}

pub fn print() -> anyhow::Result<Vec<Cow<'static, str>>> {
    let mut res = Vec::new();
    let fos = load().unwrap();
    for mut section in fos.into_iter() {
        section.remove_old();
        if section.has_annotation() {
            res.append(&mut section.to_md_frags());
        }
        section.mark_as_old();
    }
    Ok(res)
}
