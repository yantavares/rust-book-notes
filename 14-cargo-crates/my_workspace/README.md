# Rust Workspace Example

This guide provides an overview of how to define and work with a workspace in Rust, leveraging Cargo's capabilities to manage multiple packages within a single workspace.

## Defining a Workspace

To define a workspace in Rust, create a `Cargo.toml` file at the root of your workspace directory. Configure your workspace by specifying its members in the `Cargo.toml` file like so:

```toml
[workspace]
members = [
    "path_to_package1",
    "path_to_package2",
    ...
]
```

The `members` field is an array listing the paths to the packages that are part of the workspace, relative to the `Cargo.toml` file.

## Building and Testing Packages

- **Building Packages**: Run `cargo build` at the root of the workspace to build all the packages within the workspace.
- **Testing Packages**: Use `cargo test` at the workspace root to test all the packages.
- **Specific Packages**: To build or test a specific package, either navigate to the package's directory and run the command, or use the `-p` option followed by the package name (e.g., `cargo build -p package_name`).

## Running a Package

To run the `main` binary of a package directly from the workspace root, use:

```bash
cargo run --package package_name
```

If not specified, `cargo run` executed in the root of the workspace will run the `main` binary of the package located at the workspace root, assuming there is one designated as such.

## Shared Resources

- **Shared Target Directory**: All packages in a workspace share the same `target` directory, allowing Cargo to build dependencies once and share the compiled code among packages.
- **Shared Cargo.lock**: The workspace packages also share a single `Cargo.lock` file, ensuring that all packages use the same versions of dependencies.
