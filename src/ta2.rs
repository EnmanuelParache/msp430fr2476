#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub ta2ctl: TA2CTL,
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    pub ta2cctl0: TA2CCTL0,
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    pub ta2cctl1: TA2CCTL1,
    #[doc = "0x06 - Timer_A Capture/Compare Control Register"]
    pub ta2cctl2: TA2CCTL2,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - TimerA register"]
    pub ta2r: TA2R,
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    pub ta2ccr0: TA2CCR0,
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    pub ta2ccr1: TA2CCR1,
    #[doc = "0x16 - Timer_A Capture/Compare Register"]
    pub ta2ccr2: TA2CCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub ta2ex0: TA2EX0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub ta2iv: TA2IV,
}
#[doc = "TA2CTL (rw) register accessor: an alias for `Reg<TA2CTL_SPEC>`"]
pub type TA2CTL = crate::Reg<ta2ctl::TA2CTL_SPEC>;
#[doc = "TimerAx Control Register"]
pub mod ta2ctl;
#[doc = "TA2CCTL0 (rw) register accessor: an alias for `Reg<TA2CCTL0_SPEC>`"]
pub type TA2CCTL0 = crate::Reg<ta2cctl0::TA2CCTL0_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta2cctl0;
#[doc = "TA2CCTL1 (rw) register accessor: an alias for `Reg<TA2CCTL1_SPEC>`"]
pub type TA2CCTL1 = crate::Reg<ta2cctl1::TA2CCTL1_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta2cctl1;
#[doc = "TA2CCTL2 (rw) register accessor: an alias for `Reg<TA2CCTL2_SPEC>`"]
pub type TA2CCTL2 = crate::Reg<ta2cctl2::TA2CCTL2_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta2cctl2;
#[doc = "TA2R (rw) register accessor: an alias for `Reg<TA2R_SPEC>`"]
pub type TA2R = crate::Reg<ta2r::TA2R_SPEC>;
#[doc = "TimerA register"]
pub mod ta2r;
#[doc = "TA2CCR0 (rw) register accessor: an alias for `Reg<TA2CCR0_SPEC>`"]
pub type TA2CCR0 = crate::Reg<ta2ccr0::TA2CCR0_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta2ccr0;
#[doc = "TA2CCR1 (rw) register accessor: an alias for `Reg<TA2CCR1_SPEC>`"]
pub type TA2CCR1 = crate::Reg<ta2ccr1::TA2CCR1_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta2ccr1;
#[doc = "TA2CCR2 (rw) register accessor: an alias for `Reg<TA2CCR2_SPEC>`"]
pub type TA2CCR2 = crate::Reg<ta2ccr2::TA2CCR2_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta2ccr2;
#[doc = "TA2EX0 (rw) register accessor: an alias for `Reg<TA2EX0_SPEC>`"]
pub type TA2EX0 = crate::Reg<ta2ex0::TA2EX0_SPEC>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta2ex0;
#[doc = "TA2IV (rw) register accessor: an alias for `Reg<TA2IV_SPEC>`"]
pub type TA2IV = crate::Reg<ta2iv::TA2IV_SPEC>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta2iv;
