#[doc = "Register `RTCSUBSECINC1` reader"]
pub type R = crate::R<Rtcsubsecinc1Spec>;
#[doc = "Register `RTCSUBSECINC1` writer"]
pub type W = crate::W<Rtcsubsecinc1Spec>;
#[doc = "Field `INC23_16` reader - 7:0\\]
New value for bits 23:16 in AON_RTC:SUBSECINC."]
pub type Inc23_16R = crate::FieldReader;
#[doc = "Field `INC23_16` writer - 7:0\\]
New value for bits 23:16 in AON_RTC:SUBSECINC."]
pub type Inc23_16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
New value for bits 23:16 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc23_16(&self) -> Inc23_16R {
        Inc23_16R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
New value for bits 23:16 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    #[must_use]
    pub fn inc23_16(&mut self) -> Inc23_16W<Rtcsubsecinc1Spec> {
        Inc23_16W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Rtcsubsecinc1Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
