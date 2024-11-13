# Contributing

When contributing to this repository, you'll have more luck with getting PRs approved if you come chat with us in the
Discord server and letting us know about what you are fixing/adding.

## Pull Request Process

1. Make sure all tests and lints pass. PRs that don't pass CI will be rejected if your code is the cause of the failing
tests/lints.
2. Make sure all needed files are also included and not using absolute paths.
3. Include a sufficient explanation of your PR. What is it adding/fixing, why does this feature need to be added/fixed,
who have you discussed this with, etc. If these questions were answered in a conversation on this Discord, mention who
you talked with and what consensus was reached. Unexplained PRs will rarely be accepted.
4. Check again that tests pass.
5. Check a 3rd time.
6. Check that Clippy passes with no issues. `cargo clippy --all-targets -- -Dwarnings` is used on CI.
7. Submit PR.

## Project specific guidelines
Just some rules to try to keep the repo nice and organised
### Branches
#### `rewrite/v3`
This branch is the main branch. This is where all PRs should be made to. This branch is the most up to
date and should only be merged into with completed features.
#### `feature/feature-name`
This branch is for developing a feature. Once the feature is complete, a PR should be
made to the dev branch. This branch should be branched off of the dev branch.
#### `fix/fixed-thing`
This branch is for fixing a bug. Once the bug is fixed, a PR should be made to the dev
branch. This branch should be branched off of the dev branch.
#### `rework/refactored-thing`
This branch is for refactoring code. Once the code is refactored, a PR should be made to the dev branch.
#### `housekeeping`
This branch is for stuff relating to the repo itself. This could be updating the README, adding
new CI checks, etc. This branch should be branched off of the dev branch.
#### `docs`
This branch is for updating the documentation. This branch should be branched off of the dev branch.
This is used for stuff that doesn't actually modify the code, but the documentation.

### Project Layout
```text
+---.etc                    | Non-code files
+---.github                 | GitHub specific files
+---assets                  | Assets for the Readme
+---src                     | Source code
|   +---bin                 | The main binary that stitches everything together
|   +---lib                 | The libraries that provide the business logic
|   |   +---adapters        | Adapters and parsers for data formats
|   |   +---core            | The core logic of the application
|   |   +---derive_macros   | Derive macros. Split into directories for each macro
|   |   +---ecs             | The ECS system
|   |   +---events          | The event system
|   |   +---net             | Networking code
|   |   +---plugins         | Plugins interface
|   |   +---storage         | Storage backend
|   |   +---utils           | Utility functions
|   |   \---world           | Code for interacting with the world
|   \---tests               | Unit tests
```
If you add a new directory, please add it to the above list along with its purpose.

### Code rules
1. Tests that only generate/dump data must be `#[ignore]`d. These tests are not useful for CI and should not be run.
2. No absolute paths. This will break the CI and make it harder to run the code on different machines.
3. Try to avoid just chaining `../` to get to the root of the project. This makes it harder to move files around and work
out where a referenced file is. There is a `root!()` macro that can be used to get the root of the project as a string.
4. Don't be lazy and use `unwrap()`. If you are sure that a value will always be `Some`, use `expect()`. If you are not
sure, use `match` or `if let`. Please also have a more detailed `error!()` message if you are using `expect()`.
5. Avoid `.clone()`ing data. If you need to clone data, make sure that it is necessary and that the data is not too large.
Cloning is ok however in sections of code that only need to run once and small performance hits are acceptable (eg, loading
config files, starting up the database).
6. New dependencies should be added to the workspace `Cargo.toml` file. This will make it easier to manage dependencies
and will make sure that all dependencies are of the same version.
7. If you are adding a new feature that warrants major separation, add it as a new crate and then include it in the
workspace `Cargo.toml` file. This will make it easier to manage the code and will make sure that the code is well
separated.
8. If you are adding an extra sub-crate, you must create a new set of `thiserror` based error types for that crate. This
will make it easier to understand where an error is coming from and will make it easier to handle errors.
9. Use `cargo clippy` to check for any issues with the code. This will be checked in CI and will cause the build to fail
if there are any issues. There is no excuse for *your* code to fail the lints.
10. Use `#[expect(lint)]` instead of `#[allow(lint)]` if you are sure that the lint is not an issue. This will make it
easier to find and remove these lints in the future.
11. Use `#[cfg(test)]` to only include code in tests. This will make the code easier to read and understand.
12. Where applicable, add doc strings to functions and modules. This will make it easier for others to understand the code.
Check https://doc.rust-lang.org/nightly/rustdoc/how-to-write-documentation.html for more information on how to write good
documentation.
13. Unsafe code is ok as long as it is well documented and the reason for the unsafe code is explained. If you are not sure
if the code is safe, ask in the Discord.
14. Limit the use of raw instructions as much as possible. This will make the code easier to read and understand. There
are some cases where raw instructions are needed, but these should be kept to a minimum.
15. You will be asked to fix your PR if folders like `.vscode` or `.idea` are included in the PR. These folders are
specific to your IDE and should not be included in the PR.
16. If you are adding a new feature, make sure to add tests for it. This will make sure that the feature works as expected
and will help prevent regressions in the future.
17. If you are fixing a bug, make sure to add a test that reproduces the bug. This will make sure that the bug is fixed
and will help prevent regressions in the future.
18. If your code isn't sufficiently documented, you will be asked to add documentation.
19. If your code doesn't have tests where it should, you will be asked to add tests.

## Code of Conduct

Please note we have a code of conduct, please follow it in all your interactions with the project.

## License

By contributing, you agree that your contributions will be licensed under the project's license.

### [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
