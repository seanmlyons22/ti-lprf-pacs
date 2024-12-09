#[doc = "Register `FCOR_ERR_CNT` reader"]
pub type R = crate::R<FcorErrCntSpec>;
#[doc = "Register `FCOR_ERR_CNT` writer"]
pub type W = crate::W<FcorErrCntSpec>;
#[doc = "Field `COR_ERR_CNT` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type CorErrCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cor_err_cnt(&self) -> CorErrCntR {
        CorErrCntR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcor_err_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcor_err_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcorErrCntSpec;
impl crate::RegisterSpec for FcorErrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcor_err_cnt::R`](R) reader structure"]
impl crate::Readable for FcorErrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`fcor_err_cnt::W`](W) writer structure"]
impl crate::Writable for FcorErrCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCOR_ERR_CNT to value 0"]
impl crate::Resettable for FcorErrCntSpec {
    const RESET_VALUE: u32 = 0;
}
