#[doc = "Register `TRIM_CAL_REVISION` reader"]
pub type R = crate::R<TrimCalRevisionSpec>;
#[doc = "Register `TRIM_CAL_REVISION` writer"]
pub type W = crate::W<TrimCalRevisionSpec>;
#[doc = "Field `MP1` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Mp1R = crate::FieldReader<u16>;
#[doc = "Field `FT1` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Ft1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mp1(&self) -> Mp1R {
        Mp1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ft1(&self) -> Ft1R {
        Ft1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim_cal_revision::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim_cal_revision::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrimCalRevisionSpec;
impl crate::RegisterSpec for TrimCalRevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim_cal_revision::R`](R) reader structure"]
impl crate::Readable for TrimCalRevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`trim_cal_revision::W`](W) writer structure"]
impl crate::Writable for TrimCalRevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIM_CAL_REVISION to value 0"]
impl crate::Resettable for TrimCalRevisionSpec {
    const RESET_VALUE: u32 = 0;
}
