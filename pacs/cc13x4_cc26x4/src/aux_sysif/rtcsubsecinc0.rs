#[doc = "Register `RTCSUBSECINC0` reader"]
pub type R = crate::R<Rtcsubsecinc0Spec>;
#[doc = "Register `RTCSUBSECINC0` writer"]
pub type W = crate::W<Rtcsubsecinc0Spec>;
#[doc = "Field `INC15_0` reader - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
pub type Inc15_0R = crate::FieldReader<u16>;
#[doc = "Field `INC15_0` writer - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
pub type Inc15_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc15_0(&self) -> Inc15_0R {
        Inc15_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    #[must_use]
    pub fn inc15_0(&mut self) -> Inc15_0W<Rtcsubsecinc0Spec> {
        Inc15_0W::new(self, 0)
    }
}
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcsubsecinc0Spec;
impl crate::RegisterSpec for Rtcsubsecinc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsubsecinc0::R`](R) reader structure"]
impl crate::Readable for Rtcsubsecinc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcsubsecinc0::W`](W) writer structure"]
impl crate::Writable for Rtcsubsecinc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSUBSECINC0 to value 0"]
impl crate::Resettable for Rtcsubsecinc0Spec {
    const RESET_VALUE: u32 = 0;
}
