#[doc = "Register `DEBUG4` reader"]
pub type R = crate::R<Debug4Spec>;
#[doc = "Register `DEBUG4` writer"]
pub type W = crate::W<Debug4Spec>;
#[doc = "Field `ADC_CTRL0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type AdcCtrl0R = crate::FieldReader<u16>;
#[doc = "Field `ADC_CTRL0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type AdcCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_ctrl0(&self) -> AdcCtrl0R {
        AdcCtrl0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_ctrl0(&mut self) -> AdcCtrl0W<Debug4Spec> {
        AdcCtrl0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Debug4Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Debug4Spec;
impl crate::RegisterSpec for Debug4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug4::R`](R) reader structure"]
impl crate::Readable for Debug4Spec {}
#[doc = "`write(|w| ..)` method takes [`debug4::W`](W) writer structure"]
impl crate::Writable for Debug4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG4 to value 0"]
impl crate::Resettable for Debug4Spec {
    const RESET_VALUE: u32 = 0;
}
