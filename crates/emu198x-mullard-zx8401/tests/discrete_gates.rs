//! Independent pin-net oracle from Cygnus's replacement schematic, sheet 1.
//! Keep the 74LS157 pin ordering and IC7/IC6 gates visible: this checks the
//! packed implementation against a different representation of the circuit.

use emu198x_mullard_zx8401::{Inputs, Outputs, evaluate};

fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

fn ls157(i0: bool, i1: bool, select: bool) -> bool {
    (!select && i0) || (select && i1)
}

fn gate_network(p: Inputs) -> Outputs {
    let a = |pin: u32| p.address & (1_u16 << pin) != 0_u16;
    let display_select = nand(p.dras_n, p.dras_n); // IC7C
    let mut dma = 0;
    let mut ma = 0;
    for bit in 0..7 {
        dma |= u8::from(ls157(a(bit), a(bit + 7), display_select)) << bit;
        ma |= u8::from(ls157(a(bit + 7), a(bit), p.mreq_delayed_n)) << bit;
    }
    ma |= u8::from(ls157(p.ts1, p.ts2, p.mreq_delayed_n)) << 7;
    let ic7a = nand(p.wr_n, p.rd_n);
    let ic7b = nand(ic7a, a(15));
    let ic6c = ic7b || p.mreq_delayed_n;
    let ic6d = ic6c || p.mreq_n;
    Outputs {
        dma,
        ma,
        cas_n: ic6d,
    }
}

#[test]
fn every_address_and_control_combination_matches_discrete_gates() {
    let mut checked = 0_u64;
    for address in 0..=u16::MAX {
        for controls in 0..128_u8 {
            let p = Inputs {
                address,
                ts1: controls & 1 != 0,
                ts2: controls & 2 != 0,
                dras_n: controls & 4 != 0,
                rd_n: controls & 8 != 0,
                wr_n: controls & 16 != 0,
                mreq_n: controls & 32 != 0,
                mreq_delayed_n: controls & 64 != 0,
            };
            assert_eq!(evaluate(p), gate_network(p), "{p:?}");
            checked += 1;
        }
    }
    assert_eq!(checked, 8_388_608);
}

#[test]
fn separate_mux_phases_and_type_straps_are_observable_without_cas() {
    let mut p = Inputs {
        address: 0x952a,
        ts1: false,
        ts2: true,
        dras_n: true,
        rd_n: true,
        wr_n: true,
        mreq_n: true,
        mreq_delayed_n: true,
    };
    assert_eq!(
        evaluate(p),
        Outputs {
            dma: 0x2a,
            ma: 0xaa,
            cas_n: true
        }
    );
    p.dras_n = false;
    assert_eq!(
        evaluate(p),
        Outputs {
            dma: 0x2a,
            ma: 0xaa,
            cas_n: true
        }
    );
    // Change A7–A13 independently of A0–A6 to distinguish both phases.
    p.address = 0x912a;
    assert_eq!(
        evaluate(p),
        Outputs {
            dma: 0x22,
            ma: 0xaa,
            cas_n: true
        }
    );
    p.mreq_delayed_n = false;
    assert_eq!(
        evaluate(p),
        Outputs {
            dma: 0x22,
            ma: 0x22,
            cas_n: true
        }
    );
}

#[test]
fn refresh_and_inactive_memory_requests_cannot_assert_cas() {
    let mut p = Inputs {
        address: 0x8000,
        ts1: false,
        ts2: false,
        dras_n: false,
        rd_n: true,
        wr_n: true,
        mreq_n: false,
        mreq_delayed_n: false,
    };
    assert!(evaluate(p).cas_n);
    p.rd_n = false;
    assert!(!evaluate(p).cas_n);
    p.mreq_delayed_n = true;
    assert!(evaluate(p).cas_n);
    p.mreq_delayed_n = false;
    p.mreq_n = true;
    assert!(evaluate(p).cas_n);
    p.mreq_n = false;
    p.address = 0x4000;
    assert!(evaluate(p).cas_n);
    p.address = 0x8000;
    p.rd_n = true;
    p.wr_n = false;
    assert!(!evaluate(p).cas_n);
}
