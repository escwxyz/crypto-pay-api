# Contributing to crypto-pay-api

First off, thank you for considering contributing to crypto-pay-api! It's people like you that make crypto-pay-api such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- Use a clear and descriptive title
- Describe the exact steps which reproduce the problem
- Provide specific examples to demonstrate the steps
- Describe the behavior you observed after following the steps
- Explain which behavior you expected to see instead and why
- Include any error messages or stack traces

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- A clear and descriptive title
- A detailed description of the proposed enhancement
- Examples of how the enhancement would be used
- Any potential drawbacks or considerations

### Pull Requests

- Fork the repository and create your branch from `main`
- If you've added code that should be tested, add tests
- Ensure the test suite passes
- Update the documentation
- Create a pull request

## Development Setup

1. Fork and clone the repository
2. Install Rust (if you haven't already): https://rustup.rs/
3. Run `cargo build` to compile the project
4. Run `cargo test` to run the tests

## Coding Guidelines

### Rust Code Style

- Follow the Rust Style Guide
- Use `rustfmt` to format your code
- Use `clippy` to catch common mistakes and improve your code
- Write descriptive variable and function names
- Add comments for complex logic
- Include documentation for public APIs

### Documentation

- Use doc comments (`///`) for public APIs
- Include examples in documentation
- Document error cases and return values
- Mark code examples that make API calls with `no_run`
- Keep the README.md up to date

### Testing

- Write unit tests for new functionality
- Include integration tests where appropriate
- Mock external API calls in tests
- Test error cases as well as success cases

### Commit Messages

- Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification
- Use clear and descriptive commit messages
- Start with a verb in the present tense
- Keep the first line under 72 characters
- Reference issues and pull requests when relevant

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create a new GitHub release
4. Publish to crates.io

## Getting Help

If you need help with anything:

- Open an issue with your question
- Reach out to the maintainers
- Check the documentation and examples

## License

By contributing to crypto-pay-api, you agree that your contributions will be licensed under its MIT license.
