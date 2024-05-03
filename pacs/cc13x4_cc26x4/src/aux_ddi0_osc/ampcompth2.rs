#[doc = "Register `AMPCOMPTH2` reader"]
pub type R = crate::R<Ampcompth2Spec>;
#[doc = "Register `AMPCOMPTH2` writer"]
pub type W = crate::W<Ampcompth2Spec>;
#[doc = "Field `SPARE0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare0R = crate::FieldReader;
#[doc = "Field `SPARE0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_COMP_AMPTH_HPM` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type AdcCompAmpthHpmR = crate::FieldReader;
#[doc = "Field `ADC_COMP_AMPTH_HPM` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type AdcCompAmpthHpmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPARE8` reader - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare8R = crate::FieldReader;
#[doc = "Field `SPARE8` writer - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_COMP_AMPTH_LPM` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type AdcCompAmpthLpmR = crate::FieldReader;
#[doc = "Field `ADC_COMP_AMPTH_LPM` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type AdcCompAmpthLpmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPARE16` reader - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare16R = crate::FieldReader;
#[doc = "Field `SPARE16` writer - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare16W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPMUPDATE_HTH` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LpmupdateHthR = crate::FieldReader;
#[doc = "Field `LPMUPDATE_HTH` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LpmupdateHthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPARE24` reader - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare24R = crate::FieldReader;
#[doc = "Field `SPARE24` writer - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare24W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPMUPDATE_LTH` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LpmupdateLthR = crate::FieldReader;
#[doc = "Field `LPMUPDATE_LTH` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LpmupdateLthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&self) -> Spare0R {
        Spare0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> AdcCompAmpthHpmR {
        AdcCompAmpthHpmR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare8(&self) -> Spare8R {
        Spare8R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> AdcCompAmpthLpmR {
        AdcCompAmpthLpmR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> Spare16R {
        Spare16R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_hth(&self) -> LpmupdateHthR {
        LpmupdateHthR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&self) -> Spare24R {
        Spare24R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LpmupdateLthR {
        LpmupdateLthR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare0(&mut self) -> Spare0W<Ampcompth2Spec> {
        Spare0W::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_comp_ampth_hpm(&mut self) -> AdcCompAmpthHpmW<Ampcompth2Spec> {
        AdcCompAmpthHpmW::new(self, 2)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare8(&mut self) -> Spare8W<Ampcompth2Spec> {
        Spare8W::new(self, 8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_comp_ampth_lpm(&mut self) -> AdcCompAmpthLpmW<Ampcompth2Spec> {
        AdcCompAmpthLpmW::new(self, 10)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare16(&mut self) -> Spare16W<Ampcompth2Spec> {
        Spare16W::new(self, 16)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpmupdate_hth(&mut self) -> LpmupdateHthW<Ampcompth2Spec> {
        LpmupdateHthW::new(self, 18)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare24(&mut self) -> Spare24W<Ampcompth2Spec> {
        Spare24W::new(self, 24)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpmupdate_lth(&mut self) -> LpmupdateLthW<Ampcompth2Spec> {
        LpmupdateLthW::new(self, 26)
    }
}
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcompth2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcompth2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ampcompth2Spec;
impl crate::RegisterSpec for Ampcompth2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcompth2::R`](R) reader structure"]
impl crate::Readable for Ampcompth2Spec {}
#[doc = "`write(|w| ..)` method takes [`ampcompth2::W`](W) writer structure"]
impl crate::Writable for Ampcompth2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCOMPTH2 to value 0"]
impl crate::Resettable for Ampcompth2Spec {
    const RESET_VALUE: u32 = 0;
}
