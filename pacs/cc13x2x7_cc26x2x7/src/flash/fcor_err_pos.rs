#[doc = "Register `FCOR_ERR_POS` reader"]
pub type R = crate::R<FcorErrPosSpec>;
#[doc = "Register `FCOR_ERR_POS` writer"]
pub type W = crate::W<FcorErrPosSpec>;
#[doc = "Field `SERR_POS` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SerrPosR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn serr_pos(&self) -> SerrPosR {
        SerrPosR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcor_err_pos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcor_err_pos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcorErrPosSpec;
impl crate::RegisterSpec for FcorErrPosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcor_err_pos::R`](R) reader structure"]
impl crate::Readable for FcorErrPosSpec {}
#[doc = "`write(|w| ..)` method takes [`fcor_err_pos::W`](W) writer structure"]
impl crate::Writable for FcorErrPosSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCOR_ERR_POS to value 0"]
impl crate::Resettable for FcorErrPosSpec {
    const RESET_VALUE: u32 = 0;
}
