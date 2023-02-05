#[doc = "Register `STMPINTRIG` reader"]
pub struct R(crate::R<STMPINTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMPINTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMPINTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMPINTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMPINTRIG` writer"]
pub struct W(crate::W<STMPINTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMPINTRIG_SPEC>;
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
impl From<crate::W<STMPINTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMPINTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_START_WCNT` reader - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
pub type IN_START_WCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IN_START_WCNT` writer - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
pub type IN_START_WCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STMPINTRIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STMPINTRIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub fn in_start_wcnt(&self) -> IN_START_WCNT_R {
        IN_START_WCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    #[must_use]
    pub fn in_start_wcnt(&mut self) -> IN_START_WCNT_W<0> {
        IN_START_WCNT_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCLK Counter Trigger Value for Input Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpintrig](index.html) module"]
pub struct STMPINTRIG_SPEC;
impl crate::RegisterSpec for STMPINTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmpintrig::R](R) reader structure"]
impl crate::Readable for STMPINTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmpintrig::W](W) writer structure"]
impl crate::Writable for STMPINTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STMPINTRIG to value 0"]
impl crate::Resettable for STMPINTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
