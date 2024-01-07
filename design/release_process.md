
# Release Workflow

The version of all packages in the workspace should be bumped after a release.
The officially supported plugins in `plugins/` should also follow the entire
projects version. All packages, without exception have their version bumped,
regardless if the crate actually had any meaniful update between the last
release.

First bump version of all packages in workspace (careful not to replace other package numbers)
```sh
fd -e toml . . | xargs sed -i 's/0\.0\.x/0\.0\.y/'
```

Ensure the following are updates
- package numbers
- dependencies to packages
- READMEs
- docs

