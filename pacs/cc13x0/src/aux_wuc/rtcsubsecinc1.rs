#[doc = "Register `RTCSUBSECINC1` reader"]
pub type R = crate::R<Rtcsubsecinc1Spec>;
#[doc = "Register `RTCSUBSECINC1` writer"]
pub type W = crate::W<Rtcsubsecinc1Spec>;
#[doc = "Field `INC23_16` reader - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
pub type Inc23_16R = crate::FieldReader;
#[doc = "Field `INC23_16` writer - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
pub type Inc23_16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    pub fn inc23_16(&self) -> Inc23_16R {
        Inc23_16R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    #[must_use]
    pub fn inc23_16(&mut self) -> Inc23_16W<Rtcsubsecinc1Spec> {
        Inc23_16W::new(self, 0)
    }
}
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcsubsecinc1Spec;
impl crate::RegisterSpec for Rtcsubsecinc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsubsecinc1::R`](R) reader structure"]
impl crate::Readable for Rtcsubsecinc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcsubsecinc1::W`](W) writer structure"]
impl crate::Writable for Rtcsubsecinc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSUBSECINC1 to value 0"]
impl crate::Resettable for Rtcsubsecinc1Spec {
    const RESET_VALUE: u32 = 0;
}
