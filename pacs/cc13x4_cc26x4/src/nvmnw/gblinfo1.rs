#[doc = "Register `GBLINFO1` reader"]
pub struct R(crate::R<GBLINFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GBLINFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GBLINFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GBLINFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GBLINFO1` writer"]
pub struct W(crate::W<GBLINFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GBLINFO1_SPEC>;
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
impl From<crate::W<GBLINFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GBLINFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAWIDTH` reader - 7:0\\]
Data width in bits"]
pub type DATAWIDTH_R = crate::FieldReader<u8, DATAWIDTH_A>;
#[doc = "7:0\\]
Data width in bits\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAWIDTH_A {
    #[doc = "128: Data width is 128 bits"]
    W128BIT = 128,
    #[doc = "64: Data width is 64 bits"]
    W64BIT = 64,
}
impl From<DATAWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAWIDTH_A) -> Self {
        variant as _
    }
}
impl DATAWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAWIDTH_A> {
        match self.bits {
            128 => Some(DATAWIDTH_A::W128BIT),
            64 => Some(DATAWIDTH_A::W64BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `W128BIT`"]
    #[inline(always)]
    pub fn is_w128bit(&self) -> bool {
        *self == DATAWIDTH_A::W128BIT
    }
    #[doc = "Checks if the value of the field is `W64BIT`"]
    #[inline(always)]
    pub fn is_w64bit(&self) -> bool {
        *self == DATAWIDTH_A::W64BIT
    }
}
#[doc = "Field `DATAWIDTH` writer - 7:0\\]
Data width in bits"]
pub type DATAWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO1_SPEC, u8, DATAWIDTH_A, 8, O>;
impl<'a, const O: u8> DATAWIDTH_W<'a, O> {
    #[doc = "Data width is 128 bits"]
    #[inline(always)]
    pub fn w128bit(self) -> &'a mut W {
        self.variant(DATAWIDTH_A::W128BIT)
    }
    #[doc = "Data width is 64 bits"]
    #[inline(always)]
    pub fn w64bit(self) -> &'a mut W {
        self.variant(DATAWIDTH_A::W64BIT)
    }
}
#[doc = "Field `ECCWIDTH` reader - 12:8\\]
ECC data width in bits"]
pub type ECCWIDTH_R = crate::FieldReader<u8, ECCWIDTH_A>;
#[doc = "12:8\\]
ECC data width in bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCWIDTH_A {
    #[doc = "16: ECC data width is 16 bits"]
    W16BIT = 16,
    #[doc = "8: ECC data width is 8 bits"]
    W8BIT = 8,
    #[doc = "0: ECC data width is 0. ECC not used."]
    W0BIT = 0,
}
impl From<ECCWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCWIDTH_A) -> Self {
        variant as _
    }
}
impl ECCWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECCWIDTH_A> {
        match self.bits {
            16 => Some(ECCWIDTH_A::W16BIT),
            8 => Some(ECCWIDTH_A::W8BIT),
            0 => Some(ECCWIDTH_A::W0BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `W16BIT`"]
    #[inline(always)]
    pub fn is_w16bit(&self) -> bool {
        *self == ECCWIDTH_A::W16BIT
    }
    #[doc = "Checks if the value of the field is `W8BIT`"]
    #[inline(always)]
    pub fn is_w8bit(&self) -> bool {
        *self == ECCWIDTH_A::W8BIT
    }
    #[doc = "Checks if the value of the field is `W0BIT`"]
    #[inline(always)]
    pub fn is_w0bit(&self) -> bool {
        *self == ECCWIDTH_A::W0BIT
    }
}
#[doc = "Field `ECCWIDTH` writer - 12:8\\]
ECC data width in bits"]
pub type ECCWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO1_SPEC, u8, ECCWIDTH_A, 5, O>;
impl<'a, const O: u8> ECCWIDTH_W<'a, O> {
    #[doc = "ECC data width is 16 bits"]
    #[inline(always)]
    pub fn w16bit(self) -> &'a mut W {
        self.variant(ECCWIDTH_A::W16BIT)
    }
    #[doc = "ECC data width is 8 bits"]
    #[inline(always)]
    pub fn w8bit(self) -> &'a mut W {
        self.variant(ECCWIDTH_A::W8BIT)
    }
    #[doc = "ECC data width is 0. ECC not used."]
    #[inline(always)]
    pub fn w0bit(self) -> &'a mut W {
        self.variant(ECCWIDTH_A::W0BIT)
    }
}
#[doc = "Field `REDWIDTH` reader - 18:16\\]
Redundant data width in bits"]
pub type REDWIDTH_R = crate::FieldReader<u8, REDWIDTH_A>;
#[doc = "18:16\\]
Redundant data width in bits\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REDWIDTH_A {
    #[doc = "4: Redundant data width is 4 bits"]
    W4BIT = 4,
    #[doc = "2: Redundant data width is 2 bits"]
    W2BIT = 2,
    #[doc = "0: Redundant data width is 0. Redundancy/Repair not present."]
    W0BIT = 0,
}
impl From<REDWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: REDWIDTH_A) -> Self {
        variant as _
    }
}
impl REDWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REDWIDTH_A> {
        match self.bits {
            4 => Some(REDWIDTH_A::W4BIT),
            2 => Some(REDWIDTH_A::W2BIT),
            0 => Some(REDWIDTH_A::W0BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `W4BIT`"]
    #[inline(always)]
    pub fn is_w4bit(&self) -> bool {
        *self == REDWIDTH_A::W4BIT
    }
    #[doc = "Checks if the value of the field is `W2BIT`"]
    #[inline(always)]
    pub fn is_w2bit(&self) -> bool {
        *self == REDWIDTH_A::W2BIT
    }
    #[doc = "Checks if the value of the field is `W0BIT`"]
    #[inline(always)]
    pub fn is_w0bit(&self) -> bool {
        *self == REDWIDTH_A::W0BIT
    }
}
#[doc = "Field `REDWIDTH` writer - 18:16\\]
Redundant data width in bits"]
pub type REDWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO1_SPEC, u8, REDWIDTH_A, 3, O>;
impl<'a, const O: u8> REDWIDTH_W<'a, O> {
    #[doc = "Redundant data width is 4 bits"]
    #[inline(always)]
    pub fn w4bit(self) -> &'a mut W {
        self.variant(REDWIDTH_A::W4BIT)
    }
    #[doc = "Redundant data width is 2 bits"]
    #[inline(always)]
    pub fn w2bit(self) -> &'a mut W {
        self.variant(REDWIDTH_A::W2BIT)
    }
    #[doc = "Redundant data width is 0. Redundancy/Repair not present."]
    #[inline(always)]
    pub fn w0bit(self) -> &'a mut W {
        self.variant(REDWIDTH_A::W0BIT)
    }
}
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO1_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data width in bits"]
    #[inline(always)]
    pub fn datawidth(&self) -> DATAWIDTH_R {
        DATAWIDTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ECC data width in bits"]
    #[inline(always)]
    pub fn eccwidth(&self) -> ECCWIDTH_R {
        ECCWIDTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Redundant data width in bits"]
    #[inline(always)]
    pub fn redwidth(&self) -> REDWIDTH_R {
        REDWIDTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data width in bits"]
    #[inline(always)]
    #[must_use]
    pub fn datawidth(&mut self) -> DATAWIDTH_W<0> {
        DATAWIDTH_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ECC data width in bits"]
    #[inline(always)]
    #[must_use]
    pub fn eccwidth(&mut self) -> ECCWIDTH_W<8> {
        ECCWIDTH_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Redundant data width in bits"]
    #[inline(always)]
    #[must_use]
    pub fn redwidth(&mut self) -> REDWIDTH_W<16> {
        REDWIDTH_W::new(self)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<19> {
        RESERVED19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gblinfo1](index.html) module"]
pub struct GBLINFO1_SPEC;
impl crate::RegisterSpec for GBLINFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gblinfo1::R](R) reader structure"]
impl crate::Readable for GBLINFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gblinfo1::W](W) writer structure"]
impl crate::Writable for GBLINFO1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GBLINFO1 to value 0x0004_0080"]
impl crate::Resettable for GBLINFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0080;
}
