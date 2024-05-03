#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    fetchstat: Fetchstat,
    cpustat: Cpustat,
    wustat: Wustat,
    reg1_0: Reg1_0,
    reg3_2: Reg3_2,
    reg5_4: Reg5_4,
    reg7_6: Reg7_6,
    loopaddr: Loopaddr,
    loopcnt: Loopcnt,
    nonsecddiacc0: Nonsecddiacc0,
    nonsecddiacc1: Nonsecddiacc1,
    nonsecddiacc2: Nonsecddiacc2,
    nonsecddiacc3: Nonsecddiacc3,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fetchstat(&self) -> &Fetchstat {
        &self.fetchstat
    }
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn cpustat(&self) -> &Cpustat {
        &self.cpustat
    }
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn wustat(&self) -> &Wustat {
        &self.wustat
    }
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn reg1_0(&self) -> &Reg1_0 {
        &self.reg1_0
    }
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn reg3_2(&self) -> &Reg3_2 {
        &self.reg3_2
    }
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn reg5_4(&self) -> &Reg5_4 {
        &self.reg5_4
    }
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn reg7_6(&self) -> &Reg7_6 {
        &self.reg7_6
    }
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn loopaddr(&self) -> &Loopaddr {
        &self.loopaddr
    }
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn loopcnt(&self) -> &Loopcnt {
        &self.loopcnt
    }
    #[doc = "0x28 - Non-Secure DDI Access 0 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    #[inline(always)]
    pub const fn nonsecddiacc0(&self) -> &Nonsecddiacc0 {
        &self.nonsecddiacc0
    }
    #[doc = "0x2c - Non-Secure DDI Access 1 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    #[inline(always)]
    pub const fn nonsecddiacc1(&self) -> &Nonsecddiacc1 {
        &self.nonsecddiacc1
    }
    #[doc = "0x30 - Non-Secure DDI Access 2 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    #[inline(always)]
    pub const fn nonsecddiacc2(&self) -> &Nonsecddiacc2 {
        &self.nonsecddiacc2
    }
    #[doc = "0x34 - Non-Secure DDI Access 3 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    #[inline(always)]
    pub const fn nonsecddiacc3(&self) -> &Nonsecddiacc3 {
        &self.nonsecddiacc3
    }
}
#[doc = "CTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "FETCHSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fetchstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fetchstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fetchstat`]
module"]
#[doc(alias = "FETCHSTAT")]
pub type Fetchstat = crate::Reg<fetchstat::FetchstatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fetchstat;
#[doc = "CPUSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpustat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpustat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpustat`]
module"]
#[doc(alias = "CPUSTAT")]
pub type Cpustat = crate::Reg<cpustat::CpustatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpustat;
#[doc = "WUSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wustat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wustat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wustat`]
module"]
#[doc(alias = "WUSTAT")]
pub type Wustat = crate::Reg<wustat::WustatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod wustat;
#[doc = "REG1_0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1_0`]
module"]
#[doc(alias = "REG1_0")]
pub type Reg1_0 = crate::Reg<reg1_0::Reg1_0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg1_0;
#[doc = "REG3_2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg3_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg3_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg3_2`]
module"]
#[doc(alias = "REG3_2")]
pub type Reg3_2 = crate::Reg<reg3_2::Reg3_2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg3_2;
#[doc = "REG5_4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg5_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg5_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg5_4`]
module"]
#[doc(alias = "REG5_4")]
pub type Reg5_4 = crate::Reg<reg5_4::Reg5_4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg5_4;
#[doc = "REG7_6 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg7_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg7_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg7_6`]
module"]
#[doc(alias = "REG7_6")]
pub type Reg7_6 = crate::Reg<reg7_6::Reg7_6Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg7_6;
#[doc = "LOOPADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loopaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loopaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loopaddr`]
module"]
#[doc(alias = "LOOPADDR")]
pub type Loopaddr = crate::Reg<loopaddr::LoopaddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopaddr;
#[doc = "LOOPCNT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loopcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loopcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loopcnt`]
module"]
#[doc(alias = "LOOPCNT")]
pub type Loopcnt = crate::Reg<loopcnt::LoopcntSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopcnt;
#[doc = "NONSECDDIACC0 (rw) register accessor: Non-Secure DDI Access 0 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonsecddiacc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonsecddiacc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecddiacc0`]
module"]
#[doc(alias = "NONSECDDIACC0")]
pub type Nonsecddiacc0 = crate::Reg<nonsecddiacc0::Nonsecddiacc0Spec>;
#[doc = "Non-Secure DDI Access 0 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc0;
#[doc = "NONSECDDIACC1 (rw) register accessor: Non-Secure DDI Access 1 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonsecddiacc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonsecddiacc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecddiacc1`]
module"]
#[doc(alias = "NONSECDDIACC1")]
pub type Nonsecddiacc1 = crate::Reg<nonsecddiacc1::Nonsecddiacc1Spec>;
#[doc = "Non-Secure DDI Access 1 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc1;
#[doc = "NONSECDDIACC2 (rw) register accessor: Non-Secure DDI Access 2 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonsecddiacc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonsecddiacc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecddiacc2`]
module"]
#[doc(alias = "NONSECDDIACC2")]
pub type Nonsecddiacc2 = crate::Reg<nonsecddiacc2::Nonsecddiacc2Spec>;
#[doc = "Non-Secure DDI Access 2 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc2;
#[doc = "NONSECDDIACC3 (rw) register accessor: Non-Secure DDI Access 3 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonsecddiacc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonsecddiacc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonsecddiacc3`]
module"]
#[doc(alias = "NONSECDDIACC3")]
pub type Nonsecddiacc3 = crate::Reg<nonsecddiacc3::Nonsecddiacc3Spec>;
#[doc = "Non-Secure DDI Access 3 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc3;
