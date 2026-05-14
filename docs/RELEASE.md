# Release Process

The CLT LCD Remote Controller is shipped as platform-native bundles
(`.deb` / `.rpm` / `.AppImage` for Linux, `.msi` / `.exe` for Windows
and `.dmg` for macOS). Tauri does **not** cross-compile, so each OS
must be built on a runner of that OS. We delegate this to GitHub
Actions.

## Workflows

| File                                | Trigger                          | Purpose                                                 |
|-------------------------------------|----------------------------------|---------------------------------------------------------|
| `.github/workflows/ci.yml`          | Every push / pull request        | Frontend type-check + bundle, Rust fmt / clippy / tests |
| `.github/workflows/release.yml`     | Push of a `v*` tag, or manual    | Build Linux + Windows + macOS bundles, upload artefacts |

## Cutting a release

1. Bump the version in three places so they match:
   * `package.json` -> `version`
   * `src-tauri/Cargo.toml` -> `[package].version`
   * `src-tauri/tauri.conf.json` -> (the version is read from `Cargo.toml`)
2. Update `CHANGELOG.md` with a new section for the release.
3. Commit and push the changes on a branch, open a PR, and merge to
   `main` once CI is green.
4. Tag the merge commit and push the tag:

   ```bash
   git checkout main
   git pull
   git tag v1.0.5
   git push origin v1.0.5
   ```

5. The `Release` workflow runs automatically on the tag push:
   * Builds the app on `ubuntu-22.04`, `windows-latest` and
     `macos-latest`.
   * Creates a **draft** GitHub Release named
     `CLT LCD Remote Controller v1.0.5` and attaches every produced
     installer to it.
6. Open the draft release in the GitHub UI, edit the release notes
   if needed and click **Publish**.

## Manual / branch builds

To produce installers for a branch without publishing a release, open
the **Actions** tab on GitHub, select the **Release** workflow and
click **Run workflow**. The artefacts will be available under the run
summary as `clt-lcd-remote-controller-<platform>` zip downloads.

## Local builds

A local `npm run tauri:build` produces installers for **only** the
operating system you are running, because Tauri bundles use the host
WebView and installer toolchain. If you need a Windows installer, run
the build on a Windows machine (or rely on the Actions workflow).

## Code-signing (future work)

The current pipeline produces unsigned binaries. To enable signing:

* **Windows** - import a `.pfx` certificate into the runner (via a
  GitHub secret) and configure `tauri.conf.json` `bundle.windows`
  with the certificate thumbprint and a timestamp URL.
* **macOS** - add Apple Developer ID secrets
  (`APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`,
  `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`,
  `APPLE_TEAM_ID`) and pass them to `tauri-action`.

Both flows are documented at
<https://tauri.app/v2/distribute/sign/>.
