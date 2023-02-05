#[doc = "Register `FLOCK` reader"]
pub struct R(crate::R<FLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOCK` writer"]
pub struct W(crate::W<FLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOCK_SPEC>;
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
impl From<crate::W<FLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCOM` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ENCOM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENCOM` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ENCOM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOCK_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOCK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn encom(&self) -> ENCOM_R {
        ENCOM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn encom(&mut self) -> ENCOM_W<0> {
        ENCOM_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flock](index.html) module"]
pub struct FLOCK_SPEC;
impl crate::RegisterSpec for FLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flock::R](R) reader structure"]
impl crate::Readable for FLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flock::W](W) writer structure"]
impl crate::Writable for FLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLOCK to value 0x55aa"]
impl crate::Resettable for FLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x55aa;
}
