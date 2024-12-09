#[doc = "Register `TDCCLKCTL` reader"]
pub type R = crate::R<TdcclkctlSpec>;
#[doc = "Register `TDCCLKCTL` writer"]
pub type W = crate::W<TdcclkctlSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
TDC counter clock request. 0: Disable TDC counter clock. 1: Enable TDC counter clock. Only modify REQ when equal to ACK."]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
TDC counter clock request. 0: Disable TDC counter clock. 1: Enable TDC counter clock. Only modify REQ when equal to ACK."]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - 1:1\\]
TDC counter clock acknowledgement. 0: TDC counter clock is disabled. 1: TDC counter clock is enabled."]
pub type AckR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TDC counter clock request. 0: Disable TDC counter clock. 1: Enable TDC counter clock. Only modify REQ when equal to ACK."]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TDC counter clock acknowledgement. 0: TDC counter clock is disabled. 1: TDC counter clock is enabled."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TDC counter clock request. 0: Disable TDC counter clock. 1: Enable TDC counter clock. Only modify REQ when equal to ACK."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<TdcclkctlSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=RCOSC_HF (24 or 48 MHz), wait until ACK=1. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=XOSC_HF, wait until ACK=1 and DDI_0_OSC:STAT2.XOSC_HF_FREQGOOD=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcclkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcclkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdcclkctlSpec;
impl crate::RegisterSpec for TdcclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdcclkctl::R`](R) reader structure"]
impl crate::Readable for TdcclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tdcclkctl::W`](W) writer structure"]
impl crate::Writable for TdcclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDCCLKCTL to value 0"]
impl crate::Resettable for TdcclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
