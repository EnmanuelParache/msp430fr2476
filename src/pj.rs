#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    pub pjin: PJIN,
    #[doc = "0x02 - Port J Output"]
    pub pjout: PJOUT,
    #[doc = "0x04 - Port J Direction"]
    pub pjdir: PJDIR,
    #[doc = "0x06 - Port J Resistor Enable"]
    pub pjren: PJREN,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - Port J Select 0"]
    pub pjsel0: PJSEL0,
    #[doc = "0x0c - Port J Select 1"]
    pub pjsel1: PJSEL1,
    _reserved6: [u8; 0x08],
    #[doc = "0x16 - Port J Complement Select"]
    pub pjselc: PJSELC,
}
#[doc = "PJIN (rw) register accessor: an alias for `Reg<PJIN_SPEC>`"]
pub type PJIN = crate::Reg<pjin::PJIN_SPEC>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT (rw) register accessor: an alias for `Reg<PJOUT_SPEC>`"]
pub type PJOUT = crate::Reg<pjout::PJOUT_SPEC>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR (rw) register accessor: an alias for `Reg<PJDIR_SPEC>`"]
pub type PJDIR = crate::Reg<pjdir::PJDIR_SPEC>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN (rw) register accessor: an alias for `Reg<PJREN_SPEC>`"]
pub type PJREN = crate::Reg<pjren::PJREN_SPEC>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJSEL0 (rw) register accessor: an alias for `Reg<PJSEL0_SPEC>`"]
pub type PJSEL0 = crate::Reg<pjsel0::PJSEL0_SPEC>;
#[doc = "Port J Select 0"]
pub mod pjsel0;
#[doc = "PJSEL1 (rw) register accessor: an alias for `Reg<PJSEL1_SPEC>`"]
pub type PJSEL1 = crate::Reg<pjsel1::PJSEL1_SPEC>;
#[doc = "Port J Select 1"]
pub mod pjsel1;
#[doc = "PJSELC (rw) register accessor: an alias for `Reg<PJSELC_SPEC>`"]
pub type PJSELC = crate::Reg<pjselc::PJSELC_SPEC>;
#[doc = "Port J Complement Select"]
pub mod pjselc;
