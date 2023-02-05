#[doc = "Register `DIVMSW` reader"]
pub struct R(crate::R<DIVMSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVMSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVMSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVMSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVMSW` writer"]
pub struct W(crate::W<DIVMSW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVMSW_SPEC>;
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
impl From<crate::W<DIVMSW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVMSW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSW_ADDRESS` reader - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
pub type MSW_ADDRESS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MSW_ADDRESS` writer - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
pub type MSW_ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVMSW_SPEC, u16, u16, 11, O>;
#[doc = "Field `RESERVED11` reader - 14:11\\]
Ignore on read"]
pub type RESERVED11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED11` writer - 14:11\\]
Ignore on read"]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVMSW_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESULT_IS_ZERO` reader - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub type RESULT_IS_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `RESULT_IS_ZERO` writer - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub type RESULT_IS_ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIVMSW_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Ignore on read"]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Ignore on read"]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVMSW_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&self) -> MSW_ADDRESS_R {
        MSW_ADDRESS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> RESULT_IS_ZERO_R {
        RESULT_IS_ZERO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    #[must_use]
    pub fn msw_address(&mut self) -> MSW_ADDRESS_W<0> {
        MSW_ADDRESS_W::new(self)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn result_is_zero(&mut self) -> RESULT_IS_ZERO_W<15> {
        RESULT_IS_ZERO_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divmsw](index.html) module"]
pub struct DIVMSW_SPEC;
impl crate::RegisterSpec for DIVMSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divmsw::R](R) reader structure"]
impl crate::Readable for DIVMSW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divmsw::W](W) writer structure"]
impl crate::Writable for DIVMSW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIVMSW to value 0x8000"]
impl crate::Resettable for DIVMSW_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
