# Integrating Orbit Analyzer with CI Systems

This guide shows how to integrate the Orbit Analyzer into various CI systems to automatically validate your .orbit files.

## Overview

Integrating Orbit Analyzer into your CI pipeline ensures that all your .orbit files meet your team's coding standards and follow best practices consistently. This guide covers integration with:

- GitHub Actions
- GitLab CI
- Jenkins
- Circle CI

## GitHub Actions

Here's an example of how to integrate Orbit Analyzer into your GitHub Actions workflow:

```yaml
name: Orbit Linting

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
      
      - name: Install Orbit Analyzer
        run: cargo install orlint
      
      - name: Run Orbit Analyzer
        run: orlint --check src/**/*.orbit
```

## GitLab CI

Add this to your `.gitlab-ci.yml` file:

```yaml
orbit-lint:
  image: rust:latest
  before_script:
    - apt-get update
    - apt-get install -y libfontconfig1-dev libfreetype6-dev
    - cargo install orlint
  script:
    - orlint --check src/**/*.orbit
```

## Jenkins

Create a Jenkinsfile with these steps:

```jenkinsfile
pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }
    stages {
        stage('Setup') {
            steps {
                sh 'apt-get update'
                sh 'apt-get install -y libfontconfig1-dev libfreetype6-dev'
                sh 'cargo install orlint'
            }
        }
        stage('Lint') {
            steps {
                sh 'orlint --check src/**/*.orbit'
            }
        }
    }
}
```

## Circle CI

Add this to your `.circleci/config.yml` file:

```yaml
version: 2.1
jobs:
  lint:
    docker:
      - image: cimg/rust:stable
    steps:
      - checkout
      - run:
          name: Install system dependencies
          command: sudo apt-get update && sudo apt-get install -y libfontconfig1-dev libfreetype6-dev
      - run:
          name: Install Orbit Analyzer
          command: cargo install orlint
      - run:
          name: Run Orbit Analyzer
          command: orlint --check src/**/*.orbit

workflows:
  version: 2
  build-and-test:
    jobs:
      - lint
```

## Pre-commit Hook

You can also set up a pre-commit hook to run the analyzer before each commit:

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Find all staged .orbit files
files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.orbit$')

if [ -n "$files" ]; then
  echo "Running Orbit Analyzer on staged .orbit files..."
  orlint --check $files
  
  if [ $? -ne 0 ]; then
    echo "Orbit Analyzer found issues. Please fix them before committing."
    exit 1
  fi
fi
```

Make sure to make the hook executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Configuration

You can customize the analyzer's behavior by creating a `.orlint.toml` file in your project root:

```toml
# .orlint.toml

# Specify rules to enable
enabled_rules = [
  "non-empty-template",
  "script-tag-required",
  "style-tag-syntax",
  "component-naming"
]

# Specify custom severity levels
[rule_severity]
"non-empty-template" = "error"
"script-tag-required" = "warning"

# Configure specific rules
[rules.component-naming]
pattern = "^[A-Z][a-zA-Z0-9]*$"
```

## Conclusion

By integrating Orbit Analyzer into your CI pipeline, you can ensure that all your .orbit components follow consistent standards and best practices. This improves code quality, reduces bugs, and makes your codebase more maintainable.

For detailed information about available rules and configuration options, refer to the [Custom Lint Rules](custom-lint-rules.md) documentation.
