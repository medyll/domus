# Release Checklist — Domus Alpha

## Code Quality
- [x] 135 tests passing (35 cli · 8 core · 38 macro · 54 web)
- [x] Workspace builds cleanly (`cargo build --workspace --exclude hello-world`)
- [x] No `cargo clippy` errors (run before tag)
- [ ] `cargo clippy --workspace --exclude hello-world -- -D warnings`

## Documentation
- [x] README.md updated to reflect alpha status
- [x] CHANGELOG.md written
- [x] examples/hello-world/README.md
- [ ] Inline rustdoc on all public items (`cargo doc --no-deps`)

## WASM Build
- [ ] `cd examples/hello-world && wasm-pack build --target web`
- [ ] Open in browser and manually verify counter + todo list

## Crate Metadata (before `cargo publish`)
- [ ] Each `Cargo.toml` has `description`, `license`, `repository`
- [ ] `domius-core/Cargo.toml` — no path deps (swap to version when published)
- [ ] `domius-web/Cargo.toml` — no path deps
- [ ] `domius-cli/Cargo.toml` — no path deps

## Git
- [ ] All changes committed
- [ ] Tag `v0.1.0-alpha`
- [ ] Push tag

## Publish order (dependency graph)
1. `cargo publish -p domius-core`
2. `cargo publish -p domius-macro`
3. `cargo publish -p domius-web`
4. `cargo publish -p domius-cli`
