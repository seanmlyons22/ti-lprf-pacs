#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    _reserved1: [u8; 0x40],
    imask: Imask,
    ris: Ris,
    mis: Mis,
    iset: Iset,
    iclr: Iclr,
    imset: Imset,
    imclr: Imclr,
    emu: Emu,
    dtb: Dtb,
    _reserved10: [u8; 0x98],
    time250n: Time250n,
    time1u: Time1u,
    out: Out,
    ch0cfg: Ch0cfg,
    ch1cfg: Ch1cfg,
    ch2cfg: Ch2cfg,
    ch3cfg: Ch3cfg,
    ch4cfg: Ch4cfg,
    ch0cc: Ch0cc,
    ch1cc: Ch1cc,
    ch2cc: Ch2cc,
    ch3cc: Ch3cc,
    ch4cc: Ch4cc,
    timebit: Timebit,
    kp: Kp,
    ki: Ki,
    status: Status,
    armset: Armset,
    armclr: Armclr,
    ch0ccsr: Ch0ccsr,
    ch1ccsr: Ch1ccsr,
    ch2ccsr: Ch2ccsr,
    ch3ccsr: Ch3ccsr,
    ch4ccsr: Ch4ccsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register identifies the peripheral and its exact version."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x44 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x48 - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x4c - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x50 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x54 - Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x58 - Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x5c - Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
    #[doc = "0x60 - Emulation control register. This register controls the behavior of the IP related to core halted input."]
    #[inline(always)]
    pub const fn emu(&self) -> &Emu {
        &self.emu
    }
    #[doc = "0x64 - Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
    #[doc = "0x100 - Systimer Counter Value. This 32-bit value reads out bits \\[31:0\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 250ns with a range of about 17.9m."]
    #[inline(always)]
    pub const fn time250n(&self) -> &Time250n {
        &self.time250n
    }
    #[doc = "0x104 - Systimer Counter Value. This 32-bit value reads out bits\\[33:2\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 1us with a range of about 1 h 11m."]
    #[inline(always)]
    pub const fn time1u(&self) -> &Time1u {
        &self.time1u
    }
    #[doc = "0x108 - Systimer's channel Output Event Values"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x10c - Systimer channel 0 configuration. This channel has configurability for 250ns and 1us based capture and compare operations."]
    #[inline(always)]
    pub const fn ch0cfg(&self) -> &Ch0cfg {
        &self.ch0cfg
    }
    #[doc = "0x110 - Systimer channel 1 configuration. This channel works in 1us based capture and compare operations."]
    #[inline(always)]
    pub const fn ch1cfg(&self) -> &Ch1cfg {
        &self.ch1cfg
    }
    #[doc = "0x114 - Systimer channel 2 configuration. This channel works in 250ns based capture and compare operations."]
    #[inline(always)]
    pub const fn ch2cfg(&self) -> &Ch2cfg {
        &self.ch2cfg
    }
    #[doc = "0x118 - Systimer channel 3 configuration. This channel works in 250ns based capture and compare operations."]
    #[inline(always)]
    pub const fn ch3cfg(&self) -> &Ch3cfg {
        &self.ch3cfg
    }
    #[doc = "0x11c - Systimer channel 4 configuration. This channel works in 250ns based capture and compare operations."]
    #[inline(always)]
    pub const fn ch4cfg(&self) -> &Ch4cfg {
        &self.ch4cfg
    }
    #[doc = "0x120 - System Timer channel 0 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
    #[inline(always)]
    pub const fn ch0cc(&self) -> &Ch0cc {
        &self.ch0cc
    }
    #[doc = "0x124 - System Timer channel 1 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
    #[inline(always)]
    pub const fn ch1cc(&self) -> &Ch1cc {
        &self.ch1cc
    }
    #[doc = "0x128 - System Timer channel 2 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
    #[inline(always)]
    pub const fn ch2cc(&self) -> &Ch2cc {
        &self.ch2cc
    }
    #[doc = "0x12c - System Timer channel 3 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
    #[inline(always)]
    pub const fn ch3cc(&self) -> &Ch3cc {
        &self.ch3cc
    }
    #[doc = "0x130 - System Timer channel 4 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
    #[inline(always)]
    pub const fn ch4cc(&self) -> &Ch4cc {
        &self.ch4cc
    }
    #[doc = "0x134 - Systimer Bit. This Register will be used to specify which TIME bit is required by LGPT to be forwarded from SYSTIMER."]
    #[inline(always)]
    pub const fn timebit(&self) -> &Timebit {
        &self.timebit
    }
    #[doc = "0x138 - PI filter's Proportional Gain Value"]
    #[inline(always)]
    pub const fn kp(&self) -> &Kp {
        &self.kp
    }
    #[doc = "0x13c - PI filter's Accumulator's Gain Value"]
    #[inline(always)]
    pub const fn ki(&self) -> &Ki {
        &self.ki
    }
    #[doc = "0x140 - Systimer status register. This register can be used to read the running status of the timer and to resync the Systimer with RTC."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x144 - ARMSET on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMSET has for each channel the following effect: 1. If ARMSTA\\[x\\]==0 -$gt; no effect 2. If ARMSTA\\[x\\]==1 and channel x is in CAPTURE state then no effect on the channel 3. Else Set channel in COMPARE mode using existing CHxVAL value"]
    #[inline(always)]
    pub const fn armset(&self) -> &Armset {
        &self.armset
    }
    #[doc = "0x148 - ARMCLR on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMCLR has for each channel the following effect: 1. If ARMCLR\\[x\\]==0 no effect 2. Else Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
    #[inline(always)]
    pub const fn armclr(&self) -> &Armclr {
        &self.armclr
    }
    #[doc = "0x14c - Save/restore alias register for channel 0. A read to this register behaves exactly as a read to CH0CC. A write to CH0CCSR sets CH0VAL value of register without affecting channel state or configuration"]
    #[inline(always)]
    pub const fn ch0ccsr(&self) -> &Ch0ccsr {
        &self.ch0ccsr
    }
    #[doc = "0x150 - Save/restore alias registers channel 1. i. A read to CH1CCSR behaves exactly as a read to CH1VAL. A write to this register sets CH0CC.VAL without affecting channel state or configuration."]
    #[inline(always)]
    pub const fn ch1ccsr(&self) -> &Ch1ccsr {
        &self.ch1ccsr
    }
    #[doc = "0x154 - Save/restore alias registers channel 2. i. A read to CH2CCSR behaves exactly as a read to CH2VAL. A write to CH2CCSR sets CH2VAL value of register without affecting channel state or configuration"]
    #[inline(always)]
    pub const fn ch2ccsr(&self) -> &Ch2ccsr {
        &self.ch2ccsr
    }
    #[doc = "0x158 - Save/restore alias registers channel 3. i. A read to CH3CCSR behaves exactly as a read to CH3VAL. A write to CH3CCSR sets CH3VAL value of register without affecting channel state or configuration."]
    #[inline(always)]
    pub const fn ch3ccsr(&self) -> &Ch3ccsr {
        &self.ch3ccsr
    }
    #[doc = "0x15c - Save/restore alias registers channel 4. i. A read to CH4CCSR behaves exactly as a read to CH4VAL. A write to CH4CCSR sets CH4VAL value of register without affecting channel state or configuration."]
    #[inline(always)]
    pub const fn ch4ccsr(&self) -> &Ch4ccsr {
        &self.ch4ccsr
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register identifies the peripheral and its exact version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register identifies the peripheral and its exact version."]
pub mod desc;
#[doc = "IMASK (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod mis;
#[doc = "ISET (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
pub mod iclr;
#[doc = "IMSET (rw) register accessor: Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imset`]
module"]
#[doc(alias = "IMSET")]
pub type Imset = crate::Reg<imset::ImsetSpec>;
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
pub mod imset;
#[doc = "IMCLR (rw) register accessor: Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imclr`]
module"]
#[doc(alias = "IMCLR")]
pub type Imclr = crate::Reg<imclr::ImclrSpec>;
#[doc = "Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
pub mod imclr;
#[doc = "EMU (rw) register accessor: Emulation control register. This register controls the behavior of the IP related to core halted input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emu`]
module"]
#[doc(alias = "EMU")]
pub type Emu = crate::Reg<emu::EmuSpec>;
#[doc = "Emulation control register. This register controls the behavior of the IP related to core halted input."]
pub mod emu;
#[doc = "DTB (rw) register accessor: Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
pub mod dtb;
#[doc = "TIME250N (rw) register accessor: Systimer Counter Value. This 32-bit value reads out bits \\[31:0\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 250ns with a range of about 17.9m.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time250n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time250n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time250n`]
module"]
#[doc(alias = "TIME250N")]
pub type Time250n = crate::Reg<time250n::Time250nSpec>;
#[doc = "Systimer Counter Value. This 32-bit value reads out bits \\[31:0\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 250ns with a range of about 17.9m."]
pub mod time250n;
#[doc = "TIME1U (rw) register accessor: Systimer Counter Value. This 32-bit value reads out bits\\[33:2\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 1us with a range of about 1 h 11m.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time1u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time1u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time1u`]
module"]
#[doc(alias = "TIME1U")]
pub type Time1u = crate::Reg<time1u::Time1uSpec>;
#[doc = "Systimer Counter Value. This 32-bit value reads out bits\\[33:2\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 1us with a range of about 1 h 11m."]
pub mod time1u;
#[doc = "OUT (rw) register accessor: Systimer's channel Output Event Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Systimer's channel Output Event Values"]
pub mod out;
#[doc = "CH0CFG (rw) register accessor: Systimer channel 0 configuration. This channel has configurability for 250ns and 1us based capture and compare operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cfg`]
module"]
#[doc(alias = "CH0CFG")]
pub type Ch0cfg = crate::Reg<ch0cfg::Ch0cfgSpec>;
#[doc = "Systimer channel 0 configuration. This channel has configurability for 250ns and 1us based capture and compare operations."]
pub mod ch0cfg;
#[doc = "CH1CFG (rw) register accessor: Systimer channel 1 configuration. This channel works in 1us based capture and compare operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cfg`]
module"]
#[doc(alias = "CH1CFG")]
pub type Ch1cfg = crate::Reg<ch1cfg::Ch1cfgSpec>;
#[doc = "Systimer channel 1 configuration. This channel works in 1us based capture and compare operations."]
pub mod ch1cfg;
#[doc = "CH2CFG (rw) register accessor: Systimer channel 2 configuration. This channel works in 250ns based capture and compare operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cfg`]
module"]
#[doc(alias = "CH2CFG")]
pub type Ch2cfg = crate::Reg<ch2cfg::Ch2cfgSpec>;
#[doc = "Systimer channel 2 configuration. This channel works in 250ns based capture and compare operations."]
pub mod ch2cfg;
#[doc = "CH3CFG (rw) register accessor: Systimer channel 3 configuration. This channel works in 250ns based capture and compare operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cfg`]
module"]
#[doc(alias = "CH3CFG")]
pub type Ch3cfg = crate::Reg<ch3cfg::Ch3cfgSpec>;
#[doc = "Systimer channel 3 configuration. This channel works in 250ns based capture and compare operations."]
pub mod ch3cfg;
#[doc = "CH4CFG (rw) register accessor: Systimer channel 4 configuration. This channel works in 250ns based capture and compare operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cfg`]
module"]
#[doc(alias = "CH4CFG")]
pub type Ch4cfg = crate::Reg<ch4cfg::Ch4cfgSpec>;
#[doc = "Systimer channel 4 configuration. This channel works in 250ns based capture and compare operations."]
pub mod ch4cfg;
#[doc = "CH0CC (rw) register accessor: System Timer channel 0 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cc`]
module"]
#[doc(alias = "CH0CC")]
pub type Ch0cc = crate::Reg<ch0cc::Ch0ccSpec>;
#[doc = "System Timer channel 0 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
pub mod ch0cc;
#[doc = "CH1CC (rw) register accessor: System Timer channel 1 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cc`]
module"]
#[doc(alias = "CH1CC")]
pub type Ch1cc = crate::Reg<ch1cc::Ch1ccSpec>;
#[doc = "System Timer channel 1 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
pub mod ch1cc;
#[doc = "CH2CC (rw) register accessor: System Timer channel 2 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cc`]
module"]
#[doc(alias = "CH2CC")]
pub type Ch2cc = crate::Reg<ch2cc::Ch2ccSpec>;
#[doc = "System Timer channel 2 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
pub mod ch2cc;
#[doc = "CH3CC (rw) register accessor: System Timer channel 3 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cc`]
module"]
#[doc(alias = "CH3CC")]
pub type Ch3cc = crate::Reg<ch3cc::Ch3ccSpec>;
#[doc = "System Timer channel 3 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
pub mod ch3cc;
#[doc = "CH4CC (rw) register accessor: System Timer channel 4 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cc`]
module"]
#[doc(alias = "CH4CC")]
pub type Ch4cc = crate::Reg<ch4cc::Ch4ccSpec>;
#[doc = "System Timer channel 4 Capture/Compare Register. This Register when written with any compare value will arm the channel to work in compare mode."]
pub mod ch4cc;
#[doc = "TIMEBIT (rw) register accessor: Systimer Bit. This Register will be used to specify which TIME bit is required by LGPT to be forwarded from SYSTIMER.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timebit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timebit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timebit`]
module"]
#[doc(alias = "TIMEBIT")]
pub type Timebit = crate::Reg<timebit::TimebitSpec>;
#[doc = "Systimer Bit. This Register will be used to specify which TIME bit is required by LGPT to be forwarded from SYSTIMER."]
pub mod timebit;
#[doc = "KP (rw) register accessor: PI filter's Proportional Gain Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kp`]
module"]
#[doc(alias = "KP")]
pub type Kp = crate::Reg<kp::KpSpec>;
#[doc = "PI filter's Proportional Gain Value"]
pub mod kp;
#[doc = "KI (rw) register accessor: PI filter's Accumulator's Gain Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ki::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ki::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ki`]
module"]
#[doc(alias = "KI")]
pub type Ki = crate::Reg<ki::KiSpec>;
#[doc = "PI filter's Accumulator's Gain Value"]
pub mod ki;
#[doc = "STATUS (rw) register accessor: Systimer status register. This register can be used to read the running status of the timer and to resync the Systimer with RTC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Systimer status register. This register can be used to read the running status of the timer and to resync the Systimer with RTC."]
pub mod status;
#[doc = "ARMSET (rw) register accessor: ARMSET on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMSET has for each channel the following effect: 1. If ARMSTA\\[x\\]==0 -$gt; no effect 2. If ARMSTA\\[x\\]==1 and channel x is in CAPTURE state then no effect on the channel 3. Else Set channel in COMPARE mode using existing CHxVAL value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@armset`]
module"]
#[doc(alias = "ARMSET")]
pub type Armset = crate::Reg<armset::ArmsetSpec>;
#[doc = "ARMSET on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMSET has for each channel the following effect: 1. If ARMSTA\\[x\\]==0 -$gt; no effect 2. If ARMSTA\\[x\\]==1 and channel x is in CAPTURE state then no effect on the channel 3. Else Set channel in COMPARE mode using existing CHxVAL value"]
pub mod armset;
#[doc = "ARMCLR (rw) register accessor: ARMCLR on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMCLR has for each channel the following effect: 1. If ARMCLR\\[x\\]==0 no effect 2. Else Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@armclr`]
module"]
#[doc(alias = "ARMCLR")]
pub type Armclr = crate::Reg<armclr::ArmclrSpec>;
#[doc = "ARMCLR on read gives out the status of the 5 channels 1. channel state UNARMED returns 0 2. channel state CAPTURE or COMPARE returns 1 A write to ARMCLR has for each channel the following effect: 1. If ARMCLR\\[x\\]==0 no effect 2. Else Set channel in UNARMED state without triggering event unless a compare/capture event happens in the same cycle"]
pub mod armclr;
#[doc = "CH0CCSR (rw) register accessor: Save/restore alias register for channel 0. A read to this register behaves exactly as a read to CH0CC. A write to CH0CCSR sets CH0VAL value of register without affecting channel state or configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ccsr`]
module"]
#[doc(alias = "CH0CCSR")]
pub type Ch0ccsr = crate::Reg<ch0ccsr::Ch0ccsrSpec>;
#[doc = "Save/restore alias register for channel 0. A read to this register behaves exactly as a read to CH0CC. A write to CH0CCSR sets CH0VAL value of register without affecting channel state or configuration"]
pub mod ch0ccsr;
#[doc = "CH1CCSR (rw) register accessor: Save/restore alias registers channel 1. i. A read to CH1CCSR behaves exactly as a read to CH1VAL. A write to this register sets CH0CC.VAL without affecting channel state or configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccsr`]
module"]
#[doc(alias = "CH1CCSR")]
pub type Ch1ccsr = crate::Reg<ch1ccsr::Ch1ccsrSpec>;
#[doc = "Save/restore alias registers channel 1. i. A read to CH1CCSR behaves exactly as a read to CH1VAL. A write to this register sets CH0CC.VAL without affecting channel state or configuration."]
pub mod ch1ccsr;
#[doc = "CH2CCSR (rw) register accessor: Save/restore alias registers channel 2. i. A read to CH2CCSR behaves exactly as a read to CH2VAL. A write to CH2CCSR sets CH2VAL value of register without affecting channel state or configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccsr`]
module"]
#[doc(alias = "CH2CCSR")]
pub type Ch2ccsr = crate::Reg<ch2ccsr::Ch2ccsrSpec>;
#[doc = "Save/restore alias registers channel 2. i. A read to CH2CCSR behaves exactly as a read to CH2VAL. A write to CH2CCSR sets CH2VAL value of register without affecting channel state or configuration"]
pub mod ch2ccsr;
#[doc = "CH3CCSR (rw) register accessor: Save/restore alias registers channel 3. i. A read to CH3CCSR behaves exactly as a read to CH3VAL. A write to CH3CCSR sets CH3VAL value of register without affecting channel state or configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccsr`]
module"]
#[doc(alias = "CH3CCSR")]
pub type Ch3ccsr = crate::Reg<ch3ccsr::Ch3ccsrSpec>;
#[doc = "Save/restore alias registers channel 3. i. A read to CH3CCSR behaves exactly as a read to CH3VAL. A write to CH3CCSR sets CH3VAL value of register without affecting channel state or configuration."]
pub mod ch3ccsr;
#[doc = "CH4CCSR (rw) register accessor: Save/restore alias registers channel 4. i. A read to CH4CCSR behaves exactly as a read to CH4VAL. A write to CH4CCSR sets CH4VAL value of register without affecting channel state or configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4ccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4ccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ccsr`]
module"]
#[doc(alias = "CH4CCSR")]
pub type Ch4ccsr = crate::Reg<ch4ccsr::Ch4ccsrSpec>;
#[doc = "Save/restore alias registers channel 4. i. A read to CH4CCSR behaves exactly as a read to CH4VAL. A write to CH4CCSR sets CH4VAL value of register without affecting channel state or configuration."]
pub mod ch4ccsr;
