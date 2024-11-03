#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    _reserved2: [u8; 0x3c],
    imask: Imask,
    _reserved3: [u8; 0x04],
    ris: Ris,
    _reserved4: [u8; 0x04],
    mis: Mis,
    _reserved5: [u8; 0x04],
    iset: Iset,
    _reserved6: [u8; 0x04],
    iclr: Iclr,
    _reserved7: [u8; 0x04],
    imset: Imset,
    _reserved8: [u8; 0x04],
    imclr: Imclr,
    _reserved9: [u8; 0x04],
    dtb: Dtb,
    _reserved10: [u8; 0x80],
    dout3_0: Dout3_0,
    dout7_4: Dout7_4,
    dout11_8: Dout11_8,
    dout15_12: Dout15_12,
    dout19_16: Dout19_16,
    dout23_20: Dout23_20,
    dout27_24: Dout27_24,
    _reserved17: [u8; 0xe4],
    dout31_0: Dout31_0,
    _reserved18: [u8; 0x0c],
    doutset31_0: Doutset31_0,
    _reserved19: [u8; 0x0c],
    doutclr31_0: Doutclr31_0,
    _reserved20: [u8; 0x0c],
    douttgl31_0: Douttgl31_0,
    _reserved21: [u8; 0xcc],
    douttgl3_0: Douttgl3_0,
    douttgl7_4: Douttgl7_4,
    douttgl11_8: Douttgl11_8,
    douttgl15_12: Douttgl15_12,
    douttgl19_16: Douttgl19_16,
    douttgl23_20: Douttgl23_20,
    douttgl27_24: Douttgl27_24,
    _reserved28: [u8; 0xe4],
    doe3_0: Doe3_0,
    doe7_4: Doe7_4,
    doe11_8: Doe11_8,
    doe15_12: Doe15_12,
    doe19_16: Doe19_16,
    doe23_20: Doe23_20,
    doe27_24: Doe27_24,
    _reserved35: [u8; 0xe4],
    doe31_0: Doe31_0,
    _reserved36: [u8; 0x0c],
    doeset31_0: Doeset31_0,
    _reserved37: [u8; 0x0c],
    doeclr31_0: Doeclr31_0,
    _reserved38: [u8; 0x0c],
    doetgl31_0: Doetgl31_0,
    _reserved39: [u8; 0xcc],
    din3_0: Din3_0,
    din7_4: Din7_4,
    din11_8: Din11_8,
    din15_12: Din15_12,
    din19_16: Din19_16,
    din23_20: Din23_20,
    din27_24: Din27_24,
    _reserved46: [u8; 0xe4],
    din31_0: Din31_0,
    _reserved47: [u8; 0xfc],
    evtcfg: Evtcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Provides module ID, revision information, instance index"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Provide IP-specific instance information"]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x44 - Interrupt mask for DIO pins"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x4c - Raw interrupt flag for DIO pins"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x54 - Masked interrupt flag for DIO pins"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x5c - Set interrupt flag in RIS by writing a one"]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x64 - Clear interrupt flag in RIS by writing a one"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x6c - Set interrupt mask in IMASK by writing a one"]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x74 - Clear interrupt mask in IMASK by writing a one"]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
    #[doc = "0x7c - Digital test bus mux selection"]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
    #[doc = "0x100 - Data out 3 to 0. Alias register for byte access to DOUT31_0\\[3:0\\]
bits."]
    #[inline(always)]
    pub const fn dout3_0(&self) -> &Dout3_0 {
        &self.dout3_0
    }
    #[doc = "0x104 - Data out 7 to 4. Alias register for byte access to DOUT31_0\\[7:4\\]
bits."]
    #[inline(always)]
    pub const fn dout7_4(&self) -> &Dout7_4 {
        &self.dout7_4
    }
    #[doc = "0x108 - Data out 11 to 8. Alias register for byte access to DOUT31_0\\[11:8\\]
bits."]
    #[inline(always)]
    pub const fn dout11_8(&self) -> &Dout11_8 {
        &self.dout11_8
    }
    #[doc = "0x10c - Data out 15 to 12. Alias register for byte access to DOUT31_0\\[15:12\\]
