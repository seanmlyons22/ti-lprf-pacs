#[doc = "Register `REFCLKCTL` reader"]
pub struct R(crate::R<REFCLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCLKCTL` writer"]
pub struct W(crate::W<REFCLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCLKCTL_SPEC>;
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
impl From<crate::W<REFCLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLKCTL_SPEC, bool, O>;
#[doc = "Field `ACK` reader - 1:1\\]
Acknowledges the last value written to REQ."]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - 1:1\\]
Acknowledges the last value written to REQ."]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFCLKCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
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
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
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
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclkctl](index.html) module"]
pub struct REFCLKCTL_SPEC;
impl crate::RegisterSpec for REFCLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refclkctl::R](R) reader structure"]
impl crate::Readable for REFCLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refclkctl::W](W) writer structure"]
impl crate::Writable for REFCLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFCLKCTL to value 0"]
impl crate::Resettable for REFCLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
