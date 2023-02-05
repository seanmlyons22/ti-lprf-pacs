#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub fetchstat: FETCHSTAT,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub cpustat: CPUSTAT,
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub wustat: WUSTAT,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub reg1_0: REG1_0,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub reg3_2: REG3_2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub reg5_4: REG5_4,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub reg7_6: REG7_6,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub loopaddr: LOOPADDR,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub loopcnt: LOOPCNT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "FETCHSTAT (rw) register accessor: an alias for `Reg<FETCHSTAT_SPEC>`"]
pub type FETCHSTAT = crate::Reg<fetchstat::FETCHSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fetchstat;
#[doc = "CPUSTAT (rw) register accessor: an alias for `Reg<CPUSTAT_SPEC>`"]
pub type CPUSTAT = crate::Reg<cpustat::CPUSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpustat;
#[doc = "WUSTAT (rw) register accessor: an alias for `Reg<WUSTAT_SPEC>`"]
pub type WUSTAT = crate::Reg<wustat::WUSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod wustat;
#[doc = "REG1_0 (rw) register accessor: an alias for `Reg<REG1_0_SPEC>`"]
pub type REG1_0 = crate::Reg<reg1_0::REG1_0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg1_0;
#[doc = "REG3_2 (rw) register accessor: an alias for `Reg<REG3_2_SPEC>`"]
pub type REG3_2 = crate::Reg<reg3_2::REG3_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg3_2;
#[doc = "REG5_4 (rw) register accessor: an alias for `Reg<REG5_4_SPEC>`"]
pub type REG5_4 = crate::Reg<reg5_4::REG5_4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg5_4;
#[doc = "REG7_6 (rw) register accessor: an alias for `Reg<REG7_6_SPEC>`"]
pub type REG7_6 = crate::Reg<reg7_6::REG7_6_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg7_6;
#[doc = "LOOPADDR (rw) register accessor: an alias for `Reg<LOOPADDR_SPEC>`"]
pub type LOOPADDR = crate::Reg<loopaddr::LOOPADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopaddr;
#[doc = "LOOPCNT (rw) register accessor: an alias for `Reg<LOOPCNT_SPEC>`"]
pub type LOOPCNT = crate::Reg<loopcnt::LOOPCNT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopcnt;
