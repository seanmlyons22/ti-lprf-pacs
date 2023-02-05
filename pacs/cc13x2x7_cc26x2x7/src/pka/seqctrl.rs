#[doc = "Register `SEQCTRL` reader"]
pub struct R(crate::R<SEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCTRL` writer"]
pub struct W(crate::W<SEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_CONTROL_STAT` reader - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
pub type SW_CONTROL_STAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_CONTROL_STAT` writer - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
pub type SW_CONTROL_STAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `SEQUENCER_STAT` reader - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
pub type SEQUENCER_STAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQUENCER_STAT` writer - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
pub type SEQUENCER_STAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED16` reader - 30:16\\]
Set to zero on write, ignore on read"]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 30:16\\]
Set to zero on write, ignore on read"]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u16, u16, 15, O>;
#[doc = "Field `RESET` reader - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sw_control_stat(&self) -> SW_CONTROL_STAT_R {
        SW_CONTROL_STAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sequencer_stat(&self) -> SEQUENCER_STAT_R {
        SEQUENCER_STAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    #[must_use]
    pub fn sw_control_stat(&mut self) -> SW_CONTROL_STAT_W<0> {
        SW_CONTROL_STAT_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    #[must_use]
    pub fn sequencer_stat(&mut self) -> SEQUENCER_STAT_W<8> {
        SEQUENCER_STAT_W::new(self)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<31> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](index.html) module"]
pub struct SEQCTRL_SPEC;
impl crate::RegisterSpec for SEQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqctrl::R](R) reader structure"]
impl crate::Readable for SEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](W) writer structure"]
impl crate::Writable for SEQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL to value 0x0100"]
impl crate::Resettable for SEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
