#[doc = "Register `MCUBUSSTAT` reader"]
pub type R = crate::R<McubusstatSpec>;
#[doc = "Register `MCUBUSSTAT` writer"]
pub type W = crate::W<McubusstatSpec>;
#[doc = "Field `DISCONNECT_ACK` reader - 0:0\\]
Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
pub type DisconnectAckR = crate::BitReader;
#[doc = "Field `DISCONNECTED` reader - 1:1\\]
Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
pub type DisconnectedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ"]
    #[inline(always)]
    pub fn disconnect_ack(&self) -> DisconnectAckR {
        DisconnectAckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
    #[inline(always)]
    pub fn disconnected(&self) -> DisconnectedR {
        DisconnectedR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcubusstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcubusstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McubusstatSpec;
impl crate::RegisterSpec for McubusstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcubusstat::R`](R) reader structure"]
impl crate::Readable for McubusstatSpec {}
#[doc = "`write(|w| ..)` method takes [`mcubusstat::W`](W) writer structure"]
impl crate::Writable for McubusstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUBUSSTAT to value 0"]
impl crate::Resettable for McubusstatSpec {
    const RESET_VALUE: u32 = 0;
}
