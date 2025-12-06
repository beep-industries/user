# Contributing to User Service

Thank you for your interest in contributing to User Service! This document provides guidelines and instructions for contributing.

## Prerequisites

Before contributing, make sure you have the following installed:

- [Rust 1.90+](https://www.rust-lang.org/tools/install)
- [Docker & Docker Compose](https://docs.docker.com/get-docker/)
- [pre-commit](https://pre-commit.com/#install)
- Git

## Steps to Contribute

1. **Fork the repository**

   Click the "Fork" button on GitHub to create your own copy of the repository.

2. **Clone your fork locally**

   ```bash
   git clone https://github.com/your-username/user.git
   cd user
   ```

3. **Set up the development environment**

   Follow the instructions in the [README](README.md) to set up Keycloak and the database.

4. **Install pre-commit hooks**

   ```bash
   pre-commit install
   ```

5. **Create a branch for your changes**

   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

6. **Make your changes**

   - Write clear, readable code
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation if needed

7. **Run the pre-commit hooks**

   ```bash
   pre-commit run --all-files
   ```

8. **Commit your changes**

   Write clear and meaningful commit messages:

   ```bash
   git commit -m "feat: add user profile validation"
   git commit -m "fix: resolve authentication timeout issue"
   ```

9. **Push your branch**

   ```bash
   git push origin feature/your-feature-name
   ```

10. **Create a Pull Request**

    - Open a Pull Request against `main`
    - **Create the PR as a Draft** until all checks pass
    - Fill out the PR template completely
    - Mark as "Ready for review" only when everything is complete

11. **Wait for review**

    A maintainer will review your PR. Please be patient and address any feedback provided.

## Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions
- Keep functions small and focused

## Reporting Issues

When reporting issues, please include:

- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)

## License

By contributing to this project, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).

---

Thanks for contributing to Beep!
