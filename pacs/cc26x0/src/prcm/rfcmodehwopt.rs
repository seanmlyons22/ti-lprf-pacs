#[doc = "Register `RFCMODEHWOPT` reader"]
pub struct R(crate::R<RFCMODEHWOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCMODEHWOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCMODEHWOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCMODEHWOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCMODEHWOPT` writer"]
pub struct W(crate::W<RFCMODEHWOPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCMODEHWOPT_SPEC>;
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
impl From<crate::W<RFCMODEHWOPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCMODEHWOPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVAIL` reader - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
pub type AVAIL_R = crate::FieldReader<u8, AVAIL_A>;
#[doc = "7:0\\]
Permitted RFC modes. More than one mode can be permitted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AVAIL_A {
    #[doc = "128: Mode 7 permitted"]
    MODE7 = 128,
    #[doc = "64: Mode 6 permitted"]
    MODE6 = 64,
    #[doc = "32: Mode 5 permitted"]
    MODE5 = 32,
    #[doc = "16: Mode 4 permitted"]
    MODE4 = 16,
    #[doc = "8: Mode 3 permitted"]
    MODE3 = 8,
    #[doc = "4: Mode 2 permitted"]
    MODE2 = 4,
    #[doc = "2: Mode 1 permitted"]
    MODE1 = 2,
    #[doc = "1: Mode 0 permitted"]
    MODE0 = 1,
}
impl From<AVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: AVAIL_A) -> Self {
        variant as _
    }
}
impl AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AVAIL_A> {
        match self.bits {
            128 => Some(AVAIL_A::MODE7),
            64 => Some(AVAIL_A::MODE6),
            32 => Some(AVAIL_A::MODE5),
            16 => Some(AVAIL_A::MODE4),
            8 => Some(AVAIL_A::MODE3),
            4 => Some(AVAIL_A::MODE2),
            2 => Some(AVAIL_A::MODE1),
            1 => Some(AVAIL_A::MODE0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MODE7`"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == AVAIL_A::MODE7
    }
    #[doc = "Checks if the value of the field is `MODE6`"]
    #[inline(always)]
    pub fn is_mode6(&self) -> bool {
        *self == AVAIL_A::MODE6
    }
    #[doc = "Checks if the value of the field is `MODE5`"]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == AVAIL_A::MODE5
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == AVAIL_A::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == AVAIL_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == AVAIL_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == AVAIL_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == AVAIL_A::MODE0
    }
}
#[doc = "Field `AVAIL` writer - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
pub type AVAIL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCMODEHWOPT_SPEC, u8, AVAIL_A, 8, O>;
impl<'a, const O: u8> AVAIL_W<'a, O> {
    #[doc = "Mode 7 permitted"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE7)
    }
    #[doc = "Mode 6 permitted"]
    #[inline(always)]
    pub fn mode6(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE6)
    }
    #[doc = "Mode 5 permitted"]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE5)
    }
    #[doc = "Mode 4 permitted"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE4)
    }
    #[doc = "Mode 3 permitted"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE3)
    }
    #[doc = "Mode 2 permitted"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE2)
    }
    #[doc = "Mode 1 permitted"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE1)
    }
    #[doc = "Mode 0 permitted"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE0)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFCMODEHWOPT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    pub fn avail(&self) -> AVAIL_R {
        AVAIL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    #[must_use]
    pub fn avail(&mut self) -> AVAIL_W<0> {
        AVAIL_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allowed RFC Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcmodehwopt](index.html) module"]
pub struct RFCMODEHWOPT_SPEC;
impl crate::RegisterSpec for RFCMODEHWOPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcmodehwopt::R](R) reader structure"]
impl crate::Readable for RFCMODEHWOPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcmodehwopt::W](W) writer structure"]
impl crate::Writable for RFCMODEHWOPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCMODEHWOPT to value 0"]
impl crate::Resettable for RFCMODEHWOPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
