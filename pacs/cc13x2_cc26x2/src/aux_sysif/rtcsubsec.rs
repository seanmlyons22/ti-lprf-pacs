#[doc = "Register `RTCSUBSEC` reader"]
pub struct R(crate::R<RTCSUBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSUBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSUBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSUBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSUBSEC` writer"]
pub struct W(crate::W<RTCSUBSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSUBSEC_SPEC>;
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
impl From<crate::W<RTCSUBSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSUBSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSEC` reader - 15:0\\]
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
pub type SUBSEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUBSEC` writer - 15:0\\]
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
pub type SUBSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCSUBSEC_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCSUBSEC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0xffff) as u16)
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
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn subsec(&mut self) -> SUBSEC_W<0> {
        SUBSEC_W::new(self)
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
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsec](index.html) module"]
pub struct RTCSUBSEC_SPEC;
impl crate::RegisterSpec for RTCSUBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsubsec::R](R) reader structure"]
impl crate::Readable for RTCSUBSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsubsec::W](W) writer structure"]
impl crate::Writable for RTCSUBSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSUBSEC to value 0"]
impl crate::Resettable for RTCSUBSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
