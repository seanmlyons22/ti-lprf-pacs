#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x28],
    imask0: Imask0,
    _reserved1: [u8; 0x04],
    ris0: Ris0,
    _reserved2: [u8; 0x04],
    mis0: Mis0,
    _reserved3: [u8; 0x04],
    iset0: Iset0,
    _reserved4: [u8; 0x04],
    iclr0: Iclr0,
    _reserved5: [u8; 0x0c],
    imask1: Imask1,
    _reserved6: [u8; 0x04],
    ris1: Ris1,
    _reserved7: [u8; 0x04],
    mis1: Mis1,
    _reserved8: [u8; 0x04],
    iset1: Iset1,
    _reserved9: [u8; 0x04],
    iclr1: Iclr1,
    _reserved10: [u8; 0x0c],
    imask2: Imask2,
    _reserved11: [u8; 0x04],
    ris2: Ris2,
    _reserved12: [u8; 0x04],
    mis2: Mis2,
    _reserved13: [u8; 0x04],
    iset2: Iset2,
    _reserved14: [u8; 0x04],
    iclr2: Iclr2,
    _reserved15: [u8; 0x54],
    ctl0: Ctl0,
    ctl1: Ctl1,
    ctl2: Ctl2,
    ctl3: Ctl3,
    _reserved19: [u8; 0x04],
    scomp0: Scomp0,
    scomp1: Scomp1,
    refcfg: Refcfg,
    _reserved22: [u8; 0x28],
    wclow: Wclow,
    _reserved23: [u8; 0x04],
    wchigh: Wchigh,
    _reserved24: [u8; 0x0c],
    fifodata: Fifodata,
    _reserved25: [u8; 0x0c],
    ascres: Ascres,
    _reserved26: [u8; 0x0c],
    memctl0: Memctl0,
    memctl1: Memctl1,
    memctl2: Memctl2,
    memctl3: Memctl3,
    _reserved30: [u8; 0xf0],
    memres0: Memres0,
    memres1: Memres1,
    memres2: Memres2,
    memres3: Memres3,
    _reserved34: [u8; 0xb0],
    status: Status,
    _reserved35: [u8; 0x0abc],
    test0: Test0,
    test1: Test1,
    test2: Test2,
    test3: Test3,
    test4: Test4,
    test5: Test5,
    test6: Test6,
    _reserved42: [u8; 0x04],
    debug1: Debug1,
    debug2: Debug2,
    debug3: Debug3,
    debug4: Debug4,
}
impl RegisterBlock {
    #[doc = "0x28 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS0 to MIS0 when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask0(&self) -> &Imask0 {
        &self.imask0
    }
    #[doc = "0x30 - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR0 register bit."]
    #[inline(always)]
    pub const fn ris0(&self) -> &Ris0 {
        &self.ris0
    }
    #[doc = "0x38 - Masked interrupt status. This is an AND of the IMASK and RIS registers."]
    #[inline(always)]
    pub const fn mis0(&self) -> &Mis0 {
        &self.mis0
    }
    #[doc = "0x40 - Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET0 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
    #[inline(always)]
    pub const fn iset0(&self) -> &Iset0 {
        &self.iset0
    }
    #[doc = "0x48 - Interrupt clear. Write a 1 to clear corresponding Interrupt."]
    #[inline(always)]
    pub const fn iclr0(&self) -> &Iclr0 {
        &self.iclr0
    }
    #[doc = "0x58 - Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
    #[inline(always)]
    pub const fn imask1(&self) -> &Imask1 {
        &self.imask1
    }
    #[doc = "0x60 - Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS1 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
    #[inline(always)]
    pub const fn ris1(&self) -> &Ris1 {
        &self.ris1
    }
    #[doc = "0x68 - Masked interrupt status. This is an AND of the IMASK and RIS registers."]
    #[inline(always)]
    pub const fn mis1(&self) -> &Mis1 {
        &self.mis1
    }
    #[doc = "0x70 - Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET1 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
    #[inline(always)]
    pub const fn iset1(&self) -> &Iset1 {
        &self.iset1
    }
    #[doc = "0x78 - Interrupt clear. Write a 1 to clear corresponding Interrupt."]
    #[inline(always)]
    pub const fn iclr1(&self) -> &Iclr1 {
        &self.iclr1
    }
    #[doc = "0x88 - Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
    #[inline(always)]
    pub const fn imask2(&self) -> &Imask2 {
        &self.imask2
    }
    #[doc = "0x90 - Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS2 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
    #[inline(always)]
    pub const fn ris2(&self) -> &Ris2 {
        &self.ris2
    }
    #[doc = "0x98 - Extension of Masked interrupt status. This is an AND of the IMASK and RIS registers."]
    #[inline(always)]
    pub const fn mis2(&self) -> &Mis2 {
        &self.mis2
    }
    #[doc = "0xa0 - Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET2 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
    #[inline(always)]
    pub const fn iset2(&self) -> &Iset2 {
        &self.iset2
    }
    #[doc = "0xa8 - Interrupt clear. Write a 1 to clear corresponding Interrupt."]
    #[inline(always)]
    pub const fn iclr2(&self) -> &Iclr2 {
        &self.iclr2
    }
    #[doc = "0x100 - Control Register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x104 - Control Register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x108 - Control Register 2"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0x10c - Control Register 3. This register is used to configure ADC for ad-hoc single conversion."]
    #[inline(always)]
    pub const fn ctl3(&self) -> &Ctl3 {
        &self.ctl3
    }
    #[doc = "0x114 - Sample time compare 0 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register."]
    #[inline(always)]
    pub const fn scomp0(&self) -> &Scomp0 {
        &self.scomp0
    }
    #[doc = "0x118 - Sample time compare 1 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register."]
    #[inline(always)]
    pub const fn scomp1(&self) -> &Scomp1 {
        &self.scomp1
    }
    #[doc = "0x11c - Reference buffer configuration register"]
    #[inline(always)]
    pub const fn refcfg(&self) -> &Refcfg {
        &self.refcfg
    }
    #[doc = "0x148 - Window Comparator Low Threshold Register. The data format that is used to write and read WCLOW depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCLOW."]
    #[inline(always)]
    pub const fn wclow(&self) -> &Wclow {
        &self.wclow
    }
    #[doc = "0x150 - Window Comparator High Threshold Register. The data format that is used to write and read WCHIGH depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCHIGH."]
    #[inline(always)]
    pub const fn wchigh(&self) -> &Wchigh {
        &self.wchigh
    }
    #[doc = "0x160 - FIFO data register. This is a virtual register used to do read from FIFO."]
    #[inline(always)]
    pub const fn fifodata(&self) -> &Fifodata {
        &self.fifodata
    }
    #[doc = "0x170 - ASC result register"]
    #[inline(always)]
    pub const fn ascres(&self) -> &Ascres {
        &self.ascres
    }
    #[doc = "0x180 - Conversion Memory Control Register 0. CTL0.ENC must be set to 0 to write to this register."]
    #[inline(always)]
    pub const fn memctl0(&self) -> &Memctl0 {
        &self.memctl0
    }
    #[doc = "0x184 - Conversion Memory Control Register 1. CTL0.ENC must be set to 0 to write to this register."]
    #[inline(always)]
    pub const fn memctl1(&self) -> &Memctl1 {
        &self.memctl1
    }
    #[doc = "0x188 - Conversion Memory Control Register 2. CTL0.ENC must be set to 0 to write to this register."]
    #[inline(always)]
    pub const fn memctl2(&self) -> &Memctl2 {
        &self.memctl2
    }
    #[doc = "0x18c - Conversion Memory Control Register 3. CTL0.ENC must be set to 0 to write to this register."]
    #[inline(always)]
    pub const fn memctl3(&self) -> &Memctl3 {
        &self.memctl3
    }
    #[doc = "0x280 - Memory Result Register 0"]
    #[inline(always)]
    pub const fn memres0(&self) -> &Memres0 {
        &self.memres0
    }
    #[doc = "0x284 - Memory Result Register 1"]
    #[inline(always)]
    pub const fn memres1(&self) -> &Memres1 {
        &self.memres1
    }
    #[doc = "0x288 - Memory Result Register 2"]
    #[inline(always)]
    pub const fn memres2(&self) -> &Memres2 {
        &self.memres2
    }
    #[doc = "0x28c - Memory Result Register 3"]
    #[inline(always)]
    pub const fn memres3(&self) -> &Memres3 {
        &self.memres3
    }
    #[doc = "0x340 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0xe00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn test0(&self) -> &Test0 {
        &self.test0
    }
    #[doc = "0xe04 - Test 1 register. This is used to select ADC internal signals on DTB."]
    #[inline(always)]
    pub const fn test1(&self) -> &Test1 {
        &self.test1
    }
    #[doc = "0xe08 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn test2(&self) -> &Test2 {
        &self.test2
    }
    #[doc = "0xe0c - ADC CAL Accumulation Register"]
    #[inline(always)]
    pub const fn test3(&self) -> &Test3 {
        &self.test3
    }
    #[doc = "0xe10 - CAL Control register: Average Sample count, Step number, Recal En and Debug option to override ull_usc_ulpadchp_dft_i$lt;26:0$gt;."]
    #[inline(always)]
    pub const fn test4(&self) -> &Test4 {
        &self.test4
    }
    #[doc = "0xe14 - This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enable"]
    #[inline(always)]
    pub const fn test5(&self) -> &Test5 {
        &self.test5
    }
    #[doc = "0xe18 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn test6(&self) -> &Test6 {
        &self.test6
    }
    #[doc = "0xe20 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn debug1(&self) -> &Debug1 {
        &self.debug1
    }
    #[doc = "0xe24 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn debug2(&self) -> &Debug2 {
        &self.debug2
    }
    #[doc = "0xe28 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn debug3(&self) -> &Debug3 {
        &self.debug3
    }
    #[doc = "0xe2c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn debug4(&self) -> &Debug4 {
        &self.debug4
    }
}
#[doc = "IMASK0 (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS0 to MIS0 when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask0`]
module"]
#[doc(alias = "IMASK0")]
pub type Imask0 = crate::Reg<imask0::Imask0Spec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS0 to MIS0 when the corresponding bit-fields are set to 1."]
pub mod imask0;
#[doc = "RIS0 (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR0 register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris0`]
module"]
#[doc(alias = "RIS0")]
pub type Ris0 = crate::Reg<ris0::Ris0Spec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR0 register bit."]
pub mod ris0;
#[doc = "MIS0 (rw) register accessor: Masked interrupt status. This is an AND of the IMASK and RIS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis0`]
module"]
#[doc(alias = "MIS0")]
pub type Mis0 = crate::Reg<mis0::Mis0Spec>;
#[doc = "Masked interrupt status. This is an AND of the IMASK and RIS registers."]
pub mod mis0;
#[doc = "ISET0 (rw) register accessor: Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET0 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset0`]
module"]
#[doc(alias = "ISET0")]
pub type Iset0 = crate::Reg<iset0::Iset0Spec>;
#[doc = "Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET0 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
pub mod iset0;
#[doc = "ICLR0 (rw) register accessor: Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr0`]
module"]
#[doc(alias = "ICLR0")]
pub type Iclr0 = crate::Reg<iclr0::Iclr0Spec>;
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt."]
pub mod iclr0;
#[doc = "IMASK1 (rw) register accessor: Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask1`]
module"]
#[doc(alias = "IMASK1")]
pub type Imask1 = crate::Reg<imask1::Imask1Spec>;
#[doc = "Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
pub mod imask1;
#[doc = "RIS1 (rw) register accessor: Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS1 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris1`]
module"]
#[doc(alias = "RIS1")]
pub type Ris1 = crate::Reg<ris1::Ris1Spec>;
#[doc = "Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS1 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
pub mod ris1;
#[doc = "MIS1 (rw) register accessor: Masked interrupt status. This is an AND of the IMASK and RIS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis1`]
module"]
#[doc(alias = "MIS1")]
pub type Mis1 = crate::Reg<mis1::Mis1Spec>;
#[doc = "Masked interrupt status. This is an AND of the IMASK and RIS registers."]
pub mod mis1;
#[doc = "ISET1 (rw) register accessor: Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET1 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset1`]
module"]
#[doc(alias = "ISET1")]
pub type Iset1 = crate::Reg<iset1::Iset1Spec>;
#[doc = "Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET1 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
pub mod iset1;
#[doc = "ICLR1 (rw) register accessor: Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr1`]
module"]
#[doc(alias = "ICLR1")]
pub type Iclr1 = crate::Reg<iclr1::Iclr1Spec>;
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt."]
pub mod iclr1;
#[doc = "IMASK2 (rw) register accessor: Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask2`]
module"]
#[doc(alias = "IMASK2")]
pub type Imask2 = crate::Reg<imask2::Imask2Spec>;
#[doc = "Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
pub mod imask2;
#[doc = "RIS2 (rw) register accessor: Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS2 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris2`]
module"]
#[doc(alias = "RIS2")]
pub type Ris2 = crate::Reg<ris2::Ris2Spec>;
#[doc = "Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS2 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
pub mod ris2;
#[doc = "MIS2 (rw) register accessor: Extension of Masked interrupt status. This is an AND of the IMASK and RIS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis2`]
module"]
#[doc(alias = "MIS2")]
pub type Mis2 = crate::Reg<mis2::Mis2Spec>;
#[doc = "Extension of Masked interrupt status. This is an AND of the IMASK and RIS registers."]
pub mod mis2;
#[doc = "ISET2 (rw) register accessor: Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET2 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset2`]
module"]
#[doc(alias = "ISET2")]
pub type Iset2 = crate::Reg<iset2::Iset2Spec>;
#[doc = "Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET2 will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
pub mod iset2;
#[doc = "ICLR2 (rw) register accessor: Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr2`]
module"]
#[doc(alias = "ICLR2")]
pub type Iclr2 = crate::Reg<iclr2::Iclr2Spec>;
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt."]
pub mod iclr2;
#[doc = "CTL0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control Register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control Register 1"]
pub mod ctl1;
#[doc = "CTL2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`]
module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "Control Register 2"]
pub mod ctl2;
#[doc = "CTL3 (rw) register accessor: Control Register 3. This register is used to configure ADC for ad-hoc single conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl3`]
module"]
#[doc(alias = "CTL3")]
pub type Ctl3 = crate::Reg<ctl3::Ctl3Spec>;
#[doc = "Control Register 3. This register is used to configure ADC for ad-hoc single conversion."]
pub mod ctl3;
#[doc = "SCOMP0 (rw) register accessor: Sample time compare 0 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scomp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scomp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scomp0`]
module"]
#[doc(alias = "SCOMP0")]
pub type Scomp0 = crate::Reg<scomp0::Scomp0Spec>;
#[doc = "Sample time compare 0 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register."]
pub mod scomp0;
#[doc = "SCOMP1 (rw) register accessor: Sample time compare 1 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scomp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scomp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scomp1`]
module"]
#[doc(alias = "SCOMP1")]
pub type Scomp1 = crate::Reg<scomp1::Scomp1Spec>;
#[doc = "Sample time compare 1 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register."]
pub mod scomp1;
#[doc = "REFCFG (rw) register accessor: Reference buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refcfg`]
module"]
#[doc(alias = "REFCFG")]
pub type Refcfg = crate::Reg<refcfg::RefcfgSpec>;
#[doc = "Reference buffer configuration register"]
pub mod refcfg;
#[doc = "WCLOW (rw) register accessor: Window Comparator Low Threshold Register. The data format that is used to write and read WCLOW depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCLOW.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wclow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wclow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wclow`]
module"]
#[doc(alias = "WCLOW")]
pub type Wclow = crate::Reg<wclow::WclowSpec>;
#[doc = "Window Comparator Low Threshold Register. The data format that is used to write and read WCLOW depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCLOW."]
pub mod wclow;
#[doc = "WCHIGH (rw) register accessor: Window Comparator High Threshold Register. The data format that is used to write and read WCHIGH depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCHIGH.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wchigh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wchigh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wchigh`]
module"]
#[doc(alias = "WCHIGH")]
pub type Wchigh = crate::Reg<wchigh::WchighSpec>;
#[doc = "Window Comparator High Threshold Register. The data format that is used to write and read WCHIGH depends on the value of DF bit in CTL2 register. CTL0.ENC must be 0 to write to this register. Note: Change in ADC data format or resolution does not reset WCHIGH."]
pub mod wchigh;
#[doc = "FIFODATA (rw) register accessor: FIFO data register. This is a virtual register used to do read from FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifodata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifodata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodata`]
module"]
#[doc(alias = "FIFODATA")]
pub type Fifodata = crate::Reg<fifodata::FifodataSpec>;
#[doc = "FIFO data register. This is a virtual register used to do read from FIFO."]
pub mod fifodata;
#[doc = "ASCRES (rw) register accessor: ASC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ascres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ascres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ascres`]
module"]
#[doc(alias = "ASCRES")]
pub type Ascres = crate::Reg<ascres::AscresSpec>;
#[doc = "ASC result register"]
pub mod ascres;
#[doc = "MEMCTL0 (rw) register accessor: Conversion Memory Control Register 0. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctl0`]
module"]
#[doc(alias = "MEMCTL0")]
pub type Memctl0 = crate::Reg<memctl0::Memctl0Spec>;
#[doc = "Conversion Memory Control Register 0. CTL0.ENC must be set to 0 to write to this register."]
pub mod memctl0;
#[doc = "MEMCTL1 (rw) register accessor: Conversion Memory Control Register 1. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctl1`]
module"]
#[doc(alias = "MEMCTL1")]
pub type Memctl1 = crate::Reg<memctl1::Memctl1Spec>;
#[doc = "Conversion Memory Control Register 1. CTL0.ENC must be set to 0 to write to this register."]
pub mod memctl1;
#[doc = "MEMCTL2 (rw) register accessor: Conversion Memory Control Register 2. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctl2`]
module"]
#[doc(alias = "MEMCTL2")]
pub type Memctl2 = crate::Reg<memctl2::Memctl2Spec>;
#[doc = "Conversion Memory Control Register 2. CTL0.ENC must be set to 0 to write to this register."]
pub mod memctl2;
#[doc = "MEMCTL3 (rw) register accessor: Conversion Memory Control Register 3. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctl3`]
module"]
#[doc(alias = "MEMCTL3")]
pub type Memctl3 = crate::Reg<memctl3::Memctl3Spec>;
#[doc = "Conversion Memory Control Register 3. CTL0.ENC must be set to 0 to write to this register."]
pub mod memctl3;
#[doc = "MEMRES0 (rw) register accessor: Memory Result Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memres0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memres0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memres0`]
module"]
#[doc(alias = "MEMRES0")]
pub type Memres0 = crate::Reg<memres0::Memres0Spec>;
#[doc = "Memory Result Register 0"]
pub mod memres0;
#[doc = "MEMRES1 (rw) register accessor: Memory Result Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memres1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memres1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memres1`]
module"]
#[doc(alias = "MEMRES1")]
pub type Memres1 = crate::Reg<memres1::Memres1Spec>;
#[doc = "Memory Result Register 1"]
pub mod memres1;
#[doc = "MEMRES2 (rw) register accessor: Memory Result Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memres2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memres2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memres2`]
module"]
#[doc(alias = "MEMRES2")]
pub type Memres2 = crate::Reg<memres2::Memres2Spec>;
#[doc = "Memory Result Register 2"]
pub mod memres2;
#[doc = "MEMRES3 (rw) register accessor: Memory Result Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memres3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memres3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memres3`]
module"]
#[doc(alias = "MEMRES3")]
pub type Memres3 = crate::Reg<memres3::Memres3Spec>;
#[doc = "Memory Result Register 3"]
pub mod memres3;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "TEST0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test0`]
module"]
#[doc(alias = "TEST0")]
pub type Test0 = crate::Reg<test0::Test0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod test0;
#[doc = "TEST1 (rw) register accessor: Test 1 register. This is used to select ADC internal signals on DTB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test1`]
module"]
#[doc(alias = "TEST1")]
pub type Test1 = crate::Reg<test1::Test1Spec>;
#[doc = "Test 1 register. This is used to select ADC internal signals on DTB."]
pub mod test1;
#[doc = "TEST2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test2`]
module"]
#[doc(alias = "TEST2")]
pub type Test2 = crate::Reg<test2::Test2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod test2;
#[doc = "TEST3 (rw) register accessor: ADC CAL Accumulation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test3`]
module"]
#[doc(alias = "TEST3")]
pub type Test3 = crate::Reg<test3::Test3Spec>;
#[doc = "ADC CAL Accumulation Register"]
pub mod test3;
#[doc = "TEST4 (rw) register accessor: CAL Control register: Average Sample count, Step number, Recal En and Debug option to override ull_usc_ulpadchp_dft_i$lt;26:0$gt;.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test4`]
module"]
#[doc(alias = "TEST4")]
pub type Test4 = crate::Reg<test4::Test4Spec>;
#[doc = "CAL Control register: Average Sample count, Step number, Recal En and Debug option to override ull_usc_ulpadchp_dft_i$lt;26:0$gt;."]
pub mod test4;
#[doc = "TEST5 (rw) register accessor: This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test5`]
module"]
#[doc(alias = "TEST5")]
pub type Test5 = crate::Reg<test5::Test5Spec>;
#[doc = "This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enable"]
pub mod test5;
#[doc = "TEST6 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test6`]
module"]
#[doc(alias = "TEST6")]
pub type Test6 = crate::Reg<test6::Test6Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod test6;
#[doc = "DEBUG1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug1`]
module"]
#[doc(alias = "DEBUG1")]
pub type Debug1 = crate::Reg<debug1::Debug1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod debug1;
#[doc = "DEBUG2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug2`]
module"]
#[doc(alias = "DEBUG2")]
pub type Debug2 = crate::Reg<debug2::Debug2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod debug2;
#[doc = "DEBUG3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug3`]
module"]
#[doc(alias = "DEBUG3")]
pub type Debug3 = crate::Reg<debug3::Debug3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod debug3;
#[doc = "DEBUG4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug4`]
module"]
#[doc(alias = "DEBUG4")]
pub type Debug4 = crate::Reg<debug4::Debug4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod debug4;
