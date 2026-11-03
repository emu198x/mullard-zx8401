//! Settled pin-level logic for the Mullard ZX8401 memory controller.
//!
//! The host supplies DRASL and the externally delayed MREQDL independently.
//! This combinational model neither advances a clock nor models propagation
//! delays, resistor arbitration, DRAM storage or ULA display-bus ownership.
//! Electrical level fields ending in `_n` are active low: `false` means low.
//!
//! Pin directions follow Sinclair's ZX8401 specification, revision 3,
//! 24 April 1984, pages 2–3. The settled network is independently inspectable
//! in Cygnus's discrete replacement schematic (IC2–IC7). See the crate README
//! for original-source links and the limits of that evidence.
//!
//! ```
//! use emu198x_mullard_zx8401::{Inputs, evaluate};
//! let pins = evaluate(Inputs {
//!     address: 0x9234,
//!     ts1: false,
//!     ts2: true,
//!     dras_n: true,
//!     rd_n: false,
//!     wr_n: true,
//!     mreq_n: false,
//!     mreq_delayed_n: false,
//! });
//! assert_eq!(pins.dma, 0x34);
//! assert_eq!(pins.ma, 0x24);
//! assert!(!pins.cas_n);
//! ```

#![no_std]

/// Electrical input levels at one settled observation point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inputs {
    /// CPU address. Only A0–A13 and A15 connect to this device; A14 is ignored.
    pub address: u16,
    /// Upper-memory type-selection strap TS1, selected for MA7 with MREQDL low.
    pub ts1: bool,
    /// Upper-memory type-selection strap TS2, selected for MA7 with MREQDL high.
    pub ts2: bool,
    /// Display-RAM row address strobe DRASL (pin 1).
    pub dras_n: bool,
    /// CPU read RDL (pin 38).
    pub rd_n: bool,
    /// CPU write WRL (pin 39).
    pub wr_n: bool,
    /// CPU memory request MREQL (pin 36).
    pub mreq_n: bool,
    /// Externally delayed memory request MREQDL (pin 35).
    pub mreq_delayed_n: bool,
}

/// Settled output levels before the surrounding board resolves bus ownership.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outputs {
    /// Display-RAM mux outputs DMA0–DMA6, packed in bits 0–6; bit 7 is zero.
    pub dma: u8,
    /// Upper-RAM mux outputs MA0–MA7, packed in bits 0–7.
    pub ma: u8,
    /// Upper-RAM column address strobe CASL (pin 34), active low.
    pub cas_n: bool,
}

/// Evaluate the settled combinational network without advancing time.
///
/// DRASL selects the display address independently of MREQDL's upper-memory
/// selection. Both select polarities and the TS straps matter even when CASL
/// is inactive. Refresh (neither RD nor WR asserted) does not assert CASL.
#[must_use]
pub const fn evaluate(input: Inputs) -> Outputs {
    let low = (input.address & 0x7f) as u8;
    let high = ((input.address >> 7) & 0x7f) as u8;
    let dma = if input.dras_n { low } else { high };
    let (address, strap) = if input.mreq_delayed_n {
        (low, input.ts2)
    } else {
        (high, input.ts1)
    };
    let upper_access = input.address & 0x8000 != 0 && (!input.rd_n || !input.wr_n);
    Outputs {
        dma,
        ma: address | ((strap as u8) << 7),
        cas_n: input.mreq_n || input.mreq_delayed_n || !upper_access,
    }
}
