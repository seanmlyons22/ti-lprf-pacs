#[doc = "Register `MASK1` reader"]
pub struct R(crate::R<MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK1` writer"]
pub struct W(crate::W<MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK1_SPEC>;
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
impl From<crate::W<MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - 3:0\\]
Mask on data address when matching against COMP1. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP1. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP1 is 3, this matches a word access of 0, because 3 would be within the word."]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - 3:0\\]
Mask on data address when matching against COMP1. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP1. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP1 is 3, this matches a word access of 0, because 3 would be within the word."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK1_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mask on data address when matching against COMP1. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP1. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP1 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Mask on data address when matching against COMP1. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP1. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP1 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
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
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask1](index.html) module"]
pub struct MASK1_SPEC;
impl crate::RegisterSpec for MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask1::R](R) reader structure"]
impl crate::Readable for MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask1::W](W) writer structure"]
impl crate::Writable for MASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK1 to value 0"]
impl crate::Resettable for MASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
