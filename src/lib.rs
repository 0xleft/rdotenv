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
            if pair.is_none() {
                continue;
            }
            self.pairs.push(pair.unwrap());
        }
    }

    pub fn parse_line(line: &str) -> Option<DotEnvPair> {
        let mut split = line.split("=");
        if split.clone().count() != 2 {
            return None;
        }

        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();

        Some(DotEnvPair::new(key, value))
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