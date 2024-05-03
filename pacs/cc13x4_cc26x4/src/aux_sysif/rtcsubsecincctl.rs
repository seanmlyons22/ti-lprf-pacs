#[doc = "Register `RTCSUBSECINCCTL` reader"]
pub type R = crate::R<RtcsubsecincctlSpec>;
#[doc = "Register `RTCSUBSECINCCTL` writer"]
pub type W = crate::W<RtcsubsecincctlSpec>;
#[doc = "Field `UPD_REQ` reader - 0:0\\]
Request AON_RTC to update AON_RTC:SUBSECINC. 0: Clear request to update. 1: Set request to update. Only change UPD_REQ when it equals UPD_ACK. Clear UPD_REQ after UPD_ACK is 1."]
pub type UpdReqR = crate::BitReader;
#[doc = "Field `UPD_REQ` writer - 0:0\\]
Request AON_RTC to update AON_RTC:SUBSECINC. 0: Clear request to update. 1: Set request to update. Only change UPD_REQ when it equals UPD_ACK. Clear UPD_REQ after UPD_ACK is 1."]
pub type UpdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPD_ACK` reader - 1:1\\]
Update acknowledgement. 0: AON_RTC has not acknowledged UPD_REQ. 1: AON_RTC has acknowledged UPD_REQ."]
pub type UpdAckR = crate::BitReader;
#[doc = "Field `UPD_ACK` writer - 1:1\\]
Update acknowledgement. 0: AON_RTC has not acknowledged UPD_REQ. 1: AON_RTC has acknowledged UPD_REQ."]
pub type UpdAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Request AON_RTC to update AON_RTC:SUBSECINC. 0: Clear request to update. 1: Set request to update. Only change UPD_REQ when it equals UPD_ACK. Clear UPD_REQ after UPD_ACK is 1."]
    #[inline(always)]
    pub fn upd_req(&self) -> UpdReqR {
        UpdReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Update acknowledgement. 0: AON_RTC has not acknowledged UPD_REQ. 1: AON_RTC has acknowledged UPD_REQ."]
    #[inline(always)]
    pub fn upd_ack(&self) -> UpdAckR {
        UpdAckR::new(((self.bits >> 1) & 1) != 0)
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
Request AON_RTC to update AON_RTC:SUBSECINC. 0: Clear request to update. 1: Set request to update. Only change UPD_REQ when it equals UPD_ACK. Clear UPD_REQ after UPD_ACK is 1."]
    #[inline(always)]
    #[must_use]
    pub fn upd_req(&mut self) -> UpdReqW<RtcsubsecincctlSpec> {
        UpdReqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Update acknowledgement. 0: AON_RTC has not acknowledged UPD_REQ. 1: AON_RTC has acknowledged UPD_REQ."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ack(&mut self) -> UpdAckW<RtcsubsecincctlSpec> {
        UpdAckW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<RtcsubsecincctlSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Real Time Counter Sub Second Increment Control AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecincctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecincctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
