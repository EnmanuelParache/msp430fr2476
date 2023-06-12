#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub ta3ctl: TA3CTL,
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    pub ta3cctl0: TA3CCTL0,
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    pub ta3cctl1: TA3CCTL1,
    #[doc = "0x06 - Timer_A Capture/Compare Control Register"]
    pub ta3cctl2: TA3CCTL2,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - TimerA register"]
    pub ta3r: TA3R,
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    pub ta3ccr0: TA3CCR0,
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    pub ta3ccr1: TA3CCR1,
    #[doc = "0x16 - Timer_A Capture/Compare Register"]
    pub ta3ccr2: TA3CCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub ta3ex0: TA3EX0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub ta3iv: TA3IV,
}
#[doc = "TA3CTL (rw) register accessor: an alias for `Reg<TA3CTL_SPEC>`"]
pub type TA3CTL = crate::Reg<ta3ctl::TA3CTL_SPEC>;
#[doc = "TimerAx Control Register"]
pub mod ta3ctl;
#[doc = "TA3CCTL0 (rw) register accessor: an alias for `Reg<TA3CCTL0_SPEC>`"]
pub type TA3CCTL0 = crate::Reg<ta3cctl0::TA3CCTL0_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta3cctl0;
#[doc = "TA3CCTL1 (rw) register accessor: an alias for `Reg<TA3CCTL1_SPEC>`"]
pub type TA3CCTL1 = crate::Reg<ta3cctl1::TA3CCTL1_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta3cctl1;
#[doc = "TA3CCTL2 (rw) register accessor: an alias for `Reg<TA3CCTL2_SPEC>`"]
pub type TA3CCTL2 = crate::Reg<ta3cctl2::TA3CCTL2_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta3cctl2;
#[doc = "TA3R (rw) register accessor: an alias for `Reg<TA3R_SPEC>`"]
pub type TA3R = crate::Reg<ta3r::TA3R_SPEC>;
#[doc = "TimerA register"]
pub mod ta3r;
#[doc = "TA3CCR0 (rw) register accessor: an alias for `Reg<TA3CCR0_SPEC>`"]
pub type TA3CCR0 = crate::Reg<ta3ccr0::TA3CCR0_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta3ccr0;
#[doc = "TA3CCR1 (rw) register accessor: an alias for `Reg<TA3CCR1_SPEC>`"]
pub type TA3CCR1 = crate::Reg<ta3ccr1::TA3CCR1_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta3ccr1;
#[doc = "TA3CCR2 (rw) register accessor: an alias for `Reg<TA3CCR2_SPEC>`"]
pub type TA3CCR2 = crate::Reg<ta3ccr2::TA3CCR2_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta3ccr2;
#[doc = "TA3EX0 (rw) register accessor: an alias for `Reg<TA3EX0_SPEC>`"]
pub type TA3EX0 = crate::Reg<ta3ex0::TA3EX0_SPEC>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta3ex0;
#[doc = "TA3IV (rw) register accessor: an alias for `Reg<TA3IV_SPEC>`"]
pub type TA3IV = crate::Reg<ta3iv::TA3IV_SPEC>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta3iv;
