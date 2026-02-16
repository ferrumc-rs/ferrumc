# Releases & Versioning

## Versioning

FerrumC follows [Semantic Versioning](https://semver.org/) (SemVer).

### Version Format

**`MAJOR.MINOR.PATCH`** — e.g., `1.4.2`

| Component | When to bump | Example |
|---|---|---|
| **MAJOR** | Breaking changes — config format changes, protocol rewrites, removed features | `1.0.0` → `2.0.0` |
| **MINOR** | New features, backwards compatible — new commands, new packet support, new config options | `1.0.0` → `1.1.0` |
| **PATCH** | Bug fixes only — no new features, no breaking changes | `1.0.0` → `1.0.1` |

### Pre-release Versions

A `0.x.x` major version means the project is not yet stable — any release can contain breaking changes.

Pre-release labels are appended with a hyphen to indicate the release isn't final:

| Label | Meaning | Example |
|---|---|---|
| `alpha` | Early preview. Features incomplete, expect breakage. | `v0.2.0-alpha.1` |
| `beta` | Feature complete but not fully tested. | `v0.2.0-beta.1` |
| `rc` | Release Candidate. "Believed to be ready, testing before making it official." | `v0.2.0-rc1` |
| *(none)* | Final release. Stable, tested, ready for use. | `v0.2.0` |

The typical progression is: **alpha → beta → rc → release**. Not all stages are required — most releases go straight from development to `rc` → final, or skip `rc` entirely for small patches.

If a bug is found in `rc1`, the fix gets tagged as `rc2`. Once the RC is validated, the final version is tagged (same commit or a new one).

### Versioning in Practice

FerrumC is currently at `0.1.0` (pre-stable). A realistic release flow:

1. Development happens on `master`, PRs get merged.
2. A set of changes is deemed worth releasing.
3. Tag `v0.1.0-rc1` to test the release pipeline and binaries.
4. If everything works, tag `v0.1.0` as the final release.
5. Bug fix needed? Fix on master, tag `v0.1.1`.
6. New feature? Tag `v0.2.0`.
7. Massive breaking change (stable protocol, config rewrite)? Tag `v1.0.0`.

## Creating a Release

1. Ensure `master` is in a releasable state (CI green).
2. Tag the commit:
   ```bash
   git tag v0.1.0        # final release
   git tag v0.1.0-rc1    # release candidate (for testing)
   ```
3. Push the tag:
   ```bash
   git push origin v0.1.0
   ```
4. The release workflow runs automatically: validate → build (4 platforms) → publish GitHub Release.
5. If validation or build fails, fix the issue on master, then tag again (e.g., `v0.1.0-rc2`).

## Retagging a Failed Release

To move a tag to a new commit (e.g., after fixing a build issue):

```bash
git tag -d v0.1.0-rc2                    # delete locally
git push origin :refs/tags/v0.1.0-rc2    # delete from remote
# fix the issue, commit, then:
git tag v0.1.0-rc2                       # retag on current HEAD
git push origin v0.1.0-rc2
```

Delete any draft/failed GitHub Release from the Releases page if one was created.

## Downloading Release Binaries

Latest release URL via GitHub API:
```bash
# Latest stable release (skips pre-releases)
curl -s https://api.github.com/repos/ferrumc-rs/ferrumc/releases/latest

# Direct download (predictable URL pattern)
curl -sL https://github.com/ferrumc-rs/ferrumc/releases/download/v0.1.0/ferrumc-v0.1.0-x86_64-unknown-linux-musl.tar.gz -o ferrumc.tar.gz
```
