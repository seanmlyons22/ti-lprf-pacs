#[doc = "Register `RTCSUBSECINC0` reader"]
pub struct R(crate::R<RTCSUBSECINC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSUBSECINC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSUBSECINC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSUBSECINC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSUBSECINC0` writer"]
pub struct W(crate::W<RTCSUBSECINC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSUBSECINC0_SPEC>;
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
impl From<crate::W<RTCSUBSECINC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSUBSECINC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INC15_0` reader - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
pub type INC15_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INC15_0` writer - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
pub type INC15_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCSUBSECINC0_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCSUBSECINC0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc15_0(&self) -> INC15_0_R {
        INC15_0_R::new((self.bits & 0xffff) as u16)
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
New value for bits 15:0 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    #[must_use]
    pub fn inc15_0(&mut self) -> INC15_0_W<0> {
        INC15_0_W::new(self)
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
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc0](index.html) module"]
pub struct RTCSUBSECINC0_SPEC;
impl crate::RegisterSpec for RTCSUBSECINC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsubsecinc0::R](R) reader structure"]
impl crate::Readable for RTCSUBSECINC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc0::W](W) writer structure"]
impl crate::Writable for RTCSUBSECINC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSUBSECINC0 to value 0"]
impl crate::Resettable for RTCSUBSECINC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
