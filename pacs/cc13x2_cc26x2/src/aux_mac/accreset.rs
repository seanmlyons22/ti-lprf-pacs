#[doc = "Register `ACCRESET` reader"]
pub struct R(crate::R<ACCRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACCRESET` writer"]
pub struct W(crate::W<ACCRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACCRESET_SPEC>;
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
impl From<crate::W<ACCRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACCRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRG` reader - 15:0\\]
Write any value to this register to trigger a reset of all bits in the accumulator."]
pub type TRG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRG` writer - 15:0\\]
Write any value to this register to trigger a reset of all bits in the accumulator."]
pub type TRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACCRESET_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACCRESET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Write any value to this register to trigger a reset of all bits in the accumulator."]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Write any value to this register to trigger a reset of all bits in the accumulator."]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TRG_W<0> {
        TRG_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Accumulator Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accreset](index.html) module"]
pub struct ACCRESET_SPEC;
impl crate::RegisterSpec for ACCRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accreset::R](R) reader structure"]
impl crate::Readable for ACCRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [accreset::W](W) writer structure"]
impl crate::Writable for ACCRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACCRESET to value 0"]
impl crate::Resettable for ACCRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
