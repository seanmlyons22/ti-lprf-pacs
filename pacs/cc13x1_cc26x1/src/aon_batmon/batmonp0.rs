#[doc = "Register `BATMONP0` reader"]
pub struct R(crate::R<BATMONP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BATMONP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BATMONP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BATMONP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BATMONP0` writer"]
pub struct W(crate::W<BATMONP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BATMONP0_SPEC>;
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
impl From<crate::W<BATMONP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BATMONP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONP0_SPEC, u8, u8, 7, O>;
#[doc = "Field `RESERVED6` reader - 31:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BATMONP0_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<0> {
        CFG_W::new(self)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<7> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmonp0](index.html) module"]
pub struct BATMONP0_SPEC;
impl crate::RegisterSpec for BATMONP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [batmonp0::R](R) reader structure"]
impl crate::Readable for BATMONP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [batmonp0::W](W) writer structure"]
impl crate::Writable for BATMONP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BATMONP0 to value 0"]
impl crate::Resettable for BATMONP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