bits."]
    #[inline(always)]
    pub const fn dout15_12(&self) -> &Dout15_12 {
        &self.dout15_12
    }
    #[doc = "0x110 - Data out 19 to 16. Alias register for byte access to DOUT31_0\\[19:16\\]
bits."]
    #[inline(always)]
    pub const fn dout19_16(&self) -> &Dout19_16 {
        &self.dout19_16
    }
    #[doc = "0x114 - Data out 23 to 20. Alias register for byte access to DOUT31_0\\[23:20\\]
bits."]
    #[inline(always)]
    pub const fn dout23_20(&self) -> &Dout23_20 {
        &self.dout23_20
    }
    #[doc = "0x118 - Data out 27 to 24. Alias register for byte access to DOUT31_0\\[27:24\\]
bits."]
    #[inline(always)]
    pub const fn dout27_24(&self) -> &Dout27_24 {
        &self.dout27_24
    }
    #[doc = "0x200 - Data Output for DIO 31 to 0 pins."]
    #[inline(always)]
    pub const fn dout31_0(&self) -> &Dout31_0 {
        &self.dout31_0
    }
    #[doc = "0x210 - Data output set for DIO 31 to 0 pins. Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register."]
    #[inline(always)]
    pub const fn doutset31_0(&self) -> &Doutset31_0 {
        &self.doutset31_0
    }
    #[doc = "0x220 - Data output clear for DIO 31 to 0 pins. Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register."]
    #[inline(always)]
    pub const fn doutclr31_0(&self) -> &Doutclr31_0 {
        &self.doutclr31_0
    }
    #[doc = "0x230 - Data output toggle for DIO 31 to 0 pins. Writing 1 to a bit position will invert the corresponding bit in DOUT31_0 register."]
    #[inline(always)]
    pub const fn douttgl31_0(&self) -> &Douttgl31_0 {
        &self.douttgl31_0
    }
    #[doc = "0x300 - Data out toggle 3 to 0. Alias register for byte access to DOUTTGL31_0\\[3:0\\]
bits."]
    #[inline(always)]
    pub const fn douttgl3_0(&self) -> &Douttgl3_0 {
        &self.douttgl3_0
    }
    #[doc = "0x304 - Data out toggle 7 to 4. Alias register for byte access to DOUTTGL31_0\\[7:4\\]
bits."]
    #[inline(always)]
    pub const fn douttgl7_4(&self) -> &Douttgl7_4 {
        &self.douttgl7_4
    }
    #[doc = "0x308 - Data out toggle 11 to 8. Alias register for byte access to DOUTTGL31_0\\[11:8\\]
bits."]
    #[inline(always)]
    pub const fn douttgl11_8(&self) -> &Douttgl11_8 {
        &self.douttgl11_8
    }
    #[doc = "0x30c - Data out toggle 15 to 12. Alias register for byte access to DOUTTGL31_0\\[15:12\\]
bits."]
    #[inline(always)]
    pub const fn douttgl15_12(&self) -> &Douttgl15_12 {
        &self.douttgl15_12
    }
    #[doc = "0x310 - Data out toggle 19 to 16. Alias register for byte access to DOUTTGL31_0\\[19:16\\]
bits."]
    #[inline(always)]
    pub const fn douttgl19_16(&self) -> &Douttgl19_16 {
        &self.douttgl19_16
    }
    #[doc = "0x314 - Data out toggle 23 to 20. Alias register for byte access to DOUTTGL31_0\\[23:20\\]
bits."]
    #[inline(always)]
    pub const fn douttgl23_20(&self) -> &Douttgl23_20 {
        &self.douttgl23_20
    }
    #[doc = "0x318 - Data out toggle 27 to 24. Alias register for byte access to DOUTTGL31_0\\[27:24\\]
bits."]
    #[inline(always)]
    pub const fn douttgl27_24(&self) -> &Douttgl27_24 {
        &self.douttgl27_24
    }
    #[doc = "0x400 - Data out enable 3 to 0. Alias register for byte access to DOE31_0\\[3:0\\]
