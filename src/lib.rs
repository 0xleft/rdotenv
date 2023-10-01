use std::fs;

pub struct DotEnv {
    pub pairs: Vec<DotEnvPair>,
}

pub struct DotEnvPair {
    pub key: String,
    pub value: String,
}

impl DotEnv {
    pub fn new() -> DotEnv {
        DotEnv {
            pairs: Vec::new(),
        }
    }

    pub fn load(&mut self, path: Option<&str>) {
        let path = match path {
            Some(path) => path,
            None => ".env",
        };

        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");

        for line in contents.lines() {
            let pair = DotEnv::parse_line(line);
            self.pairs.push(pair);
        }
    }

    pub fn parse_line(line: &str) -> DotEnvPair {
        let mut key = String::new();
        let mut value = String::new();

        let mut is_key = true;
        for c in line.chars() {
            if c == '=' {
                is_key = false;
                continue;
            }

            if is_key {
                key.push(c);
            } else {
                value.push(c);
            }
        }

        DotEnvPair::new(key, value)
    }
}

impl DotEnvPair {
    fn new(key: String, value: String) -> DotEnvPair {
        DotEnvPair {
            key,
            value,
        }
    }
}