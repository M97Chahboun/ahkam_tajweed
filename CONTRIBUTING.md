# Contributing to Ahkam Tajweed

Thank you for your interest in contributing to Ahkam Tajweed! Contributions are welcome, especially improvements to Tajweed accuracy, tests, documentation, performance, and the CLI.

## Code of Conduct

Please be respectful, constructive, and patient in all interactions.

## Getting Started

1. Fork the repository.
2. Clone your fork:

   ```bash
   git clone https://github.com/M97Chahboun/ahkam_tajweed.git
   cd ahkam_tajweed
   ```

3. Create a focused branch:

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. Install Rust through [rustup](https://rustup.rs/).

## Development Checks

Before opening a pull request, run:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

If a check cannot be run locally, mention that clearly in the pull request.

## Making Changes

- Keep pull requests focused and reasonably small.
- Follow standard Rust conventions and use meaningful names.
- Add or update tests for changed behavior, including edge cases.
- Preserve Unicode-aware indexing and narration-specific behavior.
- For Tajweed rule changes, include authoritative references or explain the scholarly basis and whether the behavior applies to Hafs, Warsh, or both.
- Update documentation and `CHANGELOG.md` when appropriate.
- Do not include Quranic text or other copyrighted data without confirming that its source permits redistribution.

## Pull Request Process

Please include:

- A clear summary of the problem and solution.
- The relevant issue number, when applicable.
- Tests and checks that were run.
- Any known limitations or follow-up work.

Maintainers may request changes to improve correctness, clarity, portability, or compatibility with the project's API and license.

## AI-Assisted Contributions

AI tools, including coding assistants, are **allowed with conditions**:

- Contributors remain fully responsible for the submitted code, documentation, tests, and claims.
- Review and understand every generated change before submitting it; do not submit unverified output.
- Run the relevant formatting, linting, tests, and documentation checks locally.
- For Tajweed rules, verify generated content against reliable scholarly sources and identify the sources in the pull request when relevant. AI output is not an authority for Quranic recitation rules.
- Do not submit private, personal, copyrighted, or otherwise sensitive material to an AI service.
- Do not use AI to fabricate test results, references, attribution, or contributor experience.
- Disclose substantial AI assistance in the pull request description, especially when it generated implementation, tests, or scholarly explanations.
- Maintainers may ask for a manual explanation or revision of AI-assisted code and may reject changes that cannot be responsibly verified.

AI assistance is not a substitute for human review, domain expertise, or compliance with this project's license and third-party licenses.

## Reporting Issues

When reporting a bug, include:

- A clear description and minimal reproduction.
- Expected and actual behavior.
- The narration and input text involved, where relevant.
- Rust version (`rustc --version`) and operating system.

## License

By contributing, you agree that your contributions will be licensed under the same dual-license terms as the project: [MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE), at your option. See [LICENSE](LICENSE) for details.
