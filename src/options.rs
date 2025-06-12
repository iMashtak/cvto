pub trait Options<Opt> {
    fn set(&mut self, option: Opt);
}

pub mod java_properties {
    use std::collections::HashMap;

    use crate::options::Options;

    const KV_SEPARATOR: &str = "kv_separator";

    #[derive(Debug)]
    pub enum OutOption {
        KvSeparator(String),
    }

    pub struct OutOptions {
        inner: HashMap<String, OutOption>,
    }

    impl OutOptions {
        pub fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }

        pub fn get_kv_separator(&self) -> Option<&str> {
            self.inner.get(KV_SEPARATOR).map(|x| match x {
                OutOption::KvSeparator(x) => x.as_str(),
            })
        }
    }

    impl Options<OutOption> for OutOptions {
        fn set(&mut self, option: OutOption) {
            let name = match option {
                OutOption::KvSeparator(_) => KV_SEPARATOR,
            };
            self.inner.insert(name.to_string(), option);
        }
    }
}

pub mod protobuf {
    use std::collections::HashMap;

    use crate::options::Options;

    const INCLUDE: &str = "include";
    const INPUT: &str = "input";
    const MESSAGE: &str = "message";

    #[derive(Debug)]
    pub enum OutOption {
        Include(Vec<String>),
        Input(Vec<String>),
        Message(String),
    }

    pub struct OutOptions {
        inner: HashMap<String, OutOption>,
    }

    impl OutOptions {
        pub fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }

        pub fn get_include(&self) -> Vec<String> {
            self.inner
                .get(INCLUDE)
                .map(|x| match x {
                    OutOption::Include(y) => y.clone(),
                    _ => unreachable!(),
                })
                .unwrap_or(Vec::new())
        }

        pub fn get_input(&self) -> Vec<String> {
            self.inner
                .get(INPUT)
                .map(|x| match x {
                    OutOption::Input(y) => y.clone(),
                    _ => unreachable!(),
                })
                .unwrap_or(Vec::new())
        }

        pub fn get_message(&self) -> Option<String> {
            self.inner.get(MESSAGE).map(|x| match x {
                OutOption::Message(y) => y.clone(),
                _ => unreachable!(),
            })
        }
    }

    impl Options<OutOption> for OutOptions {
        fn set(&mut self, option: OutOption) {
            let name = match option {
                OutOption::Include(_) => INCLUDE,
                OutOption::Input(_) => INPUT,
                OutOption::Message(_) => MESSAGE,
            };
            self.inner.insert(name.to_string(), option);
        }
    }

    #[derive(Debug)]
    pub enum InOption {
        Include(Vec<String>),
        Input(Vec<String>),
        Message(String),
    }

    pub struct InOptions {
        inner: HashMap<String, InOption>,
    }

    impl InOptions {
        pub fn new() -> Self {
            Self {
                inner: HashMap::new(),
            }
        }

        pub fn get_include(&self) -> Vec<String> {
            self.inner
                .get(INCLUDE)
                .map(|x| match x {
                    InOption::Include(y) => y.clone(),
                    _ => unreachable!(),
                })
                .unwrap_or(Vec::new())
        }

        pub fn get_input(&self) -> Vec<String> {
            self.inner
                .get(INPUT)
                .map(|x| match x {
                    InOption::Input(y) => y.clone(),
                    _ => unreachable!(),
                })
                .unwrap_or(Vec::new())
        }

        pub fn get_message(&self) -> Option<String> {
            self.inner.get(MESSAGE).map(|x| match x {
                InOption::Message(y) => y.clone(),
                _ => unreachable!(),
            })
        }
    }

    impl Options<InOption> for InOptions {
        fn set(&mut self, option: InOption) {
            let name = match option {
                InOption::Include(_) => INCLUDE,
                InOption::Input(_) => INPUT,
                InOption::Message(_) => MESSAGE,
            };
            self.inner.insert(name.to_string(), option);
        }
    }
}
