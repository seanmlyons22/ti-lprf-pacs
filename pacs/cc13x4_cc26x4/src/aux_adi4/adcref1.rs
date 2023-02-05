#[doc = "Register `ADCREF1` reader"]
pub struct R(crate::R<ADCREF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCREF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCREF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCREF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCREF1` writer"]
pub struct W(crate::W<ADCREF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCREF1_SPEC>;
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
impl From<crate::W<ADCREF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCREF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTRIM` reader - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
pub type VTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRIM` writer - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
pub type VTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADCREF1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADCREF1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    #[must_use]
    pub fn vtrim(&mut self) -> VTRIM_W<0> {
        VTRIM_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcref1](index.html) module"]
pub struct ADCREF1_SPEC;
impl crate::RegisterSpec for ADCREF1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcref1::R](R) reader structure"]
impl crate::Readable for ADCREF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcref1::W](W) writer structure"]
impl crate::Writable for ADCREF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCREF1 to value 0"]
impl crate::Resettable for ADCREF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
