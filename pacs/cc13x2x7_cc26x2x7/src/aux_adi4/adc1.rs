#[doc = "Register `ADC1` reader"]
pub struct R(crate::R<ADC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1` writer"]
pub struct W(crate::W<ADC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_SPEC>;
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
impl From<crate::W<ADC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCALE_DIS` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SCALE_DIS_R = crate::BitReader<bool>;
#[doc = "Field `SCALE_DIS` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SCALE_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADC1_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ADC1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scale_dis(&self) -> SCALE_DIS_R {
        SCALE_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn scale_dis(&mut self) -> SCALE_DIS_W<0> {
        SCALE_DIS_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1](index.html) module"]
pub struct ADC1_SPEC;
impl crate::RegisterSpec for ADC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc1::R](R) reader structure"]
impl crate::Readable for ADC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1::W](W) writer structure"]
impl crate::Writable for ADC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1 to value 0"]
impl crate::Resettable for ADC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
