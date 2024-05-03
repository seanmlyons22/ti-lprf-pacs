#[doc = "Register `MCUBUSCTL` reader"]
pub type R = crate::R<McubusctlSpec>;
#[doc = "Register `MCUBUSCTL` writer"]
pub type W = crate::W<McubusctlSpec>;
#[doc = "Field `DISCONNECT_REQ` reader - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
pub type DisconnectReqR = crate::BitReader;
#[doc = "Field `DISCONNECT_REQ` writer - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
pub type DisconnectReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
    #[inline(always)]
    pub fn disconnect_req(&self) -> DisconnectReqR {
        DisconnectReqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
    #[inline(always)]
    #[must_use]
    pub fn disconnect_req(&mut self) -> DisconnectReqW<McubusctlSpec> {
        DisconnectReqW::new(self, 0)
    }
}
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcubusctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcubusctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McubusctlSpec;
impl crate::RegisterSpec for McubusctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcubusctl::R`](R) reader structure"]
impl crate::Readable for McubusctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcubusctl::W`](W) writer structure"]
impl crate::Writable for McubusctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUBUSCTL to value 0"]
impl crate::Resettable for McubusctlSpec {
    const RESET_VALUE: u32 = 0;
}
