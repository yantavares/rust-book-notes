# Chapter 14 - Cargo and Crates.io

This chapter covers essential aspects of Cargo, the Rust package manager, and its integration with crates.io, the Rust package registry. It includes details on release profiles, documentation, and Cargo workspaces, among other topics.

## Release Profiles

Release profiles in Cargo allow specifying different settings for various development scenarios, enhancing build and runtime performance for development, testing, production, and more.

- **Default Profiles**:
  - `dev`: The default profile for development, including debug symbols without optimizations.
  - `release`: Used for production builds with optimizations and no debug symbols.

- **Special Profiles**:
  - `test`: Optimized for running tests, including debug symbols.
  - `bench`: Optimized for running benchmarks.
  - `doc`: Optimized for generating documentation.

- **Custom Profiles**: Customize settings for specific scenarios by adding profiles like `[profile.custom]` in the `Cargo.toml` file.

## Building and Running

- **Build Commands**:
  - `cargo build` for development builds.
  - `cargo build --release` for production builds.

- **Test and Benchmark**:
  - `cargo test` for testing.
  - `cargo bench` for benchmarks.

- **Documentation**:
  - `cargo doc` to generate documentation.
  - `cargo doc --open` to generate and view documentation.

## Documentation Comments

- Use `///` or `/** */` for documentation comments, supporting Markdown for formatting.
- Place crate-level documentation at the top of the crate root (`lib.rs`) using `//!`.
- Documentation comments can include examples with assertions to be tested via `cargo test`.

## Re-exporting Items

- Use `pub use` to re-export items, simplifying the API exposed to users.

## Cargo Workspaces

- Workspaces allow managing multiple related packages as a single entity, ideal for projects with multiple crates.

## Installing Binary Crates

- Use `cargo install` to download and install binary crates from crates.io, making them available from the command line.
- If the binary has `cargo` prefix, it can be run directly from the command line.
e.g., `cargo install cargo-something` installs the `cargo-something` binary, which can be run as `cargo something` from the command line.

## Additional Information

- For a detailed example of workspaces, refer to the `my_workspace` directory in this chapter.
- Explore the `Cargo.toml` file for configuring profiles, dependencies, and workspace settings.
