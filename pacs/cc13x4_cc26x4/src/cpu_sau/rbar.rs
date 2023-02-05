#[doc = "Register `RBAR` reader"]
pub struct R(crate::R<RBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBAR` writer"]
pub struct W(crate::W<RBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBAR_SPEC>;
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
impl From<crate::W<RBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BADDR` reader - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
pub type BADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BADDR` writer - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
pub type BADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBAR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
    #[inline(always)]
    pub fn baddr(&self) -> BADDR_R {
        BADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
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
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the base address for the selected SAU region"]
    #[inline(always)]
    #[must_use]
    pub fn baddr(&mut self) -> BADDR_W<5> {
        BADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides indirect read and write access to the base address of the currently selected SAU region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbar](index.html) module"]
pub struct RBAR_SPEC;
impl crate::RegisterSpec for RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbar::R](R) reader structure"]
impl crate::Readable for RBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbar::W](W) writer structure"]
impl crate::Writable for RBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBAR to value 0"]
impl crate::Resettable for RBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
