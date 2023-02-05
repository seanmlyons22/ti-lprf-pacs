#[doc = "Register `TBPS` reader"]
pub struct R(crate::R<TBPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPS` writer"]
pub struct W(crate::W<TBPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPS_SPEC>;
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
impl From<crate::W<TBPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSS` reader - 7:0\\]
GPT Timer B Pre-scaler"]
pub type PSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSS` writer - 7:0\\]
GPT Timer B Pre-scaler"]
pub type PSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBPS_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBPS_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scaler"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn pss(&mut self) -> PSS_W<0> {
        PSS_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbps](index.html) module"]
pub struct TBPS_SPEC;
impl crate::RegisterSpec for TBPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbps::R](R) reader structure"]
impl crate::Readable for TBPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbps::W](W) writer structure"]
impl crate::Writable for TBPS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPS to value 0"]
impl crate::Resettable for TBPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
