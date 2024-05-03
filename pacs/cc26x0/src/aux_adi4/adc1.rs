#[doc = "Register `ADC1` reader"]
pub type R = crate::R<Adc1Spec>;
#[doc = "Register `ADC1` writer"]
pub type W = crate::W<Adc1Spec>;
#[doc = "Field `SCALE_DIS` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ScaleDisR = crate::BitReader;
#[doc = "Field `SCALE_DIS` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ScaleDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scale_dis(&self) -> ScaleDisR {
        ScaleDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn scale_dis(&mut self) -> ScaleDisW<Adc1Spec> {
        ScaleDisW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Adc1Spec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1Spec;
impl crate::RegisterSpec for Adc1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc1::R`](R) reader structure"]
impl crate::Readable for Adc1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc1::W`](W) writer structure"]
impl crate::Writable for Adc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC1 to value 0"]
impl crate::Resettable for Adc1Spec {
    const RESET_VALUE: u8 = 0;
}
