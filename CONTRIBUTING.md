# Contributing to DisasterMesh

Thank you for taking the time to contribute!  The project follows a _best-effort_ community model; respectful collaboration is expected.

## Getting started

1. **Fork** the repo and clone your fork.
2. Install Rust ≥ 1.76 (stable).
3. Run the full test-suite:
   ```bash
   cargo test
   ```
4. Create a feature branch:  `git checkout -b feature/my-awesome-change`.

## Development tips

* Run `cargo clippy --all-targets --all-features -D warnings` before pushing.
* Format with `cargo fmt`.
* Integration tests live under `tests/` – use `#[ignore]` for long-running tests.
* For tracing logs: `RUST_LOG=debug cargo run`.

## Commit style

* Use imperative present tense: "Add Bluetooth transport stub".
* Squash trivial fix-up commits before opening the PR.

## Pull request checklist

- [ ] Code compiles and tests pass (`cargo test`).
- [ ] `cargo fmt` shows no diff.
- [ ] Unit tests added for new logic.
- [ ] Documentation updated (README, doc comments, etc.).

---

### Code of Conduct

Be kind.  Harassment, discrimination, or disrespectful behaviour will not be tolerated.  We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). 