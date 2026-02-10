# Project Guidelines

## Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, semicolons, etc.)
- `refactor`: Code refactoring (no feature/fix)
- `perf`: Performance improvement
- `test`: Adding/updating tests
- `build`: Build system or dependencies
- `ci`: CI/CD configuration
- `chore`: Maintenance tasks

### Examples
```
feat(cli): add --verbose flag for debug output
fix(api): handle rate limiting from Linear API
ci: add GitHub Actions for releases and auto-PR
docs: update installation instructions
```

## Branch Strategy

- `master` - Production releases (protected)
- `dev` - Development branch (default)
- `feature/*` - New features → PR to `dev`
- `fix/*` - Bug fixes → PR to `dev`
- `refactor/*` - Refactoring → PR to `dev`
