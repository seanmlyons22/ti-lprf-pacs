#[doc = "Register `MISC_TRIM` reader"]
pub struct R(crate::R<MISC_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_TRIM` writer"]
pub struct W(crate::W<MISC_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_TRIM_SPEC>;
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
impl From<crate::W<MISC_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMPVSLOPE` reader - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TEMPVSLOPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMPVSLOPE` writer - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TEMPVSLOPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISC_TRIM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&self) -> TEMPVSLOPE_R {
        TEMPVSLOPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    #[must_use]
    pub fn tempvslope(&mut self) -> TEMPVSLOPE_W<0> {
        TEMPVSLOPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Trim Parameters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_trim](index.html) module"]
pub struct MISC_TRIM_SPEC;
impl crate::RegisterSpec for MISC_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_trim::R](R) reader structure"]
impl crate::Readable for MISC_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_trim::W](W) writer structure"]
impl crate::Writable for MISC_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_TRIM to value 0xffff_ff33"]
impl crate::Resettable for MISC_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff33;
}
