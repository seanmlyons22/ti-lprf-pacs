#[doc = "Register `MASK0` reader"]
pub struct R(crate::R<MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK0` writer"]
pub struct W(crate::W<MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK0_SPEC>;
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
impl From<crate::W<MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - 3:0\\]
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - 3:0\\]
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK0_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
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
Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
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
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](index.html) module"]
pub struct MASK0_SPEC;
impl crate::RegisterSpec for MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask0::R](R) reader structure"]
impl crate::Readable for MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask0::W](W) writer structure"]
impl crate::Writable for MASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK0 to value 0"]
impl crate::Resettable for MASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
