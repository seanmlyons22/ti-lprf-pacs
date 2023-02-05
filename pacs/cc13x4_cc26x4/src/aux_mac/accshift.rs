#[doc = "Register `ACCSHIFT` reader"]
pub struct R(crate::R<ACCSHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCSHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCSHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCSHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACCSHIFT` writer"]
pub struct W(crate::W<ACCSHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACCSHIFT_SPEC>;
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
impl From<crate::W<ACCSHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACCSHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASR1` reader - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
pub type ASR1_R = crate::BitReader<bool>;
#[doc = "Field `ASR1` writer - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
pub type ASR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACCSHIFT_SPEC, bool, O>;
#[doc = "Field `LSR1` reader - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
pub type LSR1_R = crate::BitReader<bool>;
#[doc = "Field `LSR1` writer - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
pub type LSR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACCSHIFT_SPEC, bool, O>;
#[doc = "Field `LSL1` reader - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
pub type LSL1_R = crate::BitReader<bool>;
#[doc = "Field `LSL1` writer - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
pub type LSL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACCSHIFT_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACCSHIFT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
    #[inline(always)]
    pub fn asr1(&self) -> ASR1_R {
        ASR1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
    #[inline(always)]
    pub fn lsr1(&self) -> LSR1_R {
        LSR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
    #[inline(always)]
    pub fn lsl1(&self) -> LSL1_R {
        LSL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Arithmetic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, previous sign bit inserted at bit 39."]
    #[inline(always)]
    #[must_use]
    pub fn asr1(&mut self) -> ASR1_W<0> {
        ASR1_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Logic shift right by 1 bit. Write 1 to shift the accumulator one bit to the right, 0 inserted at bit 39."]
    #[inline(always)]
    #[must_use]
    pub fn lsr1(&mut self) -> LSR1_W<1> {
        LSR1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Logic shift left by 1 bit. Write 1 to shift the accumulator one bit to the left, 0 inserted at bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn lsl1(&mut self) -> LSL1_W<2> {
        LSL1_W::new(self)
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
#[doc = "Accumulator Shift Only one shift operation can be triggered per register write.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accshift](index.html) module"]
pub struct ACCSHIFT_SPEC;
impl crate::RegisterSpec for ACCSHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accshift::R](R) reader structure"]
impl crate::Readable for ACCSHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [accshift::W](W) writer structure"]
impl crate::Writable for ACCSHIFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACCSHIFT to value 0"]
impl crate::Resettable for ACCSHIFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
