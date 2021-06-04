use blake3::{Hash, Hasher};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub text: String,
    pub note: String,
}

impl Annotation {
    pub fn hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        hasher.update(self.text.as_bytes());
        hasher.update(self.note.as_bytes());
        hasher.finalize()
    }

    pub fn to_md(&self) -> String {
        let mut res = ["> ", &self.text, "\n\n"].concat();
        if !self.note.is_empty() {
            res.push_str(&[&self.note, "\n\n"].concat());
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
}
