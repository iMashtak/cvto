pub trait Options<Opt> {
    fn set(&mut self, option: Opt);
}

pub mod java_properties {
    use std::collections::HashMap;

    use crate::options::Options;

    #[derive(Debug)]
    pub enum SerOption {
        KvSeparator(String),
    }

    const KV_SEPARATOR: &str = "kv_separator";

    pub struct SerOptions {
        inner: HashMap<String, SerOption>,
    }

    impl SerOptions {
        pub fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }

        pub fn get_kv_separator(&self) -> Option<&str> {
            self.inner.get(KV_SEPARATOR).map(|x| match x {
                SerOption::KvSeparator(x) => x.as_str(),
            })
        }
    }

    impl Options<SerOption> for SerOptions {
        fn set(&mut self, option: SerOption) {
            let name = match option {
                SerOption::KvSeparator(_) => KV_SEPARATOR,
            };
            self.inner.insert(name.to_string(), option);
        }
    }
}
