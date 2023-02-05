#[doc = "Register `ICLR` reader"]
pub struct R(crate::R<ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - 0:0\\]
0: No effect 1: Clear the DONE interrupt in the RIS register"]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "0:0\\]
0: No effect 1: Clear the DONE interrupt in the RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "1: Clear IPSTANDARD.RIS bit"]
    CLR = 1,
    #[doc = "0: Writing a 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            true => DONE_A::CLR,
            false => DONE_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == DONE_A::CLR
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DONE_A::NO_EFFECT
    }
}
#[doc = "Field `DONE` writer - 0:0\\]
0: No effect 1: Clear the DONE interrupt in the RIS register"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, DONE_A, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Clear IPSTANDARD.RIS bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(DONE_A::CLR)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DONE_A::NO_EFFECT)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICLR_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No effect 1: Clear the DONE interrupt in the RIS register"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
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
0: No effect 1: Clear the DONE interrupt in the RIS register"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
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
#[doc = "Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclr::R](R) reader structure"]
impl crate::Readable for ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
