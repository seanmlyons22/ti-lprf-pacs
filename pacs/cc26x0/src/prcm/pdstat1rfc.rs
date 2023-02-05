#[doc = "Register `PDSTAT1RFC` reader"]
pub struct R(crate::R<PDSTAT1RFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSTAT1RFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSTAT1RFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSTAT1RFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSTAT1RFC` writer"]
pub struct W(crate::W<PDSTAT1RFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSTAT1RFC_SPEC>;
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
impl From<crate::W<PDSTAT1RFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSTAT1RFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ON` reader - 0:0\\]
This is an alias for PDSTAT1.RFC_ON"]
pub type ON_R = crate::BitReader<bool>;
#[doc = "Field `ON` writer - 0:0\\]
This is an alias for PDSTAT1.RFC_ON"]
pub type ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT1RFC_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDSTAT1RFC_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is an alias for PDSTAT1.RFC_ON"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This is an alias for PDSTAT1.RFC_ON"]
    #[inline(always)]
    #[must_use]
    pub fn on(&mut self) -> ON_W<0> {
        ON_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RFC Power Domain Direct Read Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1rfc](index.html) module"]
pub struct PDSTAT1RFC_SPEC;
impl crate::RegisterSpec for PDSTAT1RFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdstat1rfc::R](R) reader structure"]
impl crate::Readable for PDSTAT1RFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdstat1rfc::W](W) writer structure"]
impl crate::Writable for PDSTAT1RFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSTAT1RFC to value 0"]
impl crate::Resettable for PDSTAT1RFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
