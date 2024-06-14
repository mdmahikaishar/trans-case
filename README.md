# Trans Case

![Rust](https://img.shields.io/badge/Rust-DD3515?style=for-the-badge&logo=rust&logoColor=white)

Transform case.


## Get Strated

```bash
cargo add trans-case
```


## Example

```rs
use trans_case::{TransCase, Case};

let sentence = TransCase::new("trans-case in rust");

println!("{}", sentence.case(Case::Upper)); // TRANS CASE IN RUST
println!("{}", sentence.case(Case::Title)); // Trans Case In Rust
println!("{}", sentence.case(Case::Camel)); // transCaseInRust
```


## Contributing

Contributions are welcome! I would like you to contribute in this project.


## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](https://github.com/mdmahikaishar/trans-case/issues) section for a list of features, enhancements, and bug fixes that are planned.


## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/mdmahikaishar/trans-case/LICENSE) file for details.