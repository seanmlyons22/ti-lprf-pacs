#[doc = "Register `CH2CMPINC` reader"]
pub struct R(crate::R<CH2CMPINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CMPINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CMPINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CMPINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CMPINC` writer"]
pub struct W(crate::W<CH2CMPINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CMPINC_SPEC>;
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
impl From<crate::W<CH2CMPINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CMPINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2CMPINC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cmpinc](index.html) module"]
pub struct CH2CMPINC_SPEC;
impl crate::RegisterSpec for CH2CMPINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2cmpinc::R](R) reader structure"]
impl crate::Readable for CH2CMPINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cmpinc::W](W) writer structure"]
impl crate::Writable for CH2CMPINC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CMPINC to value 0"]
impl crate::Resettable for CH2CMPINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
