#[doc = "Register `PWRDWNACK` reader"]
pub type R = crate::R<PwrdwnackSpec>;
#[doc = "Register `PWRDWNACK` writer"]
pub type W = crate::W<PwrdwnackSpec>;
#[doc = "Field `ACK` reader - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
pub type AckR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Power Down Acknowledgment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdwnack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdwnack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrdwnackSpec;
impl crate::RegisterSpec for PwrdwnackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrdwnack::R`](R) reader structure"]
impl crate::Readable for PwrdwnackSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrdwnack::W`](W) writer structure"]
impl crate::Writable for PwrdwnackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRDWNACK to value 0"]
impl crate::Resettable for PwrdwnackSpec {
    const RESET_VALUE: u32 = 0;
}
