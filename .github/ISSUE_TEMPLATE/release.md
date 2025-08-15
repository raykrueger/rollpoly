---
name: Release Planning
about: Plan a new release
title: 'Release v[VERSION]'
labels: 'release'
assignees: ''

---

## Release Checklist

### Pre-Release
- [ ] All planned features/fixes are merged
- [ ] All tests are passing
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)
- [ ] Version bump type determined: `patch` | `minor` | `major`

### Release Process
- [ ] Create release via GitHub Actions workflow OR
- [ ] Run `./scripts/bump-version.sh [patch|minor|major]`
- [ ] Verify release artifacts are built correctly
- [ ] Test release binaries on target platforms

### Post-Release
- [ ] Announce release (if applicable)
- [ ] Update any dependent projects
- [ ] Close related issues

## Changes in This Release

### New Features
- 

### Bug Fixes
- 

### Breaking Changes
- 

### Other Changes
- 

## Testing Notes

- [ ] Tested on Linux AMD64
- [ ] Tested on Linux ARM64  
- [ ] Tested on Windows AMD64
- [ ] Tested on Windows ARM64

## Additional Notes
