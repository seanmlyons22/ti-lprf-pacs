#[doc = "Register `PWROFFREQ` reader"]
pub type R = crate::R<PwroffreqSpec>;
#[doc = "Register `PWROFFREQ` writer"]
pub type W = crate::W<PwroffreqSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<PwroffreqSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwroffreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwroffreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwroffreqSpec;
impl crate::RegisterSpec for PwroffreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwroffreq::R`](R) reader structure"]
impl crate::Readable for PwroffreqSpec {}
#[doc = "`write(|w| ..)` method takes [`pwroffreq::W`](W) writer structure"]
impl crate::Writable for PwroffreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWROFFREQ to value 0"]
impl crate::Resettable for PwroffreqSpec {
    const RESET_VALUE: u32 = 0;
}
