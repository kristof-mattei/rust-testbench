# Rust seed application

This is a framework for building Rust applications

It supports:

- Building multi-platform images
- Reusing images when merging in PRs to preserve provenance
  - Support for tags like `pr-${PR_NUMBER}-latest` (last build on PR), `edge` (last build on `main`), `pr-${SHA_MAIN_HEAD}-${SHA_PR_HEAD}` (uniquely identifying the merge result of a PR)
- Container attestation
- Building Debian packages (`amd64` and `arm64`) with `cargo-deb`, checked with `lintian`
  - Enabled with the `HAS_DEB` repository variable, built against the Dockerfile's Debian image
  - Stored next to the container image as `ghcr.io/${OWNER}/${REPO}-deb`, re-tagged along with the images and attested at every step
- Crate publishing
- Release publishing
  - Changelog curation through checkboxes in the release PR
  - Crate publishing to crates.io
  - Container re-tagging to `:latest`
  - Debian packages attached to the GitHub release

## TODO

- [ ] Remove old containers when the new one gets build for a PR?<br />
      Or rely on a general weekly untagged cleanup?
- [ ] Remove PR containers when PR closed<br />

## License

MIT, see [LICENSE](./LICENSE)

`SPDX-License-Identifier: MIT`
