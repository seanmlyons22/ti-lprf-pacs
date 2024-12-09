#[doc = "Register `REFCLKCTL` reader"]
pub type R = crate::R<RefclkctlSpec>;
#[doc = "Register `REFCLKCTL` writer"]
pub type W = crate::W<RefclkctlSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - 1:1\\]
Acknowledges the last value written to REQ."]
pub type AckR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledges the last value written to REQ."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<RefclkctlSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refclkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefclkctlSpec;
impl crate::RegisterSpec for RefclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refclkctl::R`](R) reader structure"]
impl crate::Readable for RefclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`refclkctl::W`](W) writer structure"]
impl crate::Writable for RefclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REFCLKCTL to value 0"]
impl crate::Resettable for RefclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
