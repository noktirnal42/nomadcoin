# Contributing to NomadCoin

Thank you for your interest in contributing to NomadCoin! 🌍

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/nomadcoin.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Format code: `cargo fmt`
7. Run clippy: `cargo clippy -- -D warnings`
8. Commit with conventional commits
9. Push and open a PR

## 📝 Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples
```
feat(miner): add continuous mining mode
fix(wallet): correct address generation
docs(readme): update installation instructions
test(consensus): add validator selection tests
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_full_workflow

# Run with output
cargo test -- --nocapture
```

## 📐 Code Style

- Follow Rust idioms and conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Document public APIs with `///` comments
- Keep functions focused and small

## 🏗️ Architecture

See [Architecture](README.md#architecture) in the README.

## 🐛 Reporting Bugs

1. Check existing issues
2. Create a new issue with:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)

## 💡 Feature Requests

1. Check existing issues
2. Create a new issue with:
   - Clear description of the feature
   - Use case / motivation
   - Proposed implementation (if any)

## 🤝 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

---

Thank you for helping make NomadCoin better! 🙏
