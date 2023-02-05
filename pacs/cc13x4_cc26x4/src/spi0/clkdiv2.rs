#[doc = "Register `CLKDIV2` reader"]
pub struct R(crate::R<CLKDIV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV2` writer"]
pub struct W(crate::W<CLKDIV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV2_SPEC>;
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
impl From<crate::W<CLKDIV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - 2:0\\]
Selects divide ratio of module clock"]
pub type RATIO_R = crate::FieldReader<u8, RATIO_A>;
#[doc = "2:0\\]
Selects divide ratio of module clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "7: Divide clock source by 8"]
    DIV_BY_8 = 7,
    #[doc = "6: Divide clock source by 7"]
    DIV_BY_7 = 6,
    #[doc = "5: Divide clock source by 6"]
    DIV_BY_6 = 5,
    #[doc = "4: Divide clock source by 5"]
    DIV_BY_5 = 4,
    #[doc = "3: Divide clock source by 4"]
    DIV_BY_4 = 3,
    #[doc = "2: Divide clock source by 3"]
    DIV_BY_3 = 2,
    #[doc = "1: Divide clock source by 2"]
    DIV_BY_2 = 1,
    #[doc = "0: Do not divide clock source"]
    DIV_BY_1 = 0,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            7 => RATIO_A::DIV_BY_8,
            6 => RATIO_A::DIV_BY_7,
            5 => RATIO_A::DIV_BY_6,
            4 => RATIO_A::DIV_BY_5,
            3 => RATIO_A::DIV_BY_4,
            2 => RATIO_A::DIV_BY_3,
            1 => RATIO_A::DIV_BY_2,
            0 => RATIO_A::DIV_BY_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == RATIO_A::DIV_BY_8
    }
    #[doc = "Checks if the value of the field is `DIV_BY_7`"]
    #[inline(always)]
    pub fn is_div_by_7(&self) -> bool {
        *self == RATIO_A::DIV_BY_7
    }
    #[doc = "Checks if the value of the field is `DIV_BY_6`"]
    #[inline(always)]
    pub fn is_div_by_6(&self) -> bool {
        *self == RATIO_A::DIV_BY_6
    }
    #[doc = "Checks if the value of the field is `DIV_BY_5`"]
    #[inline(always)]
    pub fn is_div_by_5(&self) -> bool {
        *self == RATIO_A::DIV_BY_5
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == RATIO_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_3`"]
    #[inline(always)]
    pub fn is_div_by_3(&self) -> bool {
        *self == RATIO_A::DIV_BY_3
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == RATIO_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_1`"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == RATIO_A::DIV_BY_1
    }
}
#[doc = "Field `RATIO` writer - 2:0\\]
Selects divide ratio of module clock"]
pub type RATIO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLKDIV2_SPEC, u8, RATIO_A, 3, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_8)
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn div_by_7(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_7)
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn div_by_6(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_6)
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn div_by_5(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_5)
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_4)
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn div_by_3(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_3)
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_2)
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV_BY_1)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKDIV2_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects divide ratio of module clock"]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
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
#[doc = "This register is used to specify a divide ratio of the SPI functional clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv2](index.html) module"]
pub struct CLKDIV2_SPEC;
impl crate::RegisterSpec for CLKDIV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv2::R](R) reader structure"]
impl crate::Readable for CLKDIV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv2::W](W) writer structure"]
impl crate::Writable for CLKDIV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for CLKDIV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
