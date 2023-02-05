#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub meascfg: MEASCFG,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub tempp0: TEMPP0,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub tempp1: TEMPP1,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub tempp2: TEMPP2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub batmonp0: BATMONP0,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub batmonp1: BATMONP1,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub iostrp0: IOSTRP0,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub flashpumpp0: FLASHPUMPP0,
    #[doc = "0x28 - Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
    pub bat: BAT,
    #[doc = "0x2c - Battery Update Indicates BAT Updates"]
    pub batupd: BATUPD,
    #[doc = "0x30 - Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
    pub temp: TEMP,
    #[doc = "0x34 - Temperature Update Indicates TEMP Updates"]
    pub tempupd: TEMPUPD,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "MEASCFG (rw) register accessor: an alias for `Reg<MEASCFG_SPEC>`"]
pub type MEASCFG = crate::Reg<meascfg::MEASCFG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod meascfg;
#[doc = "TEMPP0 (rw) register accessor: an alias for `Reg<TEMPP0_SPEC>`"]
pub type TEMPP0 = crate::Reg<tempp0::TEMPP0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp0;
#[doc = "TEMPP1 (rw) register accessor: an alias for `Reg<TEMPP1_SPEC>`"]
pub type TEMPP1 = crate::Reg<tempp1::TEMPP1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp1;
#[doc = "TEMPP2 (rw) register accessor: an alias for `Reg<TEMPP2_SPEC>`"]
pub type TEMPP2 = crate::Reg<tempp2::TEMPP2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp2;
#[doc = "BATMONP0 (rw) register accessor: an alias for `Reg<BATMONP0_SPEC>`"]
pub type BATMONP0 = crate::Reg<batmonp0::BATMONP0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp0;
#[doc = "BATMONP1 (rw) register accessor: an alias for `Reg<BATMONP1_SPEC>`"]
pub type BATMONP1 = crate::Reg<batmonp1::BATMONP1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp1;
#[doc = "IOSTRP0 (rw) register accessor: an alias for `Reg<IOSTRP0_SPEC>`"]
pub type IOSTRP0 = crate::Reg<iostrp0::IOSTRP0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrp0;
#[doc = "FLASHPUMPP0 (rw) register accessor: an alias for `Reg<FLASHPUMPP0_SPEC>`"]
pub type FLASHPUMPP0 = crate::Reg<flashpumpp0::FLASHPUMPP0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flashpumpp0;
#[doc = "BAT (rw) register accessor: an alias for `Reg<BAT_SPEC>`"]
pub type BAT = crate::Reg<bat::BAT_SPEC>;
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
pub mod bat;
#[doc = "BATUPD (rw) register accessor: an alias for `Reg<BATUPD_SPEC>`"]
pub type BATUPD = crate::Reg<batupd::BATUPD_SPEC>;
#[doc = "Battery Update Indicates BAT Updates"]
pub mod batupd;
#[doc = "TEMP (rw) register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
pub mod temp;
#[doc = "TEMPUPD (rw) register accessor: an alias for `Reg<TEMPUPD_SPEC>`"]
pub type TEMPUPD = crate::Reg<tempupd::TEMPUPD_SPEC>;
#[doc = "Temperature Update Indicates TEMP Updates"]
pub mod tempupd;
