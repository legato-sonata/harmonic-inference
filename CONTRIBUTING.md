# Contributing to Harmonic Inference

Thank you for your interest in contributing to Harmonic Inference. This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites
- Rust 1.70+
- Python 3.10+
- Docker
- Git

### Local Environment

1. Clone the repository:
```bash
git clone https://github.com/legato-sonata/harmonic-inference.git
cd harmonic-inference
```

2. Setup Rust environment:
```bash
rustup update
cargo build --all
```

3. Setup Python environment:
```bash
python -m venv venv
source venv/bin/activate
pip install -r ml_pipeline/requirements.txt
```

## Code Style

### Rust
- Format with `cargo fmt`
- Lint with `cargo clippy -- -D warnings`
- Document public APIs with doc comments

### Python
- Format with `black`
- Lint with `pylint`
- Type check with `mypy`

## Testing Requirements

All code must include tests:

### Rust
```bash
cargo test --all
```

### Python
```bash
pytest ml_pipeline/tests -v --cov
```

## Commit Guidelines

1. Use descriptive commit messages
2. Reference issues: "Fixes #123"
3. Keep commits atomic and logical
4. Write in present tense: "Add feature" not "Added feature"

## Pull Request Process

1. Create a feature branch: `git checkout -b feature/my-feature`
2. Make your changes with tests and documentation
3. Ensure all tests pass locally
4. Push to your fork
5. Open a PR with a clear description
6. Address review feedback
7. Squash commits before merge if requested

## Architecture Decisions

- Performance-critical code goes in Rust
- ML experimentation happens in Python
- Configuration files use YAML
- API contracts defined in proto files

## Reporting Issues

Include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, Python version)
- Relevant logs or error messages

## Feature Requests

Describe:
- Use case and motivation
- Proposed solution
- Alternative approaches considered
- Potential impact on performance/complexity

## Questions?

Feel free to open an issue for questions or discussions about the project.
