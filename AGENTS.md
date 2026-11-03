# mullard-zx8401

Read PRINCIPLES.md first. This repository implements the Mullard ZX8401 as a
standalone dependency-free library under Emu198x's workspace-shape decision.

Model hardware signals at the evidence-backed observation granularity. There
is no instruction-level abstraction and no machine glue in this library.
The public API evaluates settled pin levels; it claims no propagation timing.
Primary-source pin directions and original gate evidence are linked in README.md.

Before committing: cargo fmt --all --check, cargo clippy --workspace --all-targets
-- -D warnings, cargo test --workspace. Never bypass hooks. Land later changes
through pull requests with conventional commit subjects and rebase merges.
Public API/dependency changes require agreement under the umbrella AGENTS.md.
