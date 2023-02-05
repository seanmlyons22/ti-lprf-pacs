#[doc = "Register `PWROFFREQ` reader"]
pub struct R(crate::R<PWROFFREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWROFFREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWROFFREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWROFFREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWROFFREQ` writer"]
pub struct W(crate::W<PWROFFREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWROFFREQ_SPEC>;
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
impl From<crate::W<PWROFFREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWROFFREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWROFFREQ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<0> {
        REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwroffreq](index.html) module"]
pub struct PWROFFREQ_SPEC;
impl crate::RegisterSpec for PWROFFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwroffreq::R](R) reader structure"]
impl crate::Readable for PWROFFREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwroffreq::W](W) writer structure"]
impl crate::Writable for PWROFFREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWROFFREQ to value 0"]
impl crate::Resettable for PWROFFREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
