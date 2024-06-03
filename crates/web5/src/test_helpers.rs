use std::{fs, path::PathBuf};

use serde::de::DeserializeOwned;

#[derive(Debug, serde::Deserialize)]
pub struct TestVector<I, O> {
    pub description: String,
    pub input: I,
    pub output: O,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestVectorFile<I, O> {
    pub description: String,
    pub vectors: Vec<TestVector<I, O>>,
}

impl<I, O> TestVectorFile<I, O> {
    pub fn load_from_path(file_path: &str) -> TestVectorFile<I, O>
    where
        I: DeserializeOwned,
        O: DeserializeOwned,
    {
        let mut vector_path = PathBuf::from("../../web5-spec/test-vectors/");
        vector_path.push(file_path);
        let data = fs::read_to_string(vector_path).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}
