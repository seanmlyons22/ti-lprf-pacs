#[doc = "Register `ACPR` reader"]
pub struct R(crate::R<ACPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACPR` writer"]
pub struct W(crate::W<ACPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACPR_SPEC>;
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
impl From<crate::W<ACPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
pub type PRESCALER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESCALER` writer - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACPR_SPEC, u16, u16, 13, O>;
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACPR_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acpr](index.html) module"]
pub struct ACPR_SPEC;
impl crate::RegisterSpec for ACPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acpr::R](R) reader structure"]
impl crate::Readable for ACPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acpr::W](W) writer structure"]
impl crate::Writable for ACPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACPR to value 0"]
impl crate::Resettable for ACPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
