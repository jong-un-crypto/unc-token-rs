<p>
    <a href="https://crates.io/crates/unc-token"><img src="https://img.shields.io/crates/d/unc-token?style=flat-square&logo=unc&label=crates.io" alt="Crates.io (latest)"></a>
    <a href="https://docs.rs/unc-token/latest/unc_token"><img src="https://img.shields.io/docsrs/unc-token?style=flat-square" alt="Docs.rs"></a>
    <img src="https://img.shields.io/badge/rustc-1.68%2B-lightgray.svg?style=flat-square" alt="Rust Version">
</p>

# unc-token
unc-token is crate for work with [tokens](https://docs.unc.org/concepts/basics/tokens) in unc-protocol.

The crate includes UncToken type and constructors for converting data as UncToken and as u128 type values.

## Examples

### Basic

Add unc-token to your dependencies:

```bash
cargo add unc-token
```

Here is the basic usage of unc-token crate:

```rust
use unc_token::UncToken;

fn main() {
    const TEN_UNC: UncToken = UncToken::from_unc(10);

    assert_eq!(TEN_UNC.to_string(), "10.00 UNC");
    assert_eq!(TEN_UNC.as_near(), 10);
    assert_eq!(TEN_UNC.as_millinear(), 10000);
    assert_eq!(TEN_UNC.as_yoctounc(), 10000000000000000000000000);

    let input_str = "0.123456 UNC";
    let input_near: UncToken = input_str.parse().unwrap();
    assert_eq!(
        input_near,
        UncToken::from_yoctounc(123456000000000000000000)
    );

}
```

### serde support

In order to use UncToken in `serde`-serializable structs, enable `serde` feature:

```bash
cargo add unc-token --features serde
```

Here is the basic usage of unc-token crate with serde:

```rust
// When `serde` feature is enabled, UncToken can be used in serde-serializable structs.
// UncToken will be serialized to a token-precision u128 value encoded as string.
#[derive(serde::Serialize)]
struct TransferDetails {
    amount: UncToken,
}

fn main() {
    const TEN_UNC: UncToken = UncToken::from_unc(10);

    let details = TransferDetails { amount: TEN_UNC };
    assert_eq!(
        serde_json::to_string(&details).unwrap(),
        r#"{"amount":"10000000000000000000000000"}"#
    );
}
```

### borsh support

In order to use UncToken in `borsh`-serializable structs, enable `borsh` feature:

```bash
cargo add unc-token --features borsh
```

Here is the basic usage of unc-token crate with borsh:

```rust
use borsh::{to_vec, BorshSerialize};
use unc_token::UncToken;

#[derive(BorshSerialize)]
struct TransferDetails {
    amount: UncToken,
}

fn main() {
    const TEN_UNC: UncToken = UncToken::from_unc(10);

    let details = TransferDetails { amount: TEN_UNC };
    assert_eq!(
        to_vec(&details).unwrap(),
        vec![0, 0, 0, 74, 72, 1, 20, 22, 149, 69, 8, 0, 0, 0, 0, 0]
    );
}
```

## UncToken information

UNC is used to price computation and storage on the UNC infrastructure. The network charges transaction fees in UNC to process changes and transactions.

### License

This project is licensed under the [MIT license] and [Apache-2.0 license].

[MIT license]: https://github.com/unc/unc-token/blob/main/LICENSE-MIT
[Apache-2.0 license]:  https://github.com/unc/unc-token/blob/main/LICENSE-APACHE

