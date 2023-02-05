#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    ctl: Ctl,
    armset: Armset,
    armclr: Armclr,
    _reserved4: [u8; 0x08],
    time8u: Time8u,
    time524m: Time524m,
    _reserved6: [u8; 0x08],
    ch0cc8u: Ch0cc8u,
    _reserved7: [u8; 0x0c],
    ch1cc8u: Ch1cc8u,
    ch1cfg: Ch1cfg,
    _reserved9: [u8; 0x04],
    imask: Imask,
    ris: Ris,
    mis: Mis,
    iset: Iset,
    iclr: Iclr,
    imset: Imset,
    imclr: Imclr,
    emu: Emu,
    dtb: Dtb,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - RTC Control register. This register controls resetting the of RTC counter"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08 - RTC channel mode set register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 has no effect on the compare channel. While write of 1'b1 for capture channel will arm it in capture mode if it is disabled."]
    #[inline(always)]
    pub const fn armset(&self) -> &Armset {
        &self.armset
    }
    #[doc = "0x0c - RTC channel mode clear register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 for capture/compare channel will disarm it without triggering event unless a compare/capture event happens in the same cycle."]
    #[inline(always)]
    pub const fn armclr(&self) -> &Armclr {
        &self.armclr
    }
    #[doc = "0x18 - RTC Time value register. 32-bit unsigned integer representing \\[34:3\\]
time slice of the real time clock counter. The counter runs on LFCLK. This field has a resolution of 8us, and range of about 9.5 hours."]
    #[inline(always)]
    pub const fn time8u(&self) -> &Time8u {
        &self.time8u
    }
    #[doc = "0x1c - RTC time value register. 32-bit unsigned integer representing \\[50:19\\]
time slice of the real time clock counter. This field has a resolution of about 0.5s and a range of about 71.4 years."]
    #[inline(always)]
    pub const fn time524m(&self) -> &Time524m {
        &self.time524m
    }
    #[doc = "0x28 - Channel 0 compare value. A write to this register automatically enables the channel to trigger an event when RTC timer reaches the programmed value or if the programmed value is 1 sec in the past."]
    #[inline(always)]
    pub const fn ch0cc8u(&self) -> &Ch0cc8u {
        &self.ch0cc8u
    }
    #[doc = "0x38 - Channel 1 capture value. This register captures the RTC time slice \\[34:3\\]
on each selected edge of the capture event when the ARMSET.CH1 = 1."]
    #[inline(always)]
    pub const fn ch1cc8u(&self) -> &Ch1cc8u {
        &self.ch1cc8u
    }
    #[doc = "0x3c - Channel 1 configuration register. This register can be used to select the capture edge for generating the capture event."]
    #[inline(always)]
    pub const fn ch1cfg(&self) -> &Ch1cfg {
        &self.ch1cfg
    }
    #[doc = "0x44 - Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x48 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
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
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "CTL (rw) register accessor: RTC Control register. This register controls resetting the of RTC counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "RTC Control register. This register controls resetting the of RTC counter"]
pub mod ctl;
#[doc = "ARMSET (rw) register accessor: RTC channel mode set register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 has no effect on the compare channel. While write of 1'b1 for capture channel will arm it in capture mode if it is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@armset`]
module"]
#[doc(alias = "ARMSET")]
pub type Armset = crate::Reg<armset::ArmsetSpec>;
#[doc = "RTC channel mode set register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 has no effect on the compare channel. While write of 1'b1 for capture channel will arm it in capture mode if it is disabled."]
pub mod armset;
#[doc = "ARMCLR (rw) register accessor: RTC channel mode clear register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 for capture/compare channel will disarm it without triggering event unless a compare/capture event happens in the same cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`armclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`armclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@armclr`]
module"]
#[doc(alias = "ARMCLR")]
pub type Armclr = crate::Reg<armclr::ArmclrSpec>;
#[doc = "RTC channel mode clear register. Read to each bit field of this register provides the current channel mode. - Read of 1'b0 indicates the channel is unarmed. - Read of 1'b1 indicates the channel is either in capture or compare mode. A write to each bitfield of this register the following effect: - Write of 1'b0 has no effect on channel mode. - Write of 1'b1 for capture/compare channel will disarm it without triggering event unless a compare/capture event happens in the same cycle."]
pub mod armclr;
#[doc = "TIME8U (rw) register accessor: RTC Time value register. 32-bit unsigned integer representing \\[34:3\\]
time slice of the real time clock counter. The counter runs on LFCLK. This field has a resolution of 8us, and range of about 9.5 hours.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time8u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time8u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time8u`]
module"]
#[doc(alias = "TIME8U")]
pub type Time8u = crate::Reg<time8u::Time8uSpec>;
#[doc = "RTC Time value register. 32-bit unsigned integer representing \\[34:3\\]
time slice of the real time clock counter. The counter runs on LFCLK. This field has a resolution of 8us, and range of about 9.5 hours."]
pub mod time8u;
#[doc = "TIME524M (rw) register accessor: RTC time value register. 32-bit unsigned integer representing \\[50:19\\]
time slice of the real time clock counter. This field has a resolution of about 0.5s and a range of about 71.4 years.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time524m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time524m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time524m`]
module"]
#[doc(alias = "TIME524M")]
pub type Time524m = crate::Reg<time524m::Time524mSpec>;
#[doc = "RTC time value register. 32-bit unsigned integer representing \\[50:19\\]
time slice of the real time clock counter. This field has a resolution of about 0.5s and a range of about 71.4 years."]
pub mod time524m;
#[doc = "CH0CC8U (rw) register accessor: Channel 0 compare value. A write to this register automatically enables the channel to trigger an event when RTC timer reaches the programmed value or if the programmed value is 1 sec in the past.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cc8u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cc8u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cc8u`]
module"]
#[doc(alias = "CH0CC8U")]
pub type Ch0cc8u = crate::Reg<ch0cc8u::Ch0cc8uSpec>;
#[doc = "Channel 0 compare value. A write to this register automatically enables the channel to trigger an event when RTC timer reaches the programmed value or if the programmed value is 1 sec in the past."]
pub mod ch0cc8u;
#[doc = "CH1CC8U (rw) register accessor: Channel 1 capture value. This register captures the RTC time slice \\[34:3\\]
on each selected edge of the capture event when the ARMSET.CH1 = 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cc8u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cc8u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cc8u`]
module"]
#[doc(alias = "CH1CC8U")]
pub type Ch1cc8u = crate::Reg<ch1cc8u::Ch1cc8uSpec>;
#[doc = "Channel 1 capture value. This register captures the RTC time slice \\[34:3\\]
on each selected edge of the capture event when the ARMSET.CH1 = 1."]
pub mod ch1cc8u;
#[doc = "CH1CFG (rw) register accessor: Channel 1 configuration register. This register can be used to select the capture edge for generating the capture event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cfg`]
module"]
#[doc(alias = "CH1CFG")]
pub type Ch1cfg = crate::Reg<ch1cfg::Ch1cfgSpec>;
#[doc = "Channel 1 configuration register. This register can be used to select the capture edge for generating the capture event."]
pub mod ch1cfg;
#[doc = "IMASK (rw) register accessor: Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
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
