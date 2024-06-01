#[doc = "Register `HPOSC_MEAS_2` reader"]
pub type R = crate::R<HposcMeas2Spec>;
#[doc = "Register `HPOSC_MEAS_2` writer"]
pub type W = crate::W<HposcMeas2Spec>;
#[doc = "Field `HPOSC_DT2` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDt2R = crate::FieldReader;
#[doc = "Field `HPOSC_DT2` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPOSC_T2` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcT2R = crate::FieldReader;
#[doc = "Field `HPOSC_T2` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcT2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPOSC_D2` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcD2R = crate::FieldReader<u16>;
#[doc = "Field `HPOSC_D2` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcD2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_dt2(&self) -> HposcDt2R {
        HposcDt2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_t2(&self) -> HposcT2R {
        HposcT2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_d2(&self) -> HposcD2R {
        HposcD2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_dt2(&mut self) -> HposcDt2W<HposcMeas2Spec> {
        HposcDt2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_t2(&mut self) -> HposcT2W<HposcMeas2Spec> {
        HposcT2W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_d2(&mut self) -> HposcD2W<HposcMeas2Spec> {
        HposcD2W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HposcMeas2Spec;
impl crate::RegisterSpec for HposcMeas2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hposc_meas_2::R`](R) reader structure"]
impl crate::Readable for HposcMeas2Spec {}
#[doc = "`write(|w| ..)` method takes [`hposc_meas_2::W`](W) writer structure"]
impl crate::Writable for HposcMeas2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPOSC_MEAS_2 to value 0"]
impl crate::Resettable for HposcMeas2Spec {
    const RESET_VALUE: u32 = 0;
}