bits."]
    #[inline(always)]
    pub const fn doe3_0(&self) -> &Doe3_0 {
        &self.doe3_0
    }
    #[doc = "0x404 - Data out enable 7 to 4. Alias register for byte access to DOE31_0\\[7:4\\]
bits."]
    #[inline(always)]
    pub const fn doe7_4(&self) -> &Doe7_4 {
        &self.doe7_4
    }
    #[doc = "0x408 - Data out enable 11 to 8. Alias register for byte access to DOE31_0\\[11:8\\]
bits."]
    #[inline(always)]
    pub const fn doe11_8(&self) -> &Doe11_8 {
        &self.doe11_8
    }
    #[doc = "0x40c - Data out enable 15 to 12. Alias register for byte access to DOE31_0\\[15:12\\]
bits."]
    #[inline(always)]
    pub const fn doe15_12(&self) -> &Doe15_12 {
        &self.doe15_12
    }
    #[doc = "0x410 - Data out enable 19 to 16. Alias register for byte access to DOE31_0\\[19:16\\]
bits."]
    #[inline(always)]
    pub const fn doe19_16(&self) -> &Doe19_16 {
        &self.doe19_16
    }
    #[doc = "0x414 - Data out enable 23 to 20. Alias register for byte access to DOE31_0\\[23:20\\]
bits."]
    #[inline(always)]
    pub const fn doe23_20(&self) -> &Doe23_20 {
        &self.doe23_20
    }
    #[doc = "0x418 - Data out enable 27 to 24. Alias register for byte access to DOE31_0\\[27:24\\]
bits."]
    #[inline(always)]
    pub const fn doe27_24(&self) -> &Doe27_24 {
        &self.doe27_24
    }
    #[doc = "0x500 - Data output enable for DIO 31 to 0 pins."]
    #[inline(always)]
    pub const fn doe31_0(&self) -> &Doe31_0 {
        &self.doe31_0
    }
    #[doc = "0x510 - Data output enable set for DIO 31 to 0 pins. Writing 1 to a bit position sets the corresponding bit in the DOE31_0 register."]
    #[inline(always)]
    pub const fn doeset31_0(&self) -> &Doeset31_0 {
        &self.doeset31_0
    }
    #[doc = "0x520 - Data output enable clear for DIO 31 to 0 pins. Writing 1 to a bit position clears the corresponding bit in the DOE31_0 register."]
    #[inline(always)]
    pub const fn doeclr31_0(&self) -> &Doeclr31_0 {
        &self.doeclr31_0
    }
    #[doc = "0x530 - Data output enable toggle for DIO 31 to 0 pins. Writing 1 to a bit position will invert the corresponding bit in DOE31_0 register."]
    #[inline(always)]
    pub const fn doetgl31_0(&self) -> &Doetgl31_0 {
        &self.doetgl31_0
    }
    #[doc = "0x600 - Data input 3 to 0. Alias register for byte access to DIN31_0\\[3:0\\]
bits."]
    #[inline(always)]
    pub const fn din3_0(&self) -> &Din3_0 {
        &self.din3_0
    }
    #[doc = "0x604 - Data input 7 to 4. Alias register for byte access to DIN31_0\\[7:4\\]
bits."]
    #[inline(always)]
    pub const fn din7_4(&self) -> &Din7_4 {
        &self.din7_4
    }
    #[doc = "0x608 - Data input 11 to 8. Alias register for byte access to DIN31_0\\[11:8\\]
bits."]
    #[inline(always)]
    pub const fn din11_8(&self) -> &Din11_8 {
        &self.din11_8
    }
    #[doc = "0x60c - Data input 15 to 12. Alias register for byte access to DIN31_0\\[15:12\\]
bits."]
    #[inline(always)]
    pub const fn din15_12(&self) -> &Din15_12 {
        &self.din15_12
    }
    #[doc = "0x610 - Data input 19 to 16. Alias register for byte access to DIN31_0\\[19:16\\]
bits."]
    #[inline(always)]
    pub const fn din19_16(&self) -> &Din19_16 {
        &self.din19_16
    }
    #[doc = "0x614 - Data input 23 to 20. Alias register for byte access to DIN31_0\\[23:20\\]
