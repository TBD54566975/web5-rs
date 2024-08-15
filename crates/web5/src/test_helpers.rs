use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf, sync::Mutex};

#[derive(Debug, serde::Deserialize)]
pub struct TestVector<I, O> {
    pub description: String,
    pub input: I,
    pub output: O,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestVectorFile<I, O> {
    #[allow(dead_code)]
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

#[macro_export]
macro_rules! test_name {
    () => {{
        let current_fn = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            &name[..name.len() - 3] // Strip off "::f"
        };

        let test_name = current_fn.split("::").last().unwrap();
        test_name
    }};
}

pub(crate) struct UnitTestSuite {
    tests: Mutex<Vec<String>>,
}

impl UnitTestSuite {
    pub(crate) fn new(name: &str) -> Self {
        let file_path = format!(
            "{}/../../tests/unit_test_cases/{}.json",
            env!("CARGO_MANIFEST_DIR"),
            name
        );
        let file_content =
            fs::read_to_string(file_path).expect("Failed to read test cases JSON file");

        Self {
            tests: Mutex::new(
                serde_json::from_str::<Vec<String>>(&file_content)
                    .expect("Failed to parse test cases JSON file"),
            ),
        }
    }

    pub(crate) fn include(&self, test_name: &str) {
        let mut tests = self.tests.lock().unwrap();
        if let Some(pos) = tests.iter().position(|x| *x == test_name) {
            tests.remove(pos);
        }
    }

    pub(crate) fn assert_coverage(&self) {
        let tests = self.tests.lock().unwrap();
        if !tests.is_empty() {
            panic!("The following test cases were not covered: {:?}", *tests);
        }
    }
}
