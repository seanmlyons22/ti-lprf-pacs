#[doc = "Register `IOSTRP0` reader"]
pub struct R(crate::R<IOSTRP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOSTRP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOSTRP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOSTRP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOSTRP0` writer"]
pub struct W(crate::W<IOSTRP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOSTRP0_SPEC>;
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
impl From<crate::W<IOSTRP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOSTRP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG1` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG1` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOSTRP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG2` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG2` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type CFG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOSTRP0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOSTRP0_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> CFG1_W<0> {
        CFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> CFG2_W<4> {
        CFG2_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iostrp0](index.html) module"]
pub struct IOSTRP0_SPEC;
impl crate::RegisterSpec for IOSTRP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iostrp0::R](R) reader structure"]
impl crate::Readable for IOSTRP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iostrp0::W](W) writer structure"]
impl crate::Writable for IOSTRP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOSTRP0 to value 0x28"]
impl crate::Resettable for IOSTRP0_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
