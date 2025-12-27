# Contributing to `postgres-dv`

Thank you for your interest in contributing to this project! Please review these guidelines before getting started.

## Issue Reporting

### When to Report an Issue

- You've discovered bugs but lack the knowledge or time to fix them
- You have feature requests but cannot implement them yourself

> ⚠️ **Important:** Always search existing open and closed issues before submitting to avoid duplicates.

### How to Report an Issue

1. Open a new issue
2. Provide a clear, concise title that describes the problem or feature request
3. Include a detailed description of the issue or requested feature

## Code Contributions

### When to Contribute

- You've identified and fixed bugs
- You've optimized or improved existing code
- You've developed new features that would benefit the community

### How to Contribute

1. Fork the repository and check out a secondary branch (such as `feat/awesome-feature` or `fix/great-fix`)

2. Make sure that `cargo` and `python` are installed and create a python virtual environment

   ```bash
   cargo --version
   python3 --version
   # with uv
   uv venv
   # with python
   python3 -m venv .venv
   # activate python environment
   ## on Linux/macOS
   source .venv/bin/activate
   ## on Windows
   .\.venv\Scripts\activate
   ```

3. Make your changes, test and fix formatting/linting errors

   ```bash
   make test
   make format
   make clippy-fix
   ```

   Ensure that all tests pass and that all files are correctly formatted and linted. 

   Add tests for new features.

4. Bump the version (python needed):

    ```bash
    make version-bump
    ```

4. Commit your changes

5. Submit a pull request, including a comprehensive description of your changes.

---

**Thank you for contributing!**
