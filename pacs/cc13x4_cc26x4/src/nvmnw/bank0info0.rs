#[doc = "Register `BANK0INFO0` reader"]
pub struct R(crate::R<BANK0INFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANK0INFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANK0INFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANK0INFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANK0INFO0` writer"]
pub struct W(crate::W<BANK0INFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANK0INFO0_SPEC>;
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
impl From<crate::W<BANK0INFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANK0INFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINSIZE` reader - 11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)"]
pub type MAINSIZE_R = crate::FieldReader<u16, MAINSIZE_A>;
#[doc = "11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MAINSIZE_A {
    #[doc = "512: Maximum value of MAINSIZE"]
    MAXSECTORS = 512,
    #[doc = "8: Minimum value of MAINSIZE"]
    MINSECTORS = 8,
}
impl From<MAINSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: MAINSIZE_A) -> Self {
        variant as _
    }
}
impl MAINSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAINSIZE_A> {
        match self.bits {
            512 => Some(MAINSIZE_A::MAXSECTORS),
            8 => Some(MAINSIZE_A::MINSECTORS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXSECTORS`"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == MAINSIZE_A::MAXSECTORS
    }
    #[doc = "Checks if the value of the field is `MINSECTORS`"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == MAINSIZE_A::MINSECTORS
    }
}
#[doc = "Field `MAINSIZE` writer - 11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)"]
pub type MAINSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK0INFO0_SPEC, u16, MAINSIZE_A, 12, O>;
impl<'a, const O: u8> MAINSIZE_W<'a, O> {
    #[doc = "Maximum value of MAINSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut W {
        self.variant(MAINSIZE_A::MAXSECTORS)
    }
    #[doc = "Minimum value of MAINSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut W {
        self.variant(MAINSIZE_A::MINSECTORS)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BANK0INFO0_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)"]
    #[inline(always)]
    pub fn mainsize(&self) -> MAINSIZE_R {
        MAINSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Main region size in sectors Minimum:0x8 (8) Maximum:0x200 (512)"]
    #[inline(always)]
    #[must_use]
    pub fn mainsize(&mut self) -> MAINSIZE_W<0> {
        MAINSIZE_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank0info0](index.html) module"]
pub struct BANK0INFO0_SPEC;
impl crate::RegisterSpec for BANK0INFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bank0info0::R](R) reader structure"]
impl crate::Readable for BANK0INFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bank0info0::W](W) writer structure"]
impl crate::Writable for BANK0INFO0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BANK0INFO0 to value 0x0100"]
impl crate::Resettable for BANK0INFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
