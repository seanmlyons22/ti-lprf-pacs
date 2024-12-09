#[doc = "Register `HPOSC_MEAS_5` reader"]
pub type R = crate::R<HposcMeas5Spec>;
#[doc = "Register `HPOSC_MEAS_5` writer"]
pub type W = crate::W<HposcMeas5Spec>;
#[doc = "Field `HPOSC_DT5` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcDt5R = crate::FieldReader;
#[doc = "Field `HPOSC_T5` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type HposcT5R = crate::FieldReader;
#[doc = "Field `HPOSC_D5` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type HposcD5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_dt5(&self) -> HposcDt5R {
        HposcDt5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_t5(&self) -> HposcT5R {
        HposcT5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_d5(&self) -> HposcD5R {
        HposcD5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hposc_meas_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hposc_meas_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HposcMeas5Spec;
impl crate::RegisterSpec for HposcMeas5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hposc_meas_5::R`](R) reader structure"]
impl crate::Readable for HposcMeas5Spec {}
#[doc = "`write(|w| ..)` method takes [`hposc_meas_5::W`](W) writer structure"]
impl crate::Writable for HposcMeas5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPOSC_MEAS_5 to value 0"]
impl crate::Resettable for HposcMeas5Spec {
    const RESET_VALUE: u32 = 0;
}
