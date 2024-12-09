#[doc = "Register `ADCCLKCTL` reader"]
pub type R = crate::R<AdcclkctlSpec>;
#[doc = "Register `ADCCLKCTL` writer"]
pub type W = crate::W<AdcclkctlSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
ADC clock request. 0: Disable ADC clock. 1: Enable ADC clock. Only modify REQ when equal to ACK."]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
ADC clock request. 0: Disable ADC clock. 1: Enable ADC clock. Only modify REQ when equal to ACK."]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - 1:1\\]
Clock acknowledgement. 0: ADC clock is disabled. 1: ADC clock is enabled."]
pub type AckR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC clock request. 0: Disable ADC clock. 1: Enable ADC clock. Only modify REQ when equal to ACK."]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clock acknowledgement. 0: ADC clock is disabled. 1: ADC clock is enabled."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 1) & 1) != 0)
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
ADC clock request. 0: Disable ADC clock. 1: Enable ADC clock. Only modify REQ when equal to ACK."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<AdcclkctlSpec> {
        ReqW::new(self, 0)
    }
}
#[doc = "ADC Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcclkctlSpec;
impl crate::RegisterSpec for AdcclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcclkctl::R`](R) reader structure"]
impl crate::Readable for AdcclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcclkctl::W`](W) writer structure"]
impl crate::Writable for AdcclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCLKCTL to value 0"]
impl crate::Resettable for AdcclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
