#[doc = "Register `FRDCTL` reader"]
pub struct R(crate::R<FRDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRDCTL` writer"]
pub struct W(crate::W<FRDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRDCTL_SPEC>;
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
impl From<crate::W<FRDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRDCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RWAIT` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWAIT` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRDCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRDCTL_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait(&self) -> RWAIT_R {
        RWAIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<0> {
        RM_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rwait(&mut self) -> RWAIT_W<8> {
        RWAIT_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frdctl](index.html) module"]
pub struct FRDCTL_SPEC;
impl crate::RegisterSpec for FRDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frdctl::R](R) reader structure"]
impl crate::Readable for FRDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frdctl::W](W) writer structure"]
impl crate::Writable for FRDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRDCTL to value 0x0200"]
impl crate::Resettable for FRDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
