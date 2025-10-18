# Git Branching Strategy

This document describes the Git branching strategy used for the Integer Overflow Protection project.

## Branch Types

### 1. Main Branches

- **main**: The primary branch that contains production-ready code
- **development**: The integration branch for features and changes

### 2. Supporting Branches

- **Feature branches**: Used for developing new features
- **Release branches**: Used for preparing releases
- **Hotfix branches**: Used for urgent fixes to production releases

## Branching Model

### Feature Branches

Feature branches are used to develop new features or enhancements. They are created from the `development` branch and merged back into `development` when complete.

Naming convention: `feature/<feature-name>`

Examples:
- `feature/integer-overflow-protection`
- `feature/security-analysis`
- `feature/testing`

### Release Branches

Release branches are used to prepare for production releases. They are created from the `development` branch when it's time to release.

Naming convention: `release/v<major>.<minor>.<patch>`

Example: `release/v1.0.0`

### Hotfix Branches

Hotfix branches are used to fix critical issues in production releases. They are created from the `main` branch.

Naming convention: `hotfix/<issue-description>`

Example: `hotfix/critical-security-patch`

## Workflow

1. Create a feature branch from `development`
2. Develop and test the feature
3. Create a pull request to merge the feature branch into `development`
4. Once all features for a release are integrated, create a release branch from `development`
5. Test the release branch thoroughly
6. Merge the release branch into both `main` and `development`
7. Tag the release in the `main` branch
8. For urgent fixes, create a hotfix branch from `main`
9. Merge the hotfix branch into both `main` and `development`

## Current Branches

- `main`: Production-ready code
- `development`: Integration branch for ongoing development
- `feature/integer-overflow-protection`: Implementation of integer overflow protection
- `feature/security-analysis`: Security analysis and documentation
- `feature/testing`: Implementation of comprehensive testing
- `release/v1.0.0`: Preparation for first release

## Best Practices

1. Always create feature branches from `development`
2. Keep feature branches short-lived
3. Regularly sync feature branches with `development` to avoid merge conflicts
4. Use descriptive branch names
5. Delete branches after merging
6. Use pull requests for code review before merging
7. Tag releases in the `main` branch

This branching strategy follows the GitFlow model, which provides a robust framework for managing releases and development.