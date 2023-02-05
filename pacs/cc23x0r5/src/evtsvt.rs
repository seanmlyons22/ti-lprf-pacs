#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    _reserved2: [u8; 0x5c],
    dtb: Dtb,
    _reserved3: [u8; 0x0398],
    nmisel: Nmisel,
    cpuirq0sel: Cpuirq0sel,
    cpuirq1sel: Cpuirq1sel,
    cpuirq2sel: Cpuirq2sel,
    cpuirq3sel: Cpuirq3sel,
    cpuirq4sel: Cpuirq4sel,
    cpuirq5sel: Cpuirq5sel,
    cpuirq6sel: Cpuirq6sel,
    cpuirq7sel: Cpuirq7sel,
    cpuirq8sel: Cpuirq8sel,
    cpuirq9sel: Cpuirq9sel,
    cpuirq10sel: Cpuirq10sel,
    cpuirq11sel: Cpuirq11sel,
    cpuirq12sel: Cpuirq12sel,
    cpuirq13sel: Cpuirq13sel,
    cpuirq14sel: Cpuirq14sel,
    cpuirq15sel: Cpuirq15sel,
    cpuirq16sel: Cpuirq16sel,
    cpuirq17sel: Cpuirq17sel,
    cpuirq18sel: Cpuirq18sel,
    systimc0sel: Systimc0sel,
    systimc1sel: Systimc1sel,
    systimc2sel: Systimc2sel,
    systimc3sel: Systimc3sel,
    systimc4sel: Systimc4sel,
    adctrgsel: Adctrgsel,
    lgptsyncsel: Lgptsyncsel,
    lgpt0in0sel: Lgpt0in0sel,
    lgpt0in1sel: Lgpt0in1sel,
    lgpt0in2sel: Lgpt0in2sel,
    lgpt0tensel: Lgpt0tensel,
    lgpt1in0sel: Lgpt1in0sel,
    lgpt1in1sel: Lgpt1in1sel,
    lgpt1in2sel: Lgpt1in2sel,
    lgpt1tensel: Lgpt1tensel,
    lgpt2in0sel: Lgpt2in0sel,
    lgpt2in1sel: Lgpt2in1sel,
    lgpt2in2sel: Lgpt2in2sel,
    lgpt2tensel: Lgpt2tensel,
    lgpt3in0sel: Lgpt3in0sel,
    lgpt3in1sel: Lgpt3in1sel,
    lgpt3in2sel: Lgpt3in2sel,
    lgpt3tensel: Lgpt3tensel,
    lrfdin0sel: Lrfdin0sel,
    lrfdin1sel: Lrfdin1sel,
    lrfdin2sel: Lrfdin2sel,
    _reserved49: [u8; 0x0748],
    dmach0sel: Dmach0sel,
    dmach1sel: Dmach1sel,
    dmach2sel: Dmach2sel,
    dmach3sel: Dmach3sel,
    dmach4sel: Dmach4sel,
    dmach5sel: Dmach5sel,
    dmach6sel: Dmach6sel,
    dmach7sel: Dmach7sel,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Extended Description Register. This register provides configuration details of the IP to software drivers and end users."]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x64 - Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
    #[doc = "0x400 - Output Selection for CPU NMI Interrupt"]
    #[inline(always)]
    pub const fn nmisel(&self) -> &Nmisel {
        &self.nmisel
    }
    #[doc = "0x404 - Output Selection for CPU Interrupt CPUIRQ0"]
    #[inline(always)]
    pub const fn cpuirq0sel(&self) -> &Cpuirq0sel {
        &self.cpuirq0sel
    }
    #[doc = "0x408 - Output Selection for CPU Interrupt CPUIRQ1"]
    #[inline(always)]
    pub const fn cpuirq1sel(&self) -> &Cpuirq1sel {
        &self.cpuirq1sel
    }
    #[doc = "0x40c - Output Selection for CPU Interrupt CPUIRQ2"]
    #[inline(always)]
    pub const fn cpuirq2sel(&self) -> &Cpuirq2sel {
        &self.cpuirq2sel
    }
    #[doc = "0x410 - Output Selection for CPU Interrupt CPUIRQ3"]
    #[inline(always)]
    pub const fn cpuirq3sel(&self) -> &Cpuirq3sel {
        &self.cpuirq3sel
    }
    #[doc = "0x414 - Output Selection for CPU Interrupt CPUIRQ4"]
    #[inline(always)]
    pub const fn cpuirq4sel(&self) -> &Cpuirq4sel {
        &self.cpuirq4sel
    }
    #[doc = "0x418 - Output Selection for CPU Interrupt CPUIRQ5"]
    #[inline(always)]
    pub const fn cpuirq5sel(&self) -> &Cpuirq5sel {
        &self.cpuirq5sel
    }
    #[doc = "0x41c - Output Selection for CPU Interrupt CPUIRQ6"]
    #[inline(always)]
    pub const fn cpuirq6sel(&self) -> &Cpuirq6sel {
        &self.cpuirq6sel
    }
    #[doc = "0x420 - Output Selection for CPU Interrupt CPUIRQ7"]
    #[inline(always)]
    pub const fn cpuirq7sel(&self) -> &Cpuirq7sel {
        &self.cpuirq7sel
    }
    #[doc = "0x424 - Output Selection for CPU Interrupt CPUIRQ8"]
    #[inline(always)]
    pub const fn cpuirq8sel(&self) -> &Cpuirq8sel {
        &self.cpuirq8sel
    }
    #[doc = "0x428 - Output Selection for CPU Interrupt CPUIRQ9"]
    #[inline(always)]
    pub const fn cpuirq9sel(&self) -> &Cpuirq9sel {
        &self.cpuirq9sel
    }
    #[doc = "0x42c - Output Selection for CPU Interrupt CPUIRQ10"]
    #[inline(always)]
    pub const fn cpuirq10sel(&self) -> &Cpuirq10sel {
        &self.cpuirq10sel
    }
    #[doc = "0x430 - Output Selection for CPU Interrupt CPUIRQ11"]
    #[inline(always)]
    pub const fn cpuirq11sel(&self) -> &Cpuirq11sel {
        &self.cpuirq11sel
    }
    #[doc = "0x434 - Output Selection for CPU Interrupt CPUIRQ12"]
    #[inline(always)]
    pub const fn cpuirq12sel(&self) -> &Cpuirq12sel {
        &self.cpuirq12sel
    }
    #[doc = "0x438 - Output Selection for CPU Interrupt CPUIRQ13"]
    #[inline(always)]
    pub const fn cpuirq13sel(&self) -> &Cpuirq13sel {
        &self.cpuirq13sel
    }
    #[doc = "0x43c - Output Selection for CPU Interrupt CPUIRQ14"]
    #[inline(always)]
    pub const fn cpuirq14sel(&self) -> &Cpuirq14sel {
        &self.cpuirq14sel
    }
    #[doc = "0x440 - Output Selection for CPU Interrupt CPUIRQ15"]
    #[inline(always)]
    pub const fn cpuirq15sel(&self) -> &Cpuirq15sel {
        &self.cpuirq15sel
    }
    #[doc = "0x444 - Output Selection for CPU Interrupt CPUIRQ16"]
    #[inline(always)]
    pub const fn cpuirq16sel(&self) -> &Cpuirq16sel {
        &self.cpuirq16sel
    }
    #[doc = "0x448 - Output Selection for CPU Interrupt CPUIRQ17"]
    #[inline(always)]
    pub const fn cpuirq17sel(&self) -> &Cpuirq17sel {
        &self.cpuirq17sel
    }
    #[doc = "0x44c - Output Selection for CPU Interrupt CPUIRQ18"]
    #[inline(always)]
    pub const fn cpuirq18sel(&self) -> &Cpuirq18sel {
        &self.cpuirq18sel
    }
    #[doc = "0x450 - Output Selection for CPU Interrupt SYSTIMC0"]
    #[inline(always)]
    pub const fn systimc0sel(&self) -> &Systimc0sel {
        &self.systimc0sel
    }
    #[doc = "0x454 - Output Selection for CPU Interrupt SYSTIMC1"]
    #[inline(always)]
    pub const fn systimc1sel(&self) -> &Systimc1sel {
        &self.systimc1sel
    }
    #[doc = "0x458 - Output Selection for CPU Interrupt SYSTIMC2"]
    #[inline(always)]
    pub const fn systimc2sel(&self) -> &Systimc2sel {
        &self.systimc2sel
    }
    #[doc = "0x45c - Output Selection for CPU Interrupt SYSTIMC3"]
    #[inline(always)]
    pub const fn systimc3sel(&self) -> &Systimc3sel {
        &self.systimc3sel
    }
    #[doc = "0x460 - Output Selection for CPU Interrupt SYSTIMC4"]
    #[inline(always)]
    pub const fn systimc4sel(&self) -> &Systimc4sel {
        &self.systimc4sel
    }
    #[doc = "0x464 - Output Selection for CPU Interrupt ADCTRG"]
    #[inline(always)]
    pub const fn adctrgsel(&self) -> &Adctrgsel {
        &self.adctrgsel
    }
    #[doc = "0x468 - Output Selection for CPU Interrupt LGPTSYNC"]
    #[inline(always)]
    pub const fn lgptsyncsel(&self) -> &Lgptsyncsel {
        &self.lgptsyncsel
    }
    #[doc = "0x46c - Output Selection for CPU Interrupt LGPT0IN0"]
    #[inline(always)]
    pub const fn lgpt0in0sel(&self) -> &Lgpt0in0sel {
        &self.lgpt0in0sel
    }
    #[doc = "0x470 - Output Selection for CPU Interrupt LGPT0IN1"]
    #[inline(always)]
    pub const fn lgpt0in1sel(&self) -> &Lgpt0in1sel {
        &self.lgpt0in1sel
    }
    #[doc = "0x474 - Output Selection for CPU Interrupt LGPT0IN2"]
    #[inline(always)]
    pub const fn lgpt0in2sel(&self) -> &Lgpt0in2sel {
        &self.lgpt0in2sel
    }
    #[doc = "0x478 - Output Selection for CPU Interrupt LGPT0TEN"]
    #[inline(always)]
    pub const fn lgpt0tensel(&self) -> &Lgpt0tensel {
        &self.lgpt0tensel
    }
    #[doc = "0x47c - Output Selection for CPU Interrupt LGPT1IN0"]
    #[inline(always)]
    pub const fn lgpt1in0sel(&self) -> &Lgpt1in0sel {
        &self.lgpt1in0sel
    }
    #[doc = "0x480 - Output Selection for CPU Interrupt LGPT1IN1"]
    #[inline(always)]
    pub const fn lgpt1in1sel(&self) -> &Lgpt1in1sel {
        &self.lgpt1in1sel
    }
    #[doc = "0x484 - Output Selection for CPU Interrupt LGPT1IN2"]
    #[inline(always)]
    pub const fn lgpt1in2sel(&self) -> &Lgpt1in2sel {
        &self.lgpt1in2sel
    }
    #[doc = "0x488 - Output Selection for CPU Interrupt LGPT1TEN"]
    #[inline(always)]
    pub const fn lgpt1tensel(&self) -> &Lgpt1tensel {
        &self.lgpt1tensel
    }
    #[doc = "0x48c - Output Selection for CPU Interrupt LGPT2IN0"]
    #[inline(always)]
    pub const fn lgpt2in0sel(&self) -> &Lgpt2in0sel {
        &self.lgpt2in0sel
    }
    #[doc = "0x490 - Output Selection for CPU Interrupt LGPT2IN1"]
    #[inline(always)]
    pub const fn lgpt2in1sel(&self) -> &Lgpt2in1sel {
        &self.lgpt2in1sel
    }
    #[doc = "0x494 - Output Selection for CPU Interrupt LGPT2IN2"]
    #[inline(always)]
    pub const fn lgpt2in2sel(&self) -> &Lgpt2in2sel {
        &self.lgpt2in2sel
    }
    #[doc = "0x498 - Output Selection for CPU Interrupt LGPT2TEN"]
    #[inline(always)]
    pub const fn lgpt2tensel(&self) -> &Lgpt2tensel {
        &self.lgpt2tensel
    }
    #[doc = "0x49c - Output Selection for CPU Interrupt LGPT3IN0"]
    #[inline(always)]
    pub const fn lgpt3in0sel(&self) -> &Lgpt3in0sel {
        &self.lgpt3in0sel
    }
    #[doc = "0x4a0 - Output Selection for CPU Interrupt LGPT3IN1"]
    #[inline(always)]
    pub const fn lgpt3in1sel(&self) -> &Lgpt3in1sel {
        &self.lgpt3in1sel
    }
    #[doc = "0x4a4 - Output Selection for CPU Interrupt LGPT3IN2"]
    #[inline(always)]
    pub const fn lgpt3in2sel(&self) -> &Lgpt3in2sel {
        &self.lgpt3in2sel
    }
    #[doc = "0x4a8 - Output Selection for CPU Interrupt LGPT3TEN"]
    #[inline(always)]
    pub const fn lgpt3tensel(&self) -> &Lgpt3tensel {
        &self.lgpt3tensel
    }
    #[doc = "0x4ac - Output Selection for CPU Interrupt LRFDIN0"]
    #[inline(always)]
    pub const fn lrfdin0sel(&self) -> &Lrfdin0sel {
        &self.lrfdin0sel
    }
    #[doc = "0x4b0 - Output Selection for CPU Interrupt LRFDIN1"]
    #[inline(always)]
    pub const fn lrfdin1sel(&self) -> &Lrfdin1sel {
        &self.lrfdin1sel
    }
    #[doc = "0x4b4 - Output Selection for CPU Interrupt LRFDIN2"]
    #[inline(always)]
    pub const fn lrfdin2sel(&self) -> &Lrfdin2sel {
        &self.lrfdin2sel
    }
    #[doc = "0xc00 - Output Selection for DMA CH0"]
    #[inline(always)]
    pub const fn dmach0sel(&self) -> &Dmach0sel {
        &self.dmach0sel
    }
    #[doc = "0xc04 - Output Selection for DMA CH1"]
    #[inline(always)]
    pub const fn dmach1sel(&self) -> &Dmach1sel {
        &self.dmach1sel
    }
    #[doc = "0xc08 - Output Selection for DMA CH2"]
    #[inline(always)]
    pub const fn dmach2sel(&self) -> &Dmach2sel {
        &self.dmach2sel
    }
    #[doc = "0xc0c - Output Selection for DMA CH3"]
    #[inline(always)]
    pub const fn dmach3sel(&self) -> &Dmach3sel {
        &self.dmach3sel
    }
    #[doc = "0xc10 - Output Selection for DMA CH4"]
    #[inline(always)]
    pub const fn dmach4sel(&self) -> &Dmach4sel {
        &self.dmach4sel
    }
    #[doc = "0xc14 - Output Selection for DMA CH5"]
    #[inline(always)]
    pub const fn dmach5sel(&self) -> &Dmach5sel {
        &self.dmach5sel
    }
    #[doc = "0xc18 - Output Selection for DMA CH6"]
    #[inline(always)]
    pub const fn dmach6sel(&self) -> &Dmach6sel {
        &self.dmach6sel
    }
    #[doc = "0xc1c - Output Selection for DMA CH7"]
    #[inline(always)]
    pub const fn dmach7sel(&self) -> &Dmach7sel {
        &self.dmach7sel
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: Extended Description Register. This register provides configuration details of the IP to software drivers and end users.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "Extended Description Register. This register provides configuration details of the IP to software drivers and end users."]
pub mod descex;
#[doc = "DTB (rw) register accessor: Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
pub mod dtb;
#[doc = "NMISEL (rw) register accessor: Output Selection for CPU NMI Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisel`]
module"]
#[doc(alias = "NMISEL")]
pub type Nmisel = crate::Reg<nmisel::NmiselSpec>;
#[doc = "Output Selection for CPU NMI Interrupt"]
pub mod nmisel;
#[doc = "CPUIRQ0SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq0sel`]
module"]
#[doc(alias = "CPUIRQ0SEL")]
pub type Cpuirq0sel = crate::Reg<cpuirq0sel::Cpuirq0selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ0"]
pub mod cpuirq0sel;
#[doc = "CPUIRQ1SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq1sel`]
module"]
#[doc(alias = "CPUIRQ1SEL")]
pub type Cpuirq1sel = crate::Reg<cpuirq1sel::Cpuirq1selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ1"]
pub mod cpuirq1sel;
#[doc = "CPUIRQ2SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq2sel`]
module"]
#[doc(alias = "CPUIRQ2SEL")]
pub type Cpuirq2sel = crate::Reg<cpuirq2sel::Cpuirq2selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ2"]
pub mod cpuirq2sel;
#[doc = "CPUIRQ3SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq3sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq3sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq3sel`]
module"]
#[doc(alias = "CPUIRQ3SEL")]
pub type Cpuirq3sel = crate::Reg<cpuirq3sel::Cpuirq3selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ3"]
pub mod cpuirq3sel;
#[doc = "CPUIRQ4SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq4sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq4sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq4sel`]
module"]
#[doc(alias = "CPUIRQ4SEL")]
pub type Cpuirq4sel = crate::Reg<cpuirq4sel::Cpuirq4selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ4"]
pub mod cpuirq4sel;
#[doc = "CPUIRQ5SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq5sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq5sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq5sel`]
module"]
#[doc(alias = "CPUIRQ5SEL")]
pub type Cpuirq5sel = crate::Reg<cpuirq5sel::Cpuirq5selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ5"]
pub mod cpuirq5sel;
#[doc = "CPUIRQ6SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq6sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq6sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq6sel`]
module"]
#[doc(alias = "CPUIRQ6SEL")]
pub type Cpuirq6sel = crate::Reg<cpuirq6sel::Cpuirq6selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ6"]
pub mod cpuirq6sel;
#[doc = "CPUIRQ7SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq7sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq7sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq7sel`]
module"]
#[doc(alias = "CPUIRQ7SEL")]
pub type Cpuirq7sel = crate::Reg<cpuirq7sel::Cpuirq7selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ7"]
pub mod cpuirq7sel;
#[doc = "CPUIRQ8SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq8sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq8sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq8sel`]
module"]
#[doc(alias = "CPUIRQ8SEL")]
pub type Cpuirq8sel = crate::Reg<cpuirq8sel::Cpuirq8selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ8"]
pub mod cpuirq8sel;
#[doc = "CPUIRQ9SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq9sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq9sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq9sel`]
module"]
#[doc(alias = "CPUIRQ9SEL")]
pub type Cpuirq9sel = crate::Reg<cpuirq9sel::Cpuirq9selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ9"]
pub mod cpuirq9sel;
#[doc = "CPUIRQ10SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq10sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq10sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq10sel`]
module"]
#[doc(alias = "CPUIRQ10SEL")]
pub type Cpuirq10sel = crate::Reg<cpuirq10sel::Cpuirq10selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ10"]
pub mod cpuirq10sel;
#[doc = "CPUIRQ11SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq11sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq11sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq11sel`]
module"]
#[doc(alias = "CPUIRQ11SEL")]
pub type Cpuirq11sel = crate::Reg<cpuirq11sel::Cpuirq11selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ11"]
pub mod cpuirq11sel;
#[doc = "CPUIRQ12SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq12sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq12sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq12sel`]
module"]
#[doc(alias = "CPUIRQ12SEL")]
pub type Cpuirq12sel = crate::Reg<cpuirq12sel::Cpuirq12selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ12"]
pub mod cpuirq12sel;
#[doc = "CPUIRQ13SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq13sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq13sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq13sel`]
module"]
#[doc(alias = "CPUIRQ13SEL")]
pub type Cpuirq13sel = crate::Reg<cpuirq13sel::Cpuirq13selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ13"]
pub mod cpuirq13sel;
#[doc = "CPUIRQ14SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq14sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq14sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq14sel`]
module"]
#[doc(alias = "CPUIRQ14SEL")]
pub type Cpuirq14sel = crate::Reg<cpuirq14sel::Cpuirq14selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ14"]
pub mod cpuirq14sel;
#[doc = "CPUIRQ15SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq15sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq15sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq15sel`]
module"]
#[doc(alias = "CPUIRQ15SEL")]
pub type Cpuirq15sel = crate::Reg<cpuirq15sel::Cpuirq15selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ15"]
pub mod cpuirq15sel;
#[doc = "CPUIRQ16SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq16sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq16sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq16sel`]
module"]
#[doc(alias = "CPUIRQ16SEL")]
pub type Cpuirq16sel = crate::Reg<cpuirq16sel::Cpuirq16selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ16"]
pub mod cpuirq16sel;
#[doc = "CPUIRQ17SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq17sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq17sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq17sel`]
module"]
#[doc(alias = "CPUIRQ17SEL")]
pub type Cpuirq17sel = crate::Reg<cpuirq17sel::Cpuirq17selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ17"]
pub mod cpuirq17sel;
#[doc = "CPUIRQ18SEL (rw) register accessor: Output Selection for CPU Interrupt CPUIRQ18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq18sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq18sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirq18sel`]
module"]
#[doc(alias = "CPUIRQ18SEL")]
pub type Cpuirq18sel = crate::Reg<cpuirq18sel::Cpuirq18selSpec>;
#[doc = "Output Selection for CPU Interrupt CPUIRQ18"]
pub mod cpuirq18sel;
#[doc = "SYSTIMC0SEL (rw) register accessor: Output Selection for CPU Interrupt SYSTIMC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimc0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimc0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimc0sel`]
module"]
#[doc(alias = "SYSTIMC0SEL")]
pub type Systimc0sel = crate::Reg<systimc0sel::Systimc0selSpec>;
#[doc = "Output Selection for CPU Interrupt SYSTIMC0"]
pub mod systimc0sel;
#[doc = "SYSTIMC1SEL (rw) register accessor: Output Selection for CPU Interrupt SYSTIMC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimc1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimc1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimc1sel`]
module"]
#[doc(alias = "SYSTIMC1SEL")]
pub type Systimc1sel = crate::Reg<systimc1sel::Systimc1selSpec>;
#[doc = "Output Selection for CPU Interrupt SYSTIMC1"]
pub mod systimc1sel;
#[doc = "SYSTIMC2SEL (rw) register accessor: Output Selection for CPU Interrupt SYSTIMC2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimc2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimc2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimc2sel`]
module"]
#[doc(alias = "SYSTIMC2SEL")]
pub type Systimc2sel = crate::Reg<systimc2sel::Systimc2selSpec>;
#[doc = "Output Selection for CPU Interrupt SYSTIMC2"]
pub mod systimc2sel;
#[doc = "SYSTIMC3SEL (rw) register accessor: Output Selection for CPU Interrupt SYSTIMC3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimc3sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimc3sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimc3sel`]
module"]
#[doc(alias = "SYSTIMC3SEL")]
pub type Systimc3sel = crate::Reg<systimc3sel::Systimc3selSpec>;
#[doc = "Output Selection for CPU Interrupt SYSTIMC3"]
pub mod systimc3sel;
#[doc = "SYSTIMC4SEL (rw) register accessor: Output Selection for CPU Interrupt SYSTIMC4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimc4sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimc4sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimc4sel`]
module"]
#[doc(alias = "SYSTIMC4SEL")]
pub type Systimc4sel = crate::Reg<systimc4sel::Systimc4selSpec>;
#[doc = "Output Selection for CPU Interrupt SYSTIMC4"]
pub mod systimc4sel;
#[doc = "ADCTRGSEL (rw) register accessor: Output Selection for CPU Interrupt ADCTRG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrgsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrgsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctrgsel`]
module"]
#[doc(alias = "ADCTRGSEL")]
pub type Adctrgsel = crate::Reg<adctrgsel::AdctrgselSpec>;
#[doc = "Output Selection for CPU Interrupt ADCTRG"]
pub mod adctrgsel;
#[doc = "LGPTSYNCSEL (rw) register accessor: Output Selection for CPU Interrupt LGPTSYNC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgptsyncsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgptsyncsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgptsyncsel`]
module"]
#[doc(alias = "LGPTSYNCSEL")]
pub type Lgptsyncsel = crate::Reg<lgptsyncsel::LgptsyncselSpec>;
#[doc = "Output Selection for CPU Interrupt LGPTSYNC"]
pub mod lgptsyncsel;
#[doc = "LGPT0IN0SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT0IN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt0in0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt0in0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt0in0sel`]
module"]
#[doc(alias = "LGPT0IN0SEL")]
pub type Lgpt0in0sel = crate::Reg<lgpt0in0sel::Lgpt0in0selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT0IN0"]
pub mod lgpt0in0sel;
#[doc = "LGPT0IN1SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT0IN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt0in1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt0in1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt0in1sel`]
module"]
#[doc(alias = "LGPT0IN1SEL")]
pub type Lgpt0in1sel = crate::Reg<lgpt0in1sel::Lgpt0in1selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT0IN1"]
pub mod lgpt0in1sel;
#[doc = "LGPT0IN2SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT0IN2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt0in2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt0in2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt0in2sel`]
module"]
#[doc(alias = "LGPT0IN2SEL")]
pub type Lgpt0in2sel = crate::Reg<lgpt0in2sel::Lgpt0in2selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT0IN2"]
pub mod lgpt0in2sel;
#[doc = "LGPT0TENSEL (rw) register accessor: Output Selection for CPU Interrupt LGPT0TEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt0tensel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt0tensel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt0tensel`]
module"]
#[doc(alias = "LGPT0TENSEL")]
pub type Lgpt0tensel = crate::Reg<lgpt0tensel::Lgpt0tenselSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT0TEN"]
pub mod lgpt0tensel;
#[doc = "LGPT1IN0SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT1IN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt1in0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt1in0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt1in0sel`]
module"]
#[doc(alias = "LGPT1IN0SEL")]
pub type Lgpt1in0sel = crate::Reg<lgpt1in0sel::Lgpt1in0selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT1IN0"]
pub mod lgpt1in0sel;
#[doc = "LGPT1IN1SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT1IN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt1in1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt1in1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt1in1sel`]
module"]
#[doc(alias = "LGPT1IN1SEL")]
pub type Lgpt1in1sel = crate::Reg<lgpt1in1sel::Lgpt1in1selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT1IN1"]
pub mod lgpt1in1sel;
#[doc = "LGPT1IN2SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT1IN2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt1in2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt1in2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt1in2sel`]
module"]
#[doc(alias = "LGPT1IN2SEL")]
pub type Lgpt1in2sel = crate::Reg<lgpt1in2sel::Lgpt1in2selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT1IN2"]
pub mod lgpt1in2sel;
#[doc = "LGPT1TENSEL (rw) register accessor: Output Selection for CPU Interrupt LGPT1TEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt1tensel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt1tensel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt1tensel`]
module"]
#[doc(alias = "LGPT1TENSEL")]
pub type Lgpt1tensel = crate::Reg<lgpt1tensel::Lgpt1tenselSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT1TEN"]
pub mod lgpt1tensel;
#[doc = "LGPT2IN0SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT2IN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt2in0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt2in0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt2in0sel`]
module"]
#[doc(alias = "LGPT2IN0SEL")]
pub type Lgpt2in0sel = crate::Reg<lgpt2in0sel::Lgpt2in0selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT2IN0"]
pub mod lgpt2in0sel;
#[doc = "LGPT2IN1SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT2IN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt2in1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt2in1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt2in1sel`]
module"]
#[doc(alias = "LGPT2IN1SEL")]
pub type Lgpt2in1sel = crate::Reg<lgpt2in1sel::Lgpt2in1selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT2IN1"]
pub mod lgpt2in1sel;
#[doc = "LGPT2IN2SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT2IN2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt2in2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt2in2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt2in2sel`]
module"]
#[doc(alias = "LGPT2IN2SEL")]
pub type Lgpt2in2sel = crate::Reg<lgpt2in2sel::Lgpt2in2selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT2IN2"]
pub mod lgpt2in2sel;
#[doc = "LGPT2TENSEL (rw) register accessor: Output Selection for CPU Interrupt LGPT2TEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt2tensel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt2tensel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt2tensel`]
module"]
#[doc(alias = "LGPT2TENSEL")]
pub type Lgpt2tensel = crate::Reg<lgpt2tensel::Lgpt2tenselSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT2TEN"]
pub mod lgpt2tensel;
#[doc = "LGPT3IN0SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT3IN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt3in0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt3in0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt3in0sel`]
module"]
#[doc(alias = "LGPT3IN0SEL")]
pub type Lgpt3in0sel = crate::Reg<lgpt3in0sel::Lgpt3in0selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT3IN0"]
pub mod lgpt3in0sel;
#[doc = "LGPT3IN1SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT3IN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt3in1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt3in1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt3in1sel`]
module"]
#[doc(alias = "LGPT3IN1SEL")]
pub type Lgpt3in1sel = crate::Reg<lgpt3in1sel::Lgpt3in1selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT3IN1"]
pub mod lgpt3in1sel;
#[doc = "LGPT3IN2SEL (rw) register accessor: Output Selection for CPU Interrupt LGPT3IN2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt3in2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt3in2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt3in2sel`]
module"]
#[doc(alias = "LGPT3IN2SEL")]
pub type Lgpt3in2sel = crate::Reg<lgpt3in2sel::Lgpt3in2selSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT3IN2"]
pub mod lgpt3in2sel;
#[doc = "LGPT3TENSEL (rw) register accessor: Output Selection for CPU Interrupt LGPT3TEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt3tensel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt3tensel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lgpt3tensel`]
module"]
#[doc(alias = "LGPT3TENSEL")]
pub type Lgpt3tensel = crate::Reg<lgpt3tensel::Lgpt3tenselSpec>;
#[doc = "Output Selection for CPU Interrupt LGPT3TEN"]
pub mod lgpt3tensel;
#[doc = "LRFDIN0SEL (rw) register accessor: Output Selection for CPU Interrupt LRFDIN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrfdin0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrfdin0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrfdin0sel`]
module"]
#[doc(alias = "LRFDIN0SEL")]
pub type Lrfdin0sel = crate::Reg<lrfdin0sel::Lrfdin0selSpec>;
#[doc = "Output Selection for CPU Interrupt LRFDIN0"]
pub mod lrfdin0sel;
#[doc = "LRFDIN1SEL (rw) register accessor: Output Selection for CPU Interrupt LRFDIN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrfdin1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrfdin1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrfdin1sel`]
module"]
#[doc(alias = "LRFDIN1SEL")]
pub type Lrfdin1sel = crate::Reg<lrfdin1sel::Lrfdin1selSpec>;
#[doc = "Output Selection for CPU Interrupt LRFDIN1"]
pub mod lrfdin1sel;
#[doc = "LRFDIN2SEL (rw) register accessor: Output Selection for CPU Interrupt LRFDIN2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrfdin2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrfdin2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrfdin2sel`]
module"]
#[doc(alias = "LRFDIN2SEL")]
pub type Lrfdin2sel = crate::Reg<lrfdin2sel::Lrfdin2selSpec>;
#[doc = "Output Selection for CPU Interrupt LRFDIN2"]
pub mod lrfdin2sel;
#[doc = "DMACH0SEL (rw) register accessor: Output Selection for DMA CH0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0sel`]
module"]
#[doc(alias = "DMACH0SEL")]
pub type Dmach0sel = crate::Reg<dmach0sel::Dmach0selSpec>;
#[doc = "Output Selection for DMA CH0"]
pub mod dmach0sel;
#[doc = "DMACH1SEL (rw) register accessor: Output Selection for DMA CH1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1sel`]
module"]
#[doc(alias = "DMACH1SEL")]
pub type Dmach1sel = crate::Reg<dmach1sel::Dmach1selSpec>;
#[doc = "Output Selection for DMA CH1"]
pub mod dmach1sel;
#[doc = "DMACH2SEL (rw) register accessor: Output Selection for DMA CH2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach2sel`]
module"]
#[doc(alias = "DMACH2SEL")]
pub type Dmach2sel = crate::Reg<dmach2sel::Dmach2selSpec>;
#[doc = "Output Selection for DMA CH2"]
pub mod dmach2sel;
#[doc = "DMACH3SEL (rw) register accessor: Output Selection for DMA CH3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach3sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach3sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach3sel`]
module"]
#[doc(alias = "DMACH3SEL")]
pub type Dmach3sel = crate::Reg<dmach3sel::Dmach3selSpec>;
#[doc = "Output Selection for DMA CH3"]
pub mod dmach3sel;
#[doc = "DMACH4SEL (rw) register accessor: Output Selection for DMA CH4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach4sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach4sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach4sel`]
module"]
#[doc(alias = "DMACH4SEL")]
pub type Dmach4sel = crate::Reg<dmach4sel::Dmach4selSpec>;
#[doc = "Output Selection for DMA CH4"]
pub mod dmach4sel;
#[doc = "DMACH5SEL (rw) register accessor: Output Selection for DMA CH5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach5sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach5sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach5sel`]
module"]
#[doc(alias = "DMACH5SEL")]
pub type Dmach5sel = crate::Reg<dmach5sel::Dmach5selSpec>;
#[doc = "Output Selection for DMA CH5"]
pub mod dmach5sel;
#[doc = "DMACH6SEL (rw) register accessor: Output Selection for DMA CH6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach6sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach6sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach6sel`]
module"]
#[doc(alias = "DMACH6SEL")]
pub type Dmach6sel = crate::Reg<dmach6sel::Dmach6selSpec>;
#[doc = "Output Selection for DMA CH6"]
pub mod dmach6sel;
#[doc = "DMACH7SEL (rw) register accessor: Output Selection for DMA CH7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach7sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach7sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach7sel`]
module"]
#[doc(alias = "DMACH7SEL")]
pub type Dmach7sel = crate::Reg<dmach7sel::Dmach7selSpec>;
#[doc = "Output Selection for DMA CH7"]
pub mod dmach7sel;
