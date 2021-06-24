use crate::diff;
use blake3::{Hash, Hasher};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub text: String,
    pub note: String,
    // can be nothing
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Annotation {
    pub fn hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        hasher.update(self.text.as_bytes());
        hasher.update(self.note.as_bytes());
        hasher.finalize()
    }

    pub fn to_md(&self) -> String {
        let mut res;
        if !self.text.is_empty() {
            res = ["> ", &self.text, "\n\n"].concat();
        } else {
            res = String::new();
        }
        if !self.note.is_empty() {
            res.push_str(&[&self.note, "\n\n"].concat());
        }
        if !self.tags.is_empty() {
            for tag in self.tags.iter() {
                res.push_str(&["#", tag, " "].concat());
            }
            res.push('\n');
            res.push('\n');
        }
        res
    }
}

#[derive(Deserialize, Debug)]
pub struct Section {
    pub title: String,
    pub annotations: Vec<Annotation>,
}

impl Section {
    pub fn has_annotation(&self) -> bool {
        !self.annotations.is_empty()
    }

    pub fn to_md(&self) -> String {
        let mut res = format!("# {}\n\n", self.title);
        for annotation in self.annotations.iter() {
            res.push_str(&annotation.to_md())
        }
        res
    }

    pub fn remove_old(&mut self) {
        self.annotations.retain(|ann| diff::is_new(ann.hash().as_bytes()));
    }

    pub fn mark_as_old(&self) {
        self.annotations.iter().for_each(|ann| {
            if let Err(e) = diff::add_key(ann.hash().as_bytes()) {
                log::error!("add key failed. {}", e);
            }
        });
    }
}
