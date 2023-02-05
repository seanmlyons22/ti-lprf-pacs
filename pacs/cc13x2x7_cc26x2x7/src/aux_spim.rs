#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Master Configuration Write operation stalls until current transfer completes."]
    pub spimcfg: SPIMCFG,
    #[doc = "0x04 - MISO Configuration Write operation stalls until current transfer completes."]
    pub misocfg: MISOCFG,
    #[doc = "0x08 - MOSI Control Write operation stalls until current transfer completes."]
    pub mosictl: MOSICTL,
    #[doc = "0x0c - Transmit 8 Bit Write operation stalls until current transfer completes."]
    pub tx8: TX8,
    #[doc = "0x10 - Transmit 16 Bit Write operation stalls until current transfer completes."]
    pub tx16: TX16,
    #[doc = "0x14 - Receive 8 Bit Read operation stalls until current transfer completes."]
    pub rx8: RX8,
    #[doc = "0x18 - Receive 16 Bit Read operation stalls until current transfer completes."]
    pub rx16: RX16,
    #[doc = "0x1c - SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
    pub sclkidle: SCLKIDLE,
    #[doc = "0x20 - Data Idle Read operation stalls until current transfer completes."]
    pub dataidle: DATAIDLE,
}
#[doc = "SPIMCFG (rw) register accessor: an alias for `Reg<SPIMCFG_SPEC>`"]
pub type SPIMCFG = crate::Reg<spimcfg::SPIMCFG_SPEC>;
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes."]
pub mod spimcfg;
#[doc = "MISOCFG (rw) register accessor: an alias for `Reg<MISOCFG_SPEC>`"]
pub type MISOCFG = crate::Reg<misocfg::MISOCFG_SPEC>;
#[doc = "MISO Configuration Write operation stalls until current transfer completes."]
pub mod misocfg;
#[doc = "MOSICTL (rw) register accessor: an alias for `Reg<MOSICTL_SPEC>`"]
pub type MOSICTL = crate::Reg<mosictl::MOSICTL_SPEC>;
#[doc = "MOSI Control Write operation stalls until current transfer completes."]
pub mod mosictl;
#[doc = "TX8 (rw) register accessor: an alias for `Reg<TX8_SPEC>`"]
pub type TX8 = crate::Reg<tx8::TX8_SPEC>;
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes."]
pub mod tx8;
#[doc = "TX16 (rw) register accessor: an alias for `Reg<TX16_SPEC>`"]
pub type TX16 = crate::Reg<tx16::TX16_SPEC>;
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes."]
pub mod tx16;
#[doc = "RX8 (rw) register accessor: an alias for `Reg<RX8_SPEC>`"]
pub type RX8 = crate::Reg<rx8::RX8_SPEC>;
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes."]
pub mod rx8;
#[doc = "RX16 (rw) register accessor: an alias for `Reg<RX16_SPEC>`"]
pub type RX16 = crate::Reg<rx16::RX16_SPEC>;
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes."]
pub mod rx16;
#[doc = "SCLKIDLE (rw) register accessor: an alias for `Reg<SCLKIDLE_SPEC>`"]
pub type SCLKIDLE = crate::Reg<sclkidle::SCLKIDLE_SPEC>;
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
pub mod sclkidle;
#[doc = "DATAIDLE (rw) register accessor: an alias for `Reg<DATAIDLE_SPEC>`"]
pub type DATAIDLE = crate::Reg<dataidle::DATAIDLE_SPEC>;
#[doc = "Data Idle Read operation stalls until current transfer completes."]
pub mod dataidle;
