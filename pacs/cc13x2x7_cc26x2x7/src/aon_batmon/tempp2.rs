#[doc = "Register `TEMPP2` reader"]
pub struct R(crate::R<TEMPP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPP2` writer"]
pub struct W(crate::W<TEMPP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPP2_SPEC>;
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
impl From<crate::W<TEMPP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPP2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPP2_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<0> {
        CFG_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempp2](index.html) module"]
pub struct TEMPP2_SPEC;
impl crate::RegisterSpec for TEMPP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempp2::R](R) reader structure"]
impl crate::Readable for TEMPP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempp2::W](W) writer structure"]
impl crate::Writable for TEMPP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPP2 to value 0"]
impl crate::Resettable for TEMPP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}