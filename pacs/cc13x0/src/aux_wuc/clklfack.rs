#[doc = "Register `CLKLFACK` reader"]
pub type R = crate::R<ClklfackSpec>;
#[doc = "Register `CLKLFACK` writer"]
pub type W = crate::W<ClklfackSpec>;
#[doc = "Field `ACK` reader - 0:0\\]
Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX"]
pub type AckR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Low Frequency Clock Acknowledgment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklfack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklfack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClklfackSpec;
impl crate::RegisterSpec for ClklfackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clklfack::R`](R) reader structure"]
impl crate::Readable for ClklfackSpec {}
#[doc = "`write(|w| ..)` method takes [`clklfack::W`](W) writer structure"]
impl crate::Writable for ClklfackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKLFACK to value 0"]
impl crate::Resettable for ClklfackSpec {
    const RESET_VALUE: u32 = 0;
}
