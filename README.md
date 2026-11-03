# Mullard ZX8401

[![CI](https://github.com/emu198x/mullard-zx8401/actions/workflows/ci.yml/badge.svg)](https://github.com/emu198x/mullard-zx8401/actions/workflows/ci.yml)

`emu198x-mullard-zx8401` evaluates the settled address and CAS outputs of the
Mullard ZX8401 memory controller used on ZX Spectrum Issue 5 and 6A boards.
It is dependency-free and `no_std`.

```rust
use emu198x_mullard_zx8401::{Inputs, evaluate};

let output = evaluate(Inputs {
    address: 0x9234,
    ts1: false,
    ts2: true,
    dras_n: true,
    rd_n: false,
    wr_n: true,
    mreq_n: false,
    mreq_delayed_n: false,
});
assert_eq!(output.dma, 0x34);
assert_eq!(output.ma, 0x24);
assert!(!output.cas_n);
```

## Signals and scope

`*_n` fields hold electrical levels: `false` is low. The host supplies DRASL
and delayed MREQDL as independent inputs. The library has no clock or saved state.

| Output | Select high | Select low |
|---|---|---|
| DMA0–6, selected by DRASL | A0–6 | A7–13 |
| MA0–6, selected by MREQDL | A0–6 | A7–13 |
| MA7, selected by MREQDL | TS2 | TS1 |

CASL is low only when MREQL and MREQDL are low, A15 is high, and RD or WR is
low. A14 does not connect to the chip. Refresh with both RD and WR high cannot
assert CASL. Outputs remain defined even when CASL is inactive.

This is a settled combinational model. It does not claim nanosecond propagation
accuracy or model transient hazards. Board wiring must supply delayed signals,
resolve display-memory bus priority through the resistors, and model DRAM,
ULA ownership and upper-RAM RAS externally. Sampling this function once per
emulator half-cycle does not qualify the board's analog timing margins.

## Evidence and verification

- Sinclair Research, *Specification of the Sinclair ZX8401 integrated circuit*,
  Rev 3, cover 24 April 1984, pages 2–3: signal directions and pin map.
  [Original scan, Ferranti Archives SPEC-SIN-DAT-046](https://retrotechwiki.com/images/2/21/Ferranti_Archives_SPEC-SIN-DAT-046_Sinclair_ZX8401_integrated_circuit_specification%2C_revision_3%2C_24_April_1984.pdf).
  The five-page scan lacks the timing diagrams referenced on page 5.
- Cygnus, [*ZX8401 discrete replacement*, sheet 1, IC2–IC7](https://cygnus.speccy.cz/download/nahrada_ZX8401/ic27-zx8401-replacement-ula2.pdf):
  inspectable multiplexer and CAS gate connections. The author identifies the
  design's ZXMUX ancestry and repair-PCB use. This replacement circuit is
  functional hardware precedent, not a measured trace of original silicon.

Tests compare all 8,388,608 address/control combinations against a separate
74LS157/74LS00/74LS32 gate-net implementation. Focused tests cover separate
mux phases, straps, refresh and inactive memory requests. These establish
settled digital equivalence to that network, not propagation equivalence.

## Development

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Licensed GPL-2.0-or-later.
