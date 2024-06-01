#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spimcfg: Spimcfg,
    misocfg: Misocfg,
    mosictl: Mosictl,
    tx8: Tx8,
    tx16: Tx16,
    rx8: Rx8,
    rx16: Rx16,
    sclkidle: Sclkidle,
    dataidle: Dataidle,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Master Configuration Write operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn spimcfg(&self) -> &Spimcfg {
        &self.spimcfg
    }
    #[doc = "0x04 - MISO Configuration Write operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn misocfg(&self) -> &Misocfg {
        &self.misocfg
    }
    #[doc = "0x08 - MOSI Control Write operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn mosictl(&self) -> &Mosictl {
        &self.mosictl
    }
    #[doc = "0x0c - Transmit 8 Bit Write operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn tx8(&self) -> &Tx8 {
        &self.tx8
    }
    #[doc = "0x10 - Transmit 16 Bit Write operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn tx16(&self) -> &Tx16 {
        &self.tx16
    }
    #[doc = "0x14 - Receive 8 Bit Read operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn rx8(&self) -> &Rx8 {
        &self.rx8
    }
    #[doc = "0x18 - Receive 16 Bit Read operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn rx16(&self) -> &Rx16 {
        &self.rx16
    }
    #[doc = "0x1c - SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
    #[inline(always)]
    pub const fn sclkidle(&self) -> &Sclkidle {
        &self.sclkidle
    }
    #[doc = "0x20 - Data Idle Read operation stalls until current transfer completes."]
    #[inline(always)]
    pub const fn dataidle(&self) -> &Dataidle {
        &self.dataidle
    }
}
#[doc = "SPIMCFG (rw) register accessor: SPI Master Configuration Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spimcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spimcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spimcfg`]
module"]
#[doc(alias = "SPIMCFG")]
pub type Spimcfg = crate::Reg<spimcfg::SpimcfgSpec>;
#[doc = "SPI Master Configuration Write operation stalls until current transfer completes."]
pub mod spimcfg;
#[doc = "MISOCFG (rw) register accessor: MISO Configuration Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misocfg`]
module"]
#[doc(alias = "MISOCFG")]
pub type Misocfg = crate::Reg<misocfg::MisocfgSpec>;
#[doc = "MISO Configuration Write operation stalls until current transfer completes."]
pub mod misocfg;
#[doc = "MOSICTL (rw) register accessor: MOSI Control Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mosictl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosictl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosictl`]
module"]
#[doc(alias = "MOSICTL")]
pub type Mosictl = crate::Reg<mosictl::MosictlSpec>;
#[doc = "MOSI Control Write operation stalls until current transfer completes."]
pub mod mosictl;
#[doc = "TX8 (rw) register accessor: Transmit 8 Bit Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx8`]
module"]
#[doc(alias = "TX8")]
pub type Tx8 = crate::Reg<tx8::Tx8Spec>;
#[doc = "Transmit 8 Bit Write operation stalls until current transfer completes."]
pub mod tx8;
#[doc = "TX16 (rw) register accessor: Transmit 16 Bit Write operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx16`]
module"]
#[doc(alias = "TX16")]
pub type Tx16 = crate::Reg<tx16::Tx16Spec>;
#[doc = "Transmit 16 Bit Write operation stalls until current transfer completes."]
pub mod tx16;
#[doc = "RX8 (rw) register accessor: Receive 8 Bit Read operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx8`]
module"]
#[doc(alias = "RX8")]
pub type Rx8 = crate::Reg<rx8::Rx8Spec>;
#[doc = "Receive 8 Bit Read operation stalls until current transfer completes."]
pub mod rx8;
#[doc = "RX16 (rw) register accessor: Receive 16 Bit Read operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx16`]
module"]
#[doc(alias = "RX16")]
pub type Rx16 = crate::Reg<rx16::Rx16Spec>;
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes."]
pub mod rx16;
#[doc = "SCLKIDLE (rw) register accessor: SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sclkidle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sclkidle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sclkidle`]
module"]
#[doc(alias = "SCLKIDLE")]
pub type Sclkidle = crate::Reg<sclkidle::SclkidleSpec>;
#[doc = "SCLK Idle Read operation stalls until SCLK is idle with no remaining clock edges."]
pub mod sclkidle;
#[doc = "DATAIDLE (rw) register accessor: Data Idle Read operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataidle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataidle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataidle`]
module"]
#[doc(alias = "DATAIDLE")]
pub type Dataidle = crate::Reg<dataidle::DataidleSpec>;
#[doc = "Data Idle Read operation stalls until current transfer completes."]
pub mod dataidle;
