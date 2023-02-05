#[doc = "Register `ADCCLKCTL` reader"]
pub struct R(crate::R<ADCCLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCLKCTL` writer"]
pub struct W(crate::W<ADCCLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCLKCTL_SPEC>;
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
impl From<crate::W<ADCCLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCLKCTL_SPEC, bool, O>;
#[doc = "Field `ACK` reader - 1:1\\]
Acknowledges the last value written to REQ."]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - 1:1\\]
Acknowledges the last value written to REQ."]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCLKCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledges the last value written to REQ."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<0> {
        REQ_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledges the last value written to REQ."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<1> {
        ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclkctl](index.html) module"]
pub struct ADCCLKCTL_SPEC;
impl crate::RegisterSpec for ADCCLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcclkctl::R](R) reader structure"]
impl crate::Readable for ADCCLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcclkctl::W](W) writer structure"]
impl crate::Writable for ADCCLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCLKCTL to value 0"]
impl crate::Resettable for ADCCLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