bits."]
    #[inline(always)]
    pub const fn din23_20(&self) -> &Din23_20 {
        &self.din23_20
    }
    #[doc = "0x618 - Data input 27 to 24. Alias register for byte access to DIN31_0\\[27:24\\]
bits."]
    #[inline(always)]
    pub const fn din27_24(&self) -> &Din27_24 {
        &self.din27_24
    }
    #[doc = "0x700 - Data input from DIO 31 to 0 pins."]
    #[inline(always)]
    pub const fn din31_0(&self) -> &Din31_0 {
        &self.din31_0
    }
    #[doc = "0x800 - Event configuration. This register is used to select DIO for GPIO to publish event on SVT event fabric. It also contains enable bit that is used to mask the event."]
    #[inline(always)]
    pub const fn evtcfg(&self) -> &Evtcfg {
        &self.evtcfg
    }
}
#[doc = "DESC (rw) register accessor: Provides module ID, revision information, instance index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Provides module ID, revision information, instance index"]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: Provide IP-specific instance information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "Provide IP-specific instance information"]
pub mod descex;
#[doc = "IMASK (rw) register accessor: Interrupt mask for DIO pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask for DIO pins"]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw interrupt flag for DIO pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt flag for DIO pins"]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked interrupt flag for DIO pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt flag for DIO pins"]
pub mod mis;
#[doc = "ISET (rw) register accessor: Set interrupt flag in RIS by writing a one\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Set interrupt flag in RIS by writing a one"]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Clear interrupt flag in RIS by writing a one\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Clear interrupt flag in RIS by writing a one"]
pub mod iclr;
#[doc = "IMSET (rw) register accessor: Set interrupt mask in IMASK by writing a one\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imset`]
module"]
#[doc(alias = "IMSET")]
pub type Imset = crate::Reg<imset::ImsetSpec>;
#[doc = "Set interrupt mask in IMASK by writing a one"]
pub mod imset;
#[doc = "IMCLR (rw) register accessor: Clear interrupt mask in IMASK by writing a one\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imclr`]
module"]
#[doc(alias = "IMCLR")]
pub type Imclr = crate::Reg<imclr::ImclrSpec>;
#[doc = "Clear interrupt mask in IMASK by writing a one"]
pub mod imclr;
#[doc = "DTB (rw) register accessor: Digital test bus mux selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital test bus mux selection"]
pub mod dtb;
#[doc = "DOUT3_0 (rw) register accessor: Data out 3 to 0. Alias register for byte access to DOUT31_0\\[3:0\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout3_0`]
module"]
#[doc(alias = "DOUT3_0")]
pub type Dout3_0 = crate::Reg<dout3_0::Dout3_0Spec>;
#[doc = "Data out 3 to 0. Alias register for byte access to DOUT31_0\\[3:0\\]
bits."]
pub mod dout3_0;
#[doc = "DOUT7_4 (rw) register accessor: Data out 7 to 4. Alias register for byte access to DOUT31_0\\[7:4\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout7_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout7_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout7_4`]
module"]
#[doc(alias = "DOUT7_4")]
pub type Dout7_4 = crate::Reg<dout7_4::Dout7_4Spec>;
#[doc = "Data out 7 to 4. Alias register for byte access to DOUT31_0\\[7:4\\]
bits."]
pub mod dout7_4;
#[doc = "DOUT11_8 (rw) register accessor: Data out 11 to 8. Alias register for byte access to DOUT31_0\\[11:8\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout11_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout11_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout11_8`]
module"]
#[doc(alias = "DOUT11_8")]
pub type Dout11_8 = crate::Reg<dout11_8::Dout11_8Spec>;
#[doc = "Data out 11 to 8. Alias register for byte access to DOUT31_0\\[11:8\\]
bits."]
pub mod dout11_8;
#[doc = "DOUT15_12 (rw) register accessor: Data out 15 to 12. Alias register for byte access to DOUT31_0\\[15:12\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout15_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout15_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout15_12`]
module"]
#[doc(alias = "DOUT15_12")]
pub type Dout15_12 = crate::Reg<dout15_12::Dout15_12Spec>;
#[doc = "Data out 15 to 12. Alias register for byte access to DOUT31_0\\[15:12\\]
bits."]
pub mod dout15_12;
#[doc = "DOUT19_16 (rw) register accessor: Data out 19 to 16. Alias register for byte access to DOUT31_0\\[19:16\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout19_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout19_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout19_16`]
module"]
#[doc(alias = "DOUT19_16")]
pub type Dout19_16 = crate::Reg<dout19_16::Dout19_16Spec>;
#[doc = "Data out 19 to 16. Alias register for byte access to DOUT31_0\\[19:16\\]
bits."]
pub mod dout19_16;
#[doc = "DOUT23_20 (rw) register accessor: Data out 23 to 20. Alias register for byte access to DOUT31_0\\[23:20\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout23_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout23_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout23_20`]
module"]
#[doc(alias = "DOUT23_20")]
pub type Dout23_20 = crate::Reg<dout23_20::Dout23_20Spec>;
#[doc = "Data out 23 to 20. Alias register for byte access to DOUT31_0\\[23:20\\]
bits."]
pub mod dout23_20;
#[doc = "DOUT27_24 (rw) register accessor: Data out 27 to 24. Alias register for byte access to DOUT31_0\\[27:24\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout27_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout27_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout27_24`]
module"]
#[doc(alias = "DOUT27_24")]
pub type Dout27_24 = crate::Reg<dout27_24::Dout27_24Spec>;
#[doc = "Data out 27 to 24. Alias register for byte access to DOUT31_0\\[27:24\\]
bits."]
pub mod dout27_24;
#[doc = "DOUT31_0 (rw) register accessor: Data Output for DIO 31 to 0 pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout31_0`]
module"]
#[doc(alias = "DOUT31_0")]
pub type Dout31_0 = crate::Reg<dout31_0::Dout31_0Spec>;
#[doc = "Data Output for DIO 31 to 0 pins."]
pub mod dout31_0;
#[doc = "DOUTSET31_0 (rw) register accessor: Data output set for DIO 31 to 0 pins. Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutset31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutset31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutset31_0`]
module"]
#[doc(alias = "DOUTSET31_0")]
pub type Doutset31_0 = crate::Reg<doutset31_0::Doutset31_0Spec>;
#[doc = "Data output set for DIO 31 to 0 pins. Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register."]
pub mod doutset31_0;
#[doc = "DOUTCLR31_0 (rw) register accessor: Data output clear for DIO 31 to 0 pins. Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutclr31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutclr31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutclr31_0`]
module"]
#[doc(alias = "DOUTCLR31_0")]
pub type Doutclr31_0 = crate::Reg<doutclr31_0::Doutclr31_0Spec>;
#[doc = "Data output clear for DIO 31 to 0 pins. Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register."]
pub mod doutclr31_0;
#[doc = "DOUTTGL31_0 (rw) register accessor: Data output toggle for DIO 31 to 0 pins. Writing 1 to a bit position will invert the corresponding bit in DOUT31_0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl31_0`]
module"]
#[doc(alias = "DOUTTGL31_0")]
pub type Douttgl31_0 = crate::Reg<douttgl31_0::Douttgl31_0Spec>;
#[doc = "Data output toggle for DIO 31 to 0 pins. Writing 1 to a bit position will invert the corresponding bit in DOUT31_0 register."]
pub mod douttgl31_0;
#[doc = "DOUTTGL3_0 (rw) register accessor: Data out toggle 3 to 0. Alias register for byte access to DOUTTGL31_0\\[3:0\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl3_0`]
module"]
#[doc(alias = "DOUTTGL3_0")]
pub type Douttgl3_0 = crate::Reg<douttgl3_0::Douttgl3_0Spec>;
#[doc = "Data out toggle 3 to 0. Alias register for byte access to DOUTTGL31_0\\[3:0\\]
bits."]
pub mod douttgl3_0;
#[doc = "DOUTTGL7_4 (rw) register accessor: Data out toggle 7 to 4. Alias register for byte access to DOUTTGL31_0\\[7:4\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl7_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl7_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl7_4`]
module"]
#[doc(alias = "DOUTTGL7_4")]
pub type Douttgl7_4 = crate::Reg<douttgl7_4::Douttgl7_4Spec>;
#[doc = "Data out toggle 7 to 4. Alias register for byte access to DOUTTGL31_0\\[7:4\\]
bits."]
pub mod douttgl7_4;
#[doc = "DOUTTGL11_8 (rw) register accessor: Data out toggle 11 to 8. Alias register for byte access to DOUTTGL31_0\\[11:8\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl11_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl11_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl11_8`]
module"]
#[doc(alias = "DOUTTGL11_8")]
pub type Douttgl11_8 = crate::Reg<douttgl11_8::Douttgl11_8Spec>;
#[doc = "Data out toggle 11 to 8. Alias register for byte access to DOUTTGL31_0\\[11:8\\]
bits."]
pub mod douttgl11_8;
#[doc = "DOUTTGL15_12 (rw) register accessor: Data out toggle 15 to 12. Alias register for byte access to DOUTTGL31_0\\[15:12\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl15_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl15_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl15_12`]
module"]
#[doc(alias = "DOUTTGL15_12")]
pub type Douttgl15_12 = crate::Reg<douttgl15_12::Douttgl15_12Spec>;
#[doc = "Data out toggle 15 to 12. Alias register for byte access to DOUTTGL31_0\\[15:12\\]
bits."]
pub mod douttgl15_12;
#[doc = "DOUTTGL19_16 (rw) register accessor: Data out toggle 19 to 16. Alias register for byte access to DOUTTGL31_0\\[19:16\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl19_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl19_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl19_16`]
module"]
#[doc(alias = "DOUTTGL19_16")]
pub type Douttgl19_16 = crate::Reg<douttgl19_16::Douttgl19_16Spec>;
#[doc = "Data out toggle 19 to 16. Alias register for byte access to DOUTTGL31_0\\[19:16\\]
bits."]
pub mod douttgl19_16;
#[doc = "DOUTTGL23_20 (rw) register accessor: Data out toggle 23 to 20. Alias register for byte access to DOUTTGL31_0\\[23:20\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl23_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl23_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl23_20`]
module"]
#[doc(alias = "DOUTTGL23_20")]
pub type Douttgl23_20 = crate::Reg<douttgl23_20::Douttgl23_20Spec>;
#[doc = "Data out toggle 23 to 20. Alias register for byte access to DOUTTGL31_0\\[23:20\\]
bits."]
pub mod douttgl23_20;
#[doc = "DOUTTGL27_24 (rw) register accessor: Data out toggle 27 to 24. Alias register for byte access to DOUTTGL31_0\\[27:24\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl27_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl27_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl27_24`]
module"]
#[doc(alias = "DOUTTGL27_24")]
pub type Douttgl27_24 = crate::Reg<douttgl27_24::Douttgl27_24Spec>;
#[doc = "Data out toggle 27 to 24. Alias register for byte access to DOUTTGL31_0\\[27:24\\]
bits."]
pub mod douttgl27_24;
#[doc = "DOE3_0 (rw) register accessor: Data out enable 3 to 0. Alias register for byte access to DOE31_0\\[3:0\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe3_0`]
module"]
#[doc(alias = "DOE3_0")]
pub type Doe3_0 = crate::Reg<doe3_0::Doe3_0Spec>;
#[doc = "Data out enable 3 to 0. Alias register for byte access to DOE31_0\\[3:0\\]
bits."]
pub mod doe3_0;
#[doc = "DOE7_4 (rw) register accessor: Data out enable 7 to 4. Alias register for byte access to DOE31_0\\[7:4\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe7_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe7_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe7_4`]
module"]
#[doc(alias = "DOE7_4")]
pub type Doe7_4 = crate::Reg<doe7_4::Doe7_4Spec>;
#[doc = "Data out enable 7 to 4. Alias register for byte access to DOE31_0\\[7:4\\]
bits."]
pub mod doe7_4;
#[doc = "DOE11_8 (rw) register accessor: Data out enable 11 to 8. Alias register for byte access to DOE31_0\\[11:8\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe11_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe11_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe11_8`]
module"]
#[doc(alias = "DOE11_8")]
pub type Doe11_8 = crate::Reg<doe11_8::Doe11_8Spec>;
#[doc = "Data out enable 11 to 8. Alias register for byte access to DOE31_0\\[11:8\\]
bits."]
pub mod doe11_8;
#[doc = "DOE15_12 (rw) register accessor: Data out enable 15 to 12. Alias register for byte access to DOE31_0\\[15:12\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe15_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe15_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe15_12`]
module"]
#[doc(alias = "DOE15_12")]
pub type Doe15_12 = crate::Reg<doe15_12::Doe15_12Spec>;
#[doc = "Data out enable 15 to 12. Alias register for byte access to DOE31_0\\[15:12\\]
bits."]
pub mod doe15_12;
#[doc = "DOE19_16 (rw) register accessor: Data out enable 19 to 16. Alias register for byte access to DOE31_0\\[19:16\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe19_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe19_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe19_16`]
module"]
#[doc(alias = "DOE19_16")]
pub type Doe19_16 = crate::Reg<doe19_16::Doe19_16Spec>;
#[doc = "Data out enable 19 to 16. Alias register for byte access to DOE31_0\\[19:16\\]
bits."]
pub mod doe19_16;
#[doc = "DOE23_20 (rw) register accessor: Data out enable 23 to 20. Alias register for byte access to DOE31_0\\[23:20\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe23_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe23_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe23_20`]
module"]
#[doc(alias = "DOE23_20")]
pub type Doe23_20 = crate::Reg<doe23_20::Doe23_20Spec>;
#[doc = "Data out enable 23 to 20. Alias register for byte access to DOE31_0\\[23:20\\]
bits."]
pub mod doe23_20;
#[doc = "DOE27_24 (rw) register accessor: Data out enable 27 to 24. Alias register for byte access to DOE31_0\\[27:24\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe27_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe27_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe27_24`]
module"]
#[doc(alias = "DOE27_24")]
pub type Doe27_24 = crate::Reg<doe27_24::Doe27_24Spec>;
#[doc = "Data out enable 27 to 24. Alias register for byte access to DOE31_0\\[27:24\\]
bits."]
pub mod doe27_24;
#[doc = "DOE31_0 (rw) register accessor: Data output enable for DIO 31 to 0 pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe31_0`]
module"]
#[doc(alias = "DOE31_0")]
pub type Doe31_0 = crate::Reg<doe31_0::Doe31_0Spec>;
#[doc = "Data output enable for DIO 31 to 0 pins."]
pub mod doe31_0;
#[doc = "DOESET31_0 (rw) register accessor: Data output enable set for DIO 31 to 0 pins. Writing 1 to a bit position sets the corresponding bit in the DOE31_0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeset31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeset31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeset31_0`]
module"]
#[doc(alias = "DOESET31_0")]
pub type Doeset31_0 = crate::Reg<doeset31_0::Doeset31_0Spec>;
#[doc = "Data output enable set for DIO 31 to 0 pins. Writing 1 to a bit position sets the corresponding bit in the DOE31_0 register."]
pub mod doeset31_0;
#[doc = "DOECLR31_0 (rw) register accessor: Data output enable clear for DIO 31 to 0 pins. Writing 1 to a bit position clears the corresponding bit in the DOE31_0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeclr31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeclr31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeclr31_0`]
module"]
#[doc(alias = "DOECLR31_0")]
pub type Doeclr31_0 = crate::Reg<doeclr31_0::Doeclr31_0Spec>;
#[doc = "Data output enable clear for DIO 31 to 0 pins. Writing 1 to a bit position clears the corresponding bit in the DOE31_0 register."]
pub mod doeclr31_0;
#[doc = "DOETGL31_0 (rw) register accessor: Data output enable toggle for DIO 31 to 0 pins. Writing 1 to a bit position will invert the corresponding bit in DOE31_0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doetgl31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doetgl31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doetgl31_0`]
module"]
#[doc(alias = "DOETGL31_0")]
pub type Doetgl31_0 = crate::Reg<doetgl31_0::Doetgl31_0Spec>;
#[doc = "Data output enable toggle for DIO 31 to 0 pins. Writing 1 to a bit position will invert the corresponding bit in DOE31_0 register."]
pub mod doetgl31_0;
#[doc = "DIN3_0 (rw) register accessor: Data input 3 to 0. Alias register for byte access to DIN31_0\\[3:0\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din3_0`]
module"]
#[doc(alias = "DIN3_0")]
pub type Din3_0 = crate::Reg<din3_0::Din3_0Spec>;
#[doc = "Data input 3 to 0. Alias register for byte access to DIN31_0\\[3:0\\]
bits."]
pub mod din3_0;
#[doc = "DIN7_4 (rw) register accessor: Data input 7 to 4. Alias register for byte access to DIN31_0\\[7:4\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din7_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din7_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din7_4`]
module"]
#[doc(alias = "DIN7_4")]
pub type Din7_4 = crate::Reg<din7_4::Din7_4Spec>;
#[doc = "Data input 7 to 4. Alias register for byte access to DIN31_0\\[7:4\\]
bits."]
pub mod din7_4;
#[doc = "DIN11_8 (rw) register accessor: Data input 11 to 8. Alias register for byte access to DIN31_0\\[11:8\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din11_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din11_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din11_8`]
module"]
#[doc(alias = "DIN11_8")]
pub type Din11_8 = crate::Reg<din11_8::Din11_8Spec>;
#[doc = "Data input 11 to 8. Alias register for byte access to DIN31_0\\[11:8\\]
bits."]
pub mod din11_8;
#[doc = "DIN15_12 (rw) register accessor: Data input 15 to 12. Alias register for byte access to DIN31_0\\[15:12\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din15_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din15_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din15_12`]
module"]
#[doc(alias = "DIN15_12")]
pub type Din15_12 = crate::Reg<din15_12::Din15_12Spec>;
#[doc = "Data input 15 to 12. Alias register for byte access to DIN31_0\\[15:12\\]
bits."]
pub mod din15_12;
#[doc = "DIN19_16 (rw) register accessor: Data input 19 to 16. Alias register for byte access to DIN31_0\\[19:16\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din19_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din19_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din19_16`]
module"]
#[doc(alias = "DIN19_16")]
pub type Din19_16 = crate::Reg<din19_16::Din19_16Spec>;
#[doc = "Data input 19 to 16. Alias register for byte access to DIN31_0\\[19:16\\]
bits."]
pub mod din19_16;
#[doc = "DIN23_20 (rw) register accessor: Data input 23 to 20. Alias register for byte access to DIN31_0\\[23:20\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din23_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din23_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din23_20`]
module"]
#[doc(alias = "DIN23_20")]
pub type Din23_20 = crate::Reg<din23_20::Din23_20Spec>;
#[doc = "Data input 23 to 20. Alias register for byte access to DIN31_0\\[23:20\\]
bits."]
pub mod din23_20;
#[doc = "DIN27_24 (rw) register accessor: Data input 27 to 24. Alias register for byte access to DIN31_0\\[27:24\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din27_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din27_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din27_24`]
module"]
#[doc(alias = "DIN27_24")]
pub type Din27_24 = crate::Reg<din27_24::Din27_24Spec>;
#[doc = "Data input 27 to 24. Alias register for byte access to DIN31_0\\[27:24\\]
bits."]
pub mod din27_24;
#[doc = "DIN31_0 (rw) register accessor: Data input from DIO 31 to 0 pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din31_0`]
module"]
#[doc(alias = "DIN31_0")]
pub type Din31_0 = crate::Reg<din31_0::Din31_0Spec>;
#[doc = "Data input from DIO 31 to 0 pins."]
pub mod din31_0;
#[doc = "EVTCFG (rw) register accessor: Event configuration. This register is used to select DIO for GPIO to publish event on SVT event fabric. It also contains enable bit that is used to mask the event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtcfg`]
module"]
#[doc(alias = "EVTCFG")]
pub type Evtcfg = crate::Reg<evtcfg::EvtcfgSpec>;
#[doc = "Event configuration. This register is used to select DIO for GPIO to publish event on SVT event fabric. It also contains enable bit that is used to mask the event."]
pub mod evtcfg;
