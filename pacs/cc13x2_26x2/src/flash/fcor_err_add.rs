#[doc = "Register `FCOR_ERR_ADD` reader"]
pub type R = crate::R<FcorErrAddSpec>;
#[doc = "Register `FCOR_ERR_ADD` writer"]
pub type W = crate::W<FcorErrAddSpec>;
#[doc = "Field `FCOR_ERR_ADD` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FcorErrAddR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fcor_err_add(&self) -> FcorErrAddR {
        FcorErrAddR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcor_err_add::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcor_err_add::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcorErrAddSpec;
impl crate::RegisterSpec for FcorErrAddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcor_err_add::R`](R) reader structure"]
impl crate::Readable for FcorErrAddSpec {}
#[doc = "`write(|w| ..)` method takes [`fcor_err_add::W`](W) writer structure"]
impl crate::Writable for FcorErrAddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCOR_ERR_ADD to value 0"]
impl crate::Resettable for FcorErrAddSpec {
    const RESET_VALUE: u32 = 0;
}
