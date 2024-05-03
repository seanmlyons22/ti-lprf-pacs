#[doc = "Register `ADCCLKCTL` reader"]
pub type R = crate::R<AdcclkctlSpec>;
#[doc = "Register `ADCCLKCTL` writer"]
pub type W = crate::W<AdcclkctlSpec>;
#[doc = "Field `REQ` reader - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
pub type ReqR = crate::BitReader;
#[doc = "Field `REQ` writer - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - 1:1\\]
Acknowledges the last value written to REQ."]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - 1:1\\]
Acknowledges the last value written to REQ."]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledges the last value written to REQ."]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> ReqW<AdcclkctlSpec> {
        ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Acknowledges the last value written to REQ."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<AdcclkctlSpec> {
        AckW::new(self, 1)
    }
}
#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
