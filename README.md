# rust-feature-set-bug
Minimal case to show feature getting set, when it shouldn't.


see that `bin1/Cargo.toml` does not set "my_feature", but `bin2/Cargo.toml` does. When running `cargo run --bin bin1` it prints "true" for the feature being set, but it shouldn't, since it's not.
