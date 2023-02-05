#[doc = "Register `EFUSE` reader"]
pub struct R(crate::R<EFUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE` writer"]
pub struct W(crate::W<EFUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_SPEC>;
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
impl From<crate::W<EFUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUMPWORD` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type DUMPWORD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUMPWORD` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type DUMPWORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSE_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED16` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSE_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTRUCTION` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type INSTRUCTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTRUCTION` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type INSTRUCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSE_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dumpword(&self) -> DUMPWORD_R {
        DUMPWORD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dumpword(&mut self) -> DUMPWORD_W<0> {
        DUMPWORD_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<24> {
        INSTRUCTION_W::new(self)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> RESERVED29_W<29> {
        RESERVED29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse](index.html) module"]
pub struct EFUSE_SPEC;
impl crate::RegisterSpec for EFUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse::R](R) reader structure"]
impl crate::Readable for EFUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse::W](W) writer structure"]
impl crate::Writable for EFUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSE to value 0"]
impl crate::Resettable for EFUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
