#[doc = "Register `PWRDWNREQ` reader"]
pub type R = crate::R<PwrdwnreqSpec>;
#[doc = "Register `PWRDWNREQ` writer"]
pub type W = crate::W<PwrdwnreqSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<PwrdwnreqSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdwnreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdwnreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrdwnreqSpec;
impl crate::RegisterSpec for PwrdwnreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrdwnreq::R`](R) reader structure"]
impl crate::Readable for PwrdwnreqSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrdwnreq::W`](W) writer structure"]
impl crate::Writable for PwrdwnreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRDWNREQ to value 0"]
impl crate::Resettable for PwrdwnreqSpec {
    const RESET_VALUE: u32 = 0;
}
