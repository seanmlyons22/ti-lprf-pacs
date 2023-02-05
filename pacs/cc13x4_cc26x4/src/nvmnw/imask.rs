#[doc = "Register `IMASK` reader"]
pub struct R(crate::R<IMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMASK` writer"]
pub struct W(crate::W<IMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMASK_SPEC>;
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
impl From<crate::W<IMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in IPSTANDARD.MIS will be set"]
    ENABLED = 1,
    #[doc = "0: Interrupt is masked out"]
    DISABLED = 0,
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
            true => DONE_A::ENABLED,
            false => DONE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONE_A::DISABLED
    }
}
#[doc = "Field `DONE` writer - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMASK_SPEC, DONE_A, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in IPSTANDARD.MIS will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DONE_A::ENABLED)
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DONE_A::DISABLED)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMASK_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
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
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
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
#[doc = "Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask](index.html) module"]
pub struct IMASK_SPEC;
impl crate::RegisterSpec for IMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imask::R](R) reader structure"]
impl crate::Readable for IMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imask::W](W) writer structure"]
impl crate::Writable for IMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for IMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
