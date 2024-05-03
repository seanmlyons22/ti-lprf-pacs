#[doc = "Register `ADCTRIG` reader"]
pub type R = crate::R<AdctrigSpec>;
#[doc = "Register `ADCTRIG` writer"]
pub type W = crate::W<AdctrigSpec>;
#[doc = "Field `START` reader - 0:0\\]
Manual ADC trigger. 0: No effect. 1: Single ADC trigger. To manually trigger the ADC, you must set ADCCTL.START_SRC to NO_EVENT&lt;n> to avoid conflict with event-driven ADC trigger."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - 0:0\\]
Manual ADC trigger. 0: No effect. 1: Single ADC trigger. To manually trigger the ADC, you must set ADCCTL.START_SRC to NO_EVENT&lt;n> to avoid conflict with event-driven ADC trigger."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Manual ADC trigger. 0: No effect. 1: Single ADC trigger. To manually trigger the ADC, you must set ADCCTL.START_SRC to NO_EVENT&lt;n> to avoid conflict with event-driven ADC trigger."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Manual ADC trigger. 0: No effect. 1: Single ADC trigger. To manually trigger the ADC, you must set ADCCTL.START_SRC to NO_EVENT&lt;n> to avoid conflict with event-driven ADC trigger."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<AdctrigSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AdctrigSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "ADC Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdctrigSpec;
impl crate::RegisterSpec for AdctrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctrig::R`](R) reader structure"]
impl crate::Readable for AdctrigSpec {}
#[doc = "`write(|w| ..)` method takes [`adctrig::W`](W) writer structure"]
impl crate::Writable for AdctrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCTRIG to value 0"]
impl crate::Resettable for AdctrigSpec {
    const RESET_VALUE: u32 = 0;
}
