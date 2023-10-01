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

    pub fn load(&mut self) {
        self.load_path(".env");
    }

    pub fn load_path(&mut self, path: &str) {
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");

        for line in contents.lines() {
            let pair = DotEnv::parse_line(line);
            self.pairs.push(pair);
        }
    }

    pub fn parse_line(line: &str) -> DotEnvPair {
        let mut split = line.split("=");
        if split.clone().count() != 2 {
            panic!("Invalid line: {}", line);
        }

        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();

        DotEnvPair::new(key, value)
    }

    pub fn get(&self, key: &str) -> &str {
        for pair in &self.pairs {
            if pair.key == key {
                return &pair.value;
            }
        }

        panic!("Key not found: {}", key);
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