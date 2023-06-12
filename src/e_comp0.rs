#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cp0ctl0: CP0CTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cp0ctl1: CP0CTL1,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    pub cp0int: CP0INT,
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    pub cp0iv: CP0IV,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    pub cp0dacctl: CP0DACCTL,
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    pub cp0dacdata: CP0DACDATA,
}
#[doc = "CP0CTL0 (rw) register accessor: an alias for `Reg<CP0CTL0_SPEC>`"]
pub type CP0CTL0 = crate::Reg<cp0ctl0::CP0CTL0_SPEC>;
#[doc = "Comparator Control Register 0"]
pub mod cp0ctl0;
#[doc = "CP0CTL1 (rw) register accessor: an alias for `Reg<CP0CTL1_SPEC>`"]
pub type CP0CTL1 = crate::Reg<cp0ctl1::CP0CTL1_SPEC>;
#[doc = "Comparator Control Register 1"]
pub mod cp0ctl1;
#[doc = "CP0INT (rw) register accessor: an alias for `Reg<CP0INT_SPEC>`"]
pub type CP0INT = crate::Reg<cp0int::CP0INT_SPEC>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cp0int;
#[doc = "CP0IV (rw) register accessor: an alias for `Reg<CP0IV_SPEC>`"]
pub type CP0IV = crate::Reg<cp0iv::CP0IV_SPEC>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cp0iv;
#[doc = "CP0DACCTL (rw) register accessor: an alias for `Reg<CP0DACCTL_SPEC>`"]
pub type CP0DACCTL = crate::Reg<cp0dacctl::CP0DACCTL_SPEC>;
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cp0dacctl;
#[doc = "CP0DACDATA (rw) register accessor: an alias for `Reg<CP0DACDATA_SPEC>`"]
pub type CP0DACDATA = crate::Reg<cp0dacdata::CP0DACDATA_SPEC>;
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cp0dacdata;
