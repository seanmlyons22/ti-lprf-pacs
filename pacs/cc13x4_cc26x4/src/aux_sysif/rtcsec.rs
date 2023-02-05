#[doc = "Register `RTCSEC` reader"]
pub struct R(crate::R<RTCSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSEC` writer"]
pub struct W(crate::W<RTCSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSEC_SPEC>;
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
impl From<crate::W<RTCSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 15:0\\]
Bits 15:0 in AON_RTC:SEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads of SEC. - Then read SEC until two consecutive reads are equal."]
pub type SEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEC` writer - 15:0\\]
Bits 15:0 in AON_RTC:SEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads of SEC. - Then read SEC until two consecutive reads are equal."]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCSEC_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCSEC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Bits 15:0 in AON_RTC:SEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads of SEC. - Then read SEC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff) as u16)
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
Bits 15:0 in AON_RTC:SEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads of SEC. - Then read SEC until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
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
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsec](index.html) module"]
pub struct RTCSEC_SPEC;
impl crate::RegisterSpec for RTCSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsec::R](R) reader structure"]
impl crate::Readable for RTCSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsec::W](W) writer structure"]
impl crate::Writable for RTCSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSEC to value 0"]
impl crate::Resettable for RTCSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
