#[doc = "Register `TRIGCNT` reader"]
pub struct R(crate::R<TRIGCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGCNT` writer"]
pub struct W(crate::W<TRIGCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGCNT_SPEC>;
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
impl From<crate::W<TRIGCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIGCNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIGCNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
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
#[doc = "Trigger Counter Stop-counter control and status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigcnt](index.html) module"]
pub struct TRIGCNT_SPEC;
impl crate::RegisterSpec for TRIGCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigcnt::R](R) reader structure"]
impl crate::Readable for TRIGCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigcnt::W](W) writer structure"]
impl crate::Writable for TRIGCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGCNT to value 0"]
impl crate::Resettable for TRIGCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
