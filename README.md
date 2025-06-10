# Enum Reflect Extern

[<img alt="github" src="https://img.shields.io/badge/github-hrykr%2Fenum--reflect--extern-blue?logo=github&label=github&link=https%3A%2F%2Fgithub.com%2Fhrykr%2Fenum-reflect-extern" height="20">](https://github.com/hrykr/enum-reflect_extern)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum_reflect_extern?logo=rust" height="20">](https://crates.io/crates/enum_reflect_extern)
[<img alt="crates.io" src="https://img.shields.io/docsrs/enum_reflect_extern?logo=docs.rs&label=docs.rs" height="20">](https://docs.rs/enum_reflect_extern/latest/enum_reflect_extern/)

## Dependency for [enum_reflect](https://crates.io/crates/enum_reflect)

Traits, structs and other for [enum_reflect](https://crates.io/crates/enum_reflect).

Trait EnumReflect automatically implements by `#[derive(EnumReflect)]`.


## Installation

`cargo add enum_reflect_extern`
> or
```toml
[dependencies]
enum_reflect_extern = "0.1"
```

## Example Usage

```rust
fn print_any_enum_fields(target_enum: impl EnumReflect) {
    for (field, value) in target_enum.get_named_fields() {
        println!("Field {}", field);
    }
}