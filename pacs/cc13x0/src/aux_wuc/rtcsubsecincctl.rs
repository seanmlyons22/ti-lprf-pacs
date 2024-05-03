#[doc = "Register `RTCSUBSECINCCTL` reader"]
pub type R = crate::R<RtcsubsecincctlSpec>;
#[doc = "Register `RTCSUBSECINCCTL` writer"]
pub type W = crate::W<RtcsubsecincctlSpec>;
#[doc = "Field `UPD_REQ` reader - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
pub type UpdReqR = crate::BitReader;
#[doc = "Field `UPD_REQ` writer - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
pub type UpdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPD_ACK` reader - 1:1\\]
Acknowledgment of the UPD_REQ."]
pub type UpdAckR = crate::BitReader;
#[doc = "Field `UPD_ACK` writer - 1:1\\]
Acknowledgment of the UPD_REQ."]
pub type UpdAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
    #[inline(always)]
    pub fn upd_req(&self) -> UpdReqR {
        UpdReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledgment of the UPD_REQ."]
    #[inline(always)]
    pub fn upd_ack(&self) -> UpdAckR {
        UpdAckR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
    #[inline(always)]
    #[must_use]
    pub fn upd_req(&mut self) -> UpdReqW<RtcsubsecincctlSpec> {
        UpdReqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledgment of the UPD_REQ."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ack(&mut self) -> UpdAckW<RtcsubsecincctlSpec> {
        UpdAckW::new(self, 1)
    }
}
#[doc = "Real Time Counter Sub Second Increment Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecincctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecincctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcsubsecincctlSpec;
impl crate::RegisterSpec for RtcsubsecincctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsubsecincctl::R`](R) reader structure"]
impl crate::Readable for RtcsubsecincctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcsubsecincctl::W`](W) writer structure"]
impl crate::Writable for RtcsubsecincctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSUBSECINCCTL to value 0"]
impl crate::Resettable for RtcsubsecincctlSpec {
    const RESET_VALUE: u32 = 0;
}
