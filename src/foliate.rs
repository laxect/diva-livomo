use crate::annotation::{Annotation, Section};
use serde::Deserialize;
use serde_json::from_reader;
use std::{fs, path::PathBuf};

const FOLIATE: &str = "com.github.johnfactotum.Foliate";

fn foliate_dir() -> PathBuf {
    dirs::data_local_dir().expect("XDG CONFIG Not setting").join(FOLIATE)
}

#[derive(Deserialize)]
struct FoliateMetadata {
    title: String,
}

#[derive(Deserialize)]
struct Foliate {
    metadata: FoliateMetadata,
    #[serde(default)]
    annotations: Vec<Annotation>,
}

impl From<Foliate> for Section {
    fn from(fo: Foliate) -> Self {
        Self {
            title: fo.metadata.title,
            annotations: fo.annotations,
        }
    }
}

fn load() -> anyhow::Result<Vec<Section>> {
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

pub fn print() -> anyhow::Result<String> {
    let mut res = String::new();
    let fos = load().unwrap();
    for mut item in fos.into_iter() {
        item.remove_old();
        if item.has_annotation() {
            res.push_str(&item.to_md());
            res.push('\n');
        }
        item.mark_as_old();
    }
    Ok(res)
}
