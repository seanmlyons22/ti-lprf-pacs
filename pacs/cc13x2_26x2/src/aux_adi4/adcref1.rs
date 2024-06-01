#[doc = "Register `ADCREF1` reader"]
pub type R = crate::R<Adcref1Spec>;
#[doc = "Register `ADCREF1` writer"]
pub type W = crate::W<Adcref1Spec>;
#[doc = "Field `VTRIM` reader - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
pub type VtrimR = crate::FieldReader;
#[doc = "Field `VTRIM` writer - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
pub type VtrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    pub fn vtrim(&self) -> VtrimR {
        VtrimR::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    #[must_use]
    pub fn vtrim(&mut self) -> VtrimW<Adcref1Spec> {
        VtrimW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Adcref1Spec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcref1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcref1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcref1Spec;
impl crate::RegisterSpec for Adcref1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcref1::R`](R) reader structure"]
impl crate::Readable for Adcref1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcref1::W`](W) writer structure"]
impl crate::Writable for Adcref1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADCREF1 to value 0"]
impl crate::Resettable for Adcref1Spec {
    const RESET_VALUE: u8 = 0;
}
