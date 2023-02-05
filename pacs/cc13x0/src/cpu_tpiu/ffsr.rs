#[doc = "Register `FFSR` reader"]
pub struct R(crate::R<FFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFSR` writer"]
pub struct W(crate::W<FFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFSR_SPEC>;
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
impl From<crate::W<FFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 2:0\\]
This field always reads as zero"]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 2:0\\]
This field always reads as zero"]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FTNONSTOP` reader - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
pub type FTNONSTOP_R = crate::BitReader<bool>;
#[doc = "Field `FTNONSTOP` writer - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
pub type FTNONSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFSR_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFSR_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This field always reads as zero"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
    #[inline(always)]
    pub fn ftnonstop(&self) -> FTNONSTOP_R {
        FTNONSTOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This field always reads as zero"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Formatter can be stopped 1: Formatter cannot be stopped"]
    #[inline(always)]
    #[must_use]
    pub fn ftnonstop(&mut self) -> FTNONSTOP_W<3> {
        FTNONSTOP_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Formatter and Flush Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffsr](index.html) module"]
pub struct FFSR_SPEC;
impl crate::RegisterSpec for FFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffsr::R](R) reader structure"]
impl crate::Readable for FFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffsr::W](W) writer structure"]
impl crate::Writable for FFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFSR to value 0x08"]
impl crate::Resettable for FFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
