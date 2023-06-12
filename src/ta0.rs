#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub ta0ctl: TA0CTL,
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    pub ta0cctl0: TA0CCTL0,
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    pub ta0cctl1: TA0CCTL1,
    #[doc = "0x06 - Timer_A Capture/Compare Control Register"]
    pub ta0cctl2: TA0CCTL2,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - TimerA register"]
    pub ta0r: TA0R,
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    pub ta0ccr0: TA0CCR0,
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    pub ta0ccr1: TA0CCR1,
    #[doc = "0x16 - Timer_A Capture/Compare Register"]
    pub ta0ccr2: TA0CCR2,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub ta0ex0: TA0EX0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub ta0iv: TA0IV,
}
#[doc = "TA0CTL (rw) register accessor: an alias for `Reg<TA0CTL_SPEC>`"]
pub type TA0CTL = crate::Reg<ta0ctl::TA0CTL_SPEC>;
#[doc = "TimerAx Control Register"]
pub mod ta0ctl;
#[doc = "TA0CCTL0 (rw) register accessor: an alias for `Reg<TA0CCTL0_SPEC>`"]
pub type TA0CCTL0 = crate::Reg<ta0cctl0::TA0CCTL0_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta0cctl0;
#[doc = "TA0CCTL1 (rw) register accessor: an alias for `Reg<TA0CCTL1_SPEC>`"]
pub type TA0CCTL1 = crate::Reg<ta0cctl1::TA0CCTL1_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta0cctl1;
#[doc = "TA0CCTL2 (rw) register accessor: an alias for `Reg<TA0CCTL2_SPEC>`"]
pub type TA0CCTL2 = crate::Reg<ta0cctl2::TA0CCTL2_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta0cctl2;
#[doc = "TA0R (rw) register accessor: an alias for `Reg<TA0R_SPEC>`"]
pub type TA0R = crate::Reg<ta0r::TA0R_SPEC>;
#[doc = "TimerA register"]
pub mod ta0r;
#[doc = "TA0CCR0 (rw) register accessor: an alias for `Reg<TA0CCR0_SPEC>`"]
pub type TA0CCR0 = crate::Reg<ta0ccr0::TA0CCR0_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta0ccr0;
#[doc = "TA0CCR1 (rw) register accessor: an alias for `Reg<TA0CCR1_SPEC>`"]
pub type TA0CCR1 = crate::Reg<ta0ccr1::TA0CCR1_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta0ccr1;
#[doc = "TA0CCR2 (rw) register accessor: an alias for `Reg<TA0CCR2_SPEC>`"]
pub type TA0CCR2 = crate::Reg<ta0ccr2::TA0CCR2_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta0ccr2;
#[doc = "TA0EX0 (rw) register accessor: an alias for `Reg<TA0EX0_SPEC>`"]
pub type TA0EX0 = crate::Reg<ta0ex0::TA0EX0_SPEC>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta0ex0;
#[doc = "TA0IV (rw) register accessor: an alias for `Reg<TA0IV_SPEC>`"]
pub type TA0IV = crate::Reg<ta0iv::TA0IV_SPEC>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta0iv;
