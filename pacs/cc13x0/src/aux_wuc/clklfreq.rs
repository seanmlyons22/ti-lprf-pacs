#[doc = "Register `CLKLFREQ` reader"]
pub type R = crate::R<ClklfreqSpec>;
#[doc = "Register `CLKLFREQ` writer"]
pub type W = crate::W<ClklfreqSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<ClklfreqSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "Low Frequency Clock Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklfreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklfreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClklfreqSpec;
impl crate::RegisterSpec for ClklfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clklfreq::R`](R) reader structure"]
impl crate::Readable for ClklfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`clklfreq::W`](W) writer structure"]
impl crate::Writable for ClklfreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKLFREQ to value 0"]
impl crate::Resettable for ClklfreqSpec {
    const RESET_VALUE: u32 = 0;
}
