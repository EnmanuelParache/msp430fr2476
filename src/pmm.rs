#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Management Module control register 0"]
    pub pmmctl0: PMMCTL0,
    #[doc = "0x02 - Power Management Module Control Register 1. Allows manual overwrite of predictive LDO settings."]
    pub pmmctl1: PMMCTL1,
    #[doc = "0x04 - Power Management Module Control Register 2"]
    pub pmmctl2: PMMCTL2,
    _reserved3: [u8; 0x04],
    #[doc = "0x0a - PMM interrupt flag register"]
    pub pmmifg: PMMIFG,
    _reserved4: [u8; 0x04],
    #[doc = "0x10 - Power mode 5 control register 0"]
    pub pm5ctl0: PM5CTL0,
}
#[doc = "PMMCTL0 (rw) register accessor: an alias for `Reg<PMMCTL0_SPEC>`"]
pub type PMMCTL0 = crate::Reg<pmmctl0::PMMCTL0_SPEC>;
#[doc = "Power Management Module control register 0"]
pub mod pmmctl0;
#[doc = "PMMCTL1 (rw) register accessor: an alias for `Reg<PMMCTL1_SPEC>`"]
pub type PMMCTL1 = crate::Reg<pmmctl1::PMMCTL1_SPEC>;
#[doc = "Power Management Module Control Register 1. Allows manual overwrite of predictive LDO settings."]
pub mod pmmctl1;
#[doc = "PMMCTL2 (rw) register accessor: an alias for `Reg<PMMCTL2_SPEC>`"]
pub type PMMCTL2 = crate::Reg<pmmctl2::PMMCTL2_SPEC>;
#[doc = "Power Management Module Control Register 2"]
pub mod pmmctl2;
#[doc = "PMMIFG (rw) register accessor: an alias for `Reg<PMMIFG_SPEC>`"]
pub type PMMIFG = crate::Reg<pmmifg::PMMIFG_SPEC>;
#[doc = "PMM interrupt flag register"]
pub mod pmmifg;
#[doc = "PM5CTL0 (rw) register accessor: an alias for `Reg<PM5CTL0_SPEC>`"]
pub type PM5CTL0 = crate::Reg<pm5ctl0::PM5CTL0_SPEC>;
#[doc = "Power mode 5 control register 0"]
pub mod pm5ctl0;
