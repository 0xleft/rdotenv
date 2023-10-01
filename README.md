# rdotenv

A very simple .env file parser my rust projects.

## Usage

main.rs
```rust
use rdotenv::DotEnv;

fn main() {
    let mut env = DotEnv::new();
    env.load();

    println!("{}", env.get("TEST_KEY"));
}
```

.env
```bash
TEST_KEY=TEST_VALUE
```

output
```bash
TEST_VALUE
```

## License

MIT License

## Contributing

Feel free.