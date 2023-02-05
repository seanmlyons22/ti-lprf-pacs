#[doc = "Register `PER_DBG` reader"]
pub struct R(crate::R<PER_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_DBG` writer"]
pub struct W(crate::W<PER_DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_DBG_SPEC>;
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
impl From<crate::W<PER_DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_DBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_DEBUG_ADDR` reader - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
pub type PER_DEBUG_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PER_DEBUG_ADDR` writer - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
pub type PER_DEBUG_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PER_DBG_SPEC, u32, u32, 24, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_DBG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
    #[inline(always)]
    pub fn per_debug_addr(&self) -> PER_DEBUG_ADDR_R {
        PER_DEBUG_ADDR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Debug Parity Error Address Offset When PER_CTL.PER_DEBUG is 1, this field is used to set a parity debug address offset. The address offset must be a word-aligned address. Writes within this address offset will force incorrect parity bits to be stored together with the data written. The following reads within this same address offset will thus result in parity errors to be generated."]
    #[inline(always)]
    #[must_use]
    pub fn per_debug_addr(&mut self) -> PER_DEBUG_ADDR_W<0> {
        PER_DEBUG_ADDR_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Debug Parity error check debug address setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dbg](index.html) module"]
pub struct PER_DBG_SPEC;
impl crate::RegisterSpec for PER_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_dbg::R](R) reader structure"]
impl crate::Readable for PER_DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_dbg::W](W) writer structure"]
impl crate::Writable for PER_DBG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER_DBG to value 0"]
impl crate::Resettable for PER_DBG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
