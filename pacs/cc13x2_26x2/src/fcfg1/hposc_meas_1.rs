#[doc = "Register `HPOSC_MEAS_1` reader"]
pub type R = crate::R<HposcMeas1Spec>;
#[doc = "Register `HPOSC_MEAS_1` writer"]
pub type W = crate::W<HposcMeas1Spec>;
#[doc = "Field `HPOSC_DT1` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDt1R = crate::FieldReader;
#[doc = "Field `HPOSC_T1` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcT1R = crate::FieldReader;
#[doc = "Field `HPOSC_D1` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcD1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_dt1(&self) -> HposcDt1R {
        HposcDt1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_t1(&self) -> HposcT1R {
        HposcT1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_d1(&self) -> HposcD1R {
        HposcD1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HposcMeas1Spec;
impl crate::RegisterSpec for HposcMeas1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hposc_meas_1::R`](R) reader structure"]
impl crate::Readable for HposcMeas1Spec {}
#[doc = "`write(|w| ..)` method takes [`hposc_meas_1::W`](W) writer structure"]
impl crate::Writable for HposcMeas1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPOSC_MEAS_1 to value 0"]
impl crate::Resettable for HposcMeas1Spec {
    const RESET_VALUE: u32 = 0;
}
