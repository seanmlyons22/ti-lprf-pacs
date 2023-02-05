#[doc = "Register `TRIGCNTLOAD` reader"]
pub struct R(crate::R<TRIGCNTLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGCNTLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGCNTLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGCNTLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGCNTLOAD` writer"]
pub struct W(crate::W<TRIGCNTLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGCNTLOAD_SPEC>;
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
impl From<crate::W<TRIGCNTLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGCNTLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIGCNTLOAD_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIGCNTLOAD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
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
#[doc = "Trigger Counter Load Stop-counter load.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigcntload](index.html) module"]
pub struct TRIGCNTLOAD_SPEC;
impl crate::RegisterSpec for TRIGCNTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigcntload::R](R) reader structure"]
impl crate::Readable for TRIGCNTLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigcntload::W](W) writer structure"]
impl crate::Writable for TRIGCNTLOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGCNTLOAD to value 0"]
impl crate::Resettable for TRIGCNTLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
