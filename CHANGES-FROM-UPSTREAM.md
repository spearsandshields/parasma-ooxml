# Changes from upstream

This repository tracks [pterror/ooxml](https://github.com/pterror/ooxml). Upstream's history is
kept unchanged; the commits below sit on top of it and are rebased onto upstream's `master` when
the fork is synchronised. Each is intended to be offered upstream.

Base: upstream `master` at `28915f1` (2026-10-18).

## Fixes

- **Skip-body loops depth-track nested content** (`fix(codegen)`). The generated parser for an
  element with attributes but no modeled children stopped at the first nested end tag, which
  under-consumed elements such as `<c:extLst>` and silently truncated sibling lists (a
  three-series chart parsed as one). The generator and the committed generated parsers for all
  four schema crates now skip nested content by depth.
- **One canonical relationship-target resolver** (`fix(opc)`). `ooxml_opc::resolve_part_path`
  resolves relative and absolute-form targets to zip entry names; the wml, sml and pml crates
  delegate to it instead of carrying three divergent copies, two of which returned absolute
  targets verbatim.
- **A document without `w:body` reads as empty** (`fix(wml)`) instead of panicking in
  `Document::body()`; `body_mut()` creates the body on demand.

## Additions

- `DrawingExt::extent_emu()` returns a `<w:drawing>`'s `wp:extent` in EMU (`feat(wml)`).
- `parse_numbering()` parses `word/numbering.xml` from bytes (`feat(wml)`).
- `parse_chart_space()` parses a chart part from bytes, behind `dml-charts` (`feat(dml)`).

## Build and fixtures

- The `zip` dependency is built with `default-features = false, features = ["deflate"]`
  (`build(opc)`). OPC permits only DEFLATE or STORED entries, and the dropped codecs include
  `zstd-sys`, which does not cross-compile to `wasm32-unknown-unknown`.
- The 1×1 fixture PNG is solid red rather than white so a rendered image is distinguishable
  from a blank page (`fixtures(wml)`).
- Clippy lints new in Rust 1.95 are cleared so `cargo clippy --all-targets --all-features
  -- -D warnings` passes on the current stable toolchain (`chore`).
- The four codegen tests that read the ECMA-376 RELAX NG schemas skip, with a note, when the
  gitignored `spec/` download is absent instead of panicking on every CI runner (`test(codegen)`).

## Repository

- The README carries the fork notice and the crates.io and docs.rs badges are removed, since
  those describe upstream's published crates.
- `deploy-docs.yml` is removed; the documentation site is upstream's.
- `ci.yml` pins its actions by commit and declares `permissions: contents: read`.
- The workspace version is upstream's with a fork pre-release segment appended
  (`0.1.1-alpha.2.parasma.1`); the last number increments per fork release and resets when the
  upstream version changes.
