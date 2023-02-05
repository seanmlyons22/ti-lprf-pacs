#[doc = "Register `TEMPLL` reader"]
pub struct R(crate::R<TEMPLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPLL` writer"]
pub struct W(crate::W<TEMPLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPLL_SPEC>;
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
impl From<crate::W<TEMPLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPLL_SPEC, u8, u8, 6, O>;
#[doc = "Field `FRAC` reader - 7:6\\]
Fractional part of temperature lower limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC` writer - 7:6\\]
Fractional part of temperature lower limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPLL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INT` reader - 16:8\\]
Integer part (signed) of temperature lower limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
pub type INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT` writer - 16:8\\]
Integer part (signed) of temperature lower limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPLL_SPEC, u16, u16, 9, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPLL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Fractional part of temperature lower limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Integer part (signed) of temperature lower limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Fractional part of temperature lower limit. Total value = INTEGER + FRACTIONAL The encoding is an extension of the 2's complement encoding. 00: 0.0C 01: 0.25C 10: 0.5C 11: 0.75C For example: 000000001,00 = ( 1+0,00) = 1,00 000000000,11 = ( 0+0,75) = 0,75 000000000,10 = ( 0+0,50) = 0,50 000000000,01 = ( 0+0,25) = 0,25 000000000,00 = ( 0+0,00) = 0,00 111111111,11 = (-1+0,75) = -0,25 111111111,10 = (-1+0,50) = -0,50 111111111,01 = (-1+0,25) = -0,75 111111111,00 = (-1+0,00) = -1,00 111111110,11 = (-2+0,75) = -1,25"]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<6> {
        FRAC_W::new(self)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Integer part (signed) of temperature lower limit. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<8> {
        INT_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature Lower Limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [templl](index.html) module"]
pub struct TEMPLL_SPEC;
impl crate::RegisterSpec for TEMPLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [templl::R](R) reader structure"]
impl crate::Readable for TEMPLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [templl::W](W) writer structure"]
impl crate::Writable for TEMPLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPLL to value 0x0001_0000"]
impl crate::Resettable for TEMPLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
