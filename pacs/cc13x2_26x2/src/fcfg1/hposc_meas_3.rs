#[doc = "Register `HPOSC_MEAS_3` reader"]
pub type R = crate::R<HposcMeas3Spec>;
#[doc = "Register `HPOSC_MEAS_3` writer"]
pub type W = crate::W<HposcMeas3Spec>;
#[doc = "Field `HPOSC_DT3` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDt3R = crate::FieldReader;
#[doc = "Field `HPOSC_DT3` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPOSC_T3` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcT3R = crate::FieldReader;
#[doc = "Field `HPOSC_T3` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcT3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPOSC_D3` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcD3R = crate::FieldReader<u16>;
#[doc = "Field `HPOSC_D3` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcD3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_dt3(&self) -> HposcDt3R {
        HposcDt3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_t3(&self) -> HposcT3R {
        HposcT3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_d3(&self) -> HposcD3R {
        HposcD3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_dt3(&mut self) -> HposcDt3W<HposcMeas3Spec> {
        HposcDt3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_t3(&mut self) -> HposcT3W<HposcMeas3Spec> {
        HposcT3W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_d3(&mut self) -> HposcD3W<HposcMeas3Spec> {
        HposcD3W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HposcMeas3Spec;
impl crate::RegisterSpec for HposcMeas3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hposc_meas_3::R`](R) reader structure"]
impl crate::Readable for HposcMeas3Spec {}
#[doc = "`write(|w| ..)` method takes [`hposc_meas_3::W`](W) writer structure"]
impl crate::Writable for HposcMeas3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPOSC_MEAS_3 to value 0"]
impl crate::Resettable for HposcMeas3Spec {
    const RESET_VALUE: u32 = 0;
}
