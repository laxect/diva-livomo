use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub text: String,
    pub note: String,
}

#[derive(Deserialize, Debug)]
pub struct Foliate {
    pub title: String,
    pub annotations: Vec<Annotation>,
}
