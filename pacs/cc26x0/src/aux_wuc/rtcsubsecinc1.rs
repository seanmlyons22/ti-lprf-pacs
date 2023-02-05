#[doc = "Register `RTCSUBSECINC1` reader"]
pub struct R(crate::R<RTCSUBSECINC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSUBSECINC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSUBSECINC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSUBSECINC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSUBSECINC1` writer"]
pub struct W(crate::W<RTCSUBSECINC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSUBSECINC1_SPEC>;
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
impl From<crate::W<RTCSUBSECINC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSUBSECINC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INC23_16` reader - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
pub type INC23_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INC23_16` writer - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
pub type INC23_16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCSUBSECINC1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    pub fn inc23_16(&self) -> INC23_16_R {
        INC23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    #[must_use]
    pub fn inc23_16(&mut self) -> INC23_16_W<0> {
        INC23_16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc1](index.html) module"]
pub struct RTCSUBSECINC1_SPEC;
impl crate::RegisterSpec for RTCSUBSECINC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsubsecinc1::R](R) reader structure"]
impl crate::Readable for RTCSUBSECINC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc1::W](W) writer structure"]
impl crate::Writable for RTCSUBSECINC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSUBSECINC1 to value 0"]
impl crate::Resettable for RTCSUBSECINC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
