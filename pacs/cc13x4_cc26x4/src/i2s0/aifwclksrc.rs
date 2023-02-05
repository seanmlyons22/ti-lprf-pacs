#[doc = "Register `AIFWCLKSRC` reader"]
pub struct R(crate::R<AIFWCLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFWCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFWCLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFWCLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFWCLKSRC` writer"]
pub struct W(crate::W<AIFWCLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFWCLKSRC_SPEC>;
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
impl From<crate::W<AIFWCLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFWCLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WCLK_SRC` reader - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
pub type WCLK_SRC_R = crate::FieldReader<u8, WCLK_SRC_A>;
#[doc = "1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WCLK_SRC_A {
    #[doc = "2: Internal WCLK generator, from module PRCM"]
    INT = 2,
    #[doc = "1: External WCLK generator, from pad"]
    EXT = 1,
    #[doc = "0: None ('0')"]
    NONE = 0,
}
impl From<WCLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: WCLK_SRC_A) -> Self {
        variant as _
    }
}
impl WCLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCLK_SRC_A {
        match self.bits {
            2 => WCLK_SRC_A::INT,
            1 => WCLK_SRC_A::EXT,
            0 => WCLK_SRC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WCLK_SRC_A::INT
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == WCLK_SRC_A::EXT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WCLK_SRC_A::NONE
    }
}
#[doc = "Field `WCLK_SRC` writer - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
pub type WCLK_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AIFWCLKSRC_SPEC, u8, WCLK_SRC_A, 2, O>;
impl<'a, const O: u8> WCLK_SRC_W<'a, O> {
    #[doc = "Internal WCLK generator, from module PRCM"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(WCLK_SRC_A::INT)
    }
    #[doc = "External WCLK generator, from pad"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(WCLK_SRC_A::EXT)
    }
    #[doc = "None ('0')"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WCLK_SRC_A::NONE)
    }
}
#[doc = "Field `WCLK_INV` reader - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
pub type WCLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `WCLK_INV` writer - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
pub type WCLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIFWCLKSRC_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AIFWCLKSRC_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline(always)]
    pub fn wclk_src(&self) -> WCLK_SRC_R {
        WCLK_SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline(always)]
    pub fn wclk_inv(&self) -> WCLK_INV_R {
        WCLK_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_src(&mut self) -> WCLK_SRC_W<0> {
        WCLK_SRC_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_inv(&mut self) -> WCLK_INV_W<2> {
        WCLK_INV_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCLK Source Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifwclksrc](index.html) module"]
pub struct AIFWCLKSRC_SPEC;
impl crate::RegisterSpec for AIFWCLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aifwclksrc::R](R) reader structure"]
impl crate::Readable for AIFWCLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aifwclksrc::W](W) writer structure"]
impl crate::Writable for AIFWCLKSRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIFWCLKSRC to value 0"]
impl crate::Resettable for AIFWCLKSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
