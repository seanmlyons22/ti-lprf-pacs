#[doc = "Register `AMPCOMP_TH2` reader"]
pub type R = crate::R<AmpcompTh2Spec>;
#[doc = "Register `AMPCOMP_TH2` writer"]
pub type W = crate::W<AmpcompTh2Spec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `ADC_COMP_AMPTH_HPM` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type AdcCompAmpthHpmR = crate::FieldReader;
#[doc = "Field `RESERVED1` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `ADC_COMP_AMPTH_LPM` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type AdcCompAmpthLpmR = crate::FieldReader;
#[doc = "Field `RESERVED2` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `LPMUPDATE_HTM` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LpmupdateHtmR = crate::FieldReader;
#[doc = "Field `RESERVED3` reader - 25:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `LPMUPDATE_LTH` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LpmupdateLthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> AdcCompAmpthHpmR {
        AdcCompAmpthHpmR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> AdcCompAmpthLpmR {
        AdcCompAmpthLpmR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_htm(&self) -> LpmupdateHtmR {
        LpmupdateHtmR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LpmupdateLthR {
        LpmupdateLthR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcomp_th2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcomp_th2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpcompTh2Spec;
impl crate::RegisterSpec for AmpcompTh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcomp_th2::R`](R) reader structure"]
impl crate::Readable for AmpcompTh2Spec {}
#[doc = "`write(|w| ..)` method takes [`ampcomp_th2::W`](W) writer structure"]
impl crate::Writable for AmpcompTh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCOMP_TH2 to value 0x6b8b_0303"]
impl crate::Resettable for AmpcompTh2Spec {
    const RESET_VALUE: u32 = 0x6b8b_0303;
}
