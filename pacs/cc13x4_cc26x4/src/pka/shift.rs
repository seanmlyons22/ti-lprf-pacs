#[doc = "Register `SHIFT` reader"]
pub struct R(crate::R<SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFT` writer"]
pub struct W(crate::W<SHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFT_SPEC>;
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
impl From<crate::W<SHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUM_BITS_TO_SHIFT` reader - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub type NUM_BITS_TO_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_BITS_TO_SHIFT` writer - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
pub type NUM_BITS_TO_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFT_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED11` reader - 31:5\\]
Set to zero on write, ignore on read"]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:5\\]
Set to zero on write, ignore on read"]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFT_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    pub fn num_bits_to_shift(&self) -> NUM_BITS_TO_SHIFT_R {
        NUM_BITS_TO_SHIFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
This register specifies the number of bits to shift the input vector (in the range 0-31) during a Rshift or Lshift operation."]
    #[inline(always)]
    #[must_use]
    pub fn num_bits_to_shift(&mut self) -> NUM_BITS_TO_SHIFT_W<0> {
        NUM_BITS_TO_SHIFT_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<5> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA Bit Shift Value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shift](index.html) module"]
pub struct SHIFT_SPEC;
impl crate::RegisterSpec for SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shift::R](R) reader structure"]
impl crate::Readable for SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shift::W](W) writer structure"]
impl crate::Writable for SHIFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFT to value 0"]
impl crate::Resettable for SHIFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
