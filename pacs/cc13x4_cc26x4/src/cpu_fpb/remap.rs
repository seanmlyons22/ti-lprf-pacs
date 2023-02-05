#[doc = "Register `REMAP` reader"]
pub struct R(crate::R<REMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAP` writer"]
pub struct W(crate::W<REMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAP_SPEC>;
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
impl From<crate::W<REMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REMAP_SPEC, u8, u8, 5, O>;
#[doc = "Field `REMAP` reader - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
pub type REMAP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REMAP` writer - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
pub type REMAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REMAP_SPEC, u32, u32, 24, O>;
#[doc = "Field `RMPSPT` reader - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
pub type RMPSPT_R = crate::BitReader<bool>;
#[doc = "Field `RMPSPT` writer - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
pub type RMPSPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, REMAP_SPEC, bool, O>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REMAP_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new((self.bits >> 5) & 0x00ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
    #[inline(always)]
    pub fn rmpspt(&self) -> RMPSPT_R {
        RMPSPT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Holds the bits\\[28:5\\]
of the Flash Patch remap address"]
    #[inline(always)]
    #[must_use]
    pub fn remap(&mut self) -> REMAP_W<5> {
        REMAP_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Indicates whether the FPB unit supports the Flash Patch remap function"]
    #[inline(always)]
    #[must_use]
    pub fn rmpspt(&mut self) -> RMPSPT_W<29> {
        RMPSPT_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> RESERVED30_W<30> {
        RESERVED30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates whether the implementation supports Flash Patch remap and, if it does, holds the target address for remap\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remap](index.html) module"]
pub struct REMAP_SPEC;
impl crate::RegisterSpec for REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remap::R](R) reader structure"]
impl crate::Readable for REMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remap::W](W) writer structure"]
impl crate::Writable for REMAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for REMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
