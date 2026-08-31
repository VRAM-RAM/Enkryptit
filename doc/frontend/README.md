# Frontend

This directory contains all the documentation (for devs) about the `frontend`.

## Outputs

**Outputs** are managed in `Cli`. Anyway, `frontend/` contains `treat_output.rs`, that contains an helper for treating output :
```rust
pub fn treat_output(output: Output) {}
```

This function is useful when you have another function that returns an `Output`. It takes it, and treats it (prints the corresponding message).