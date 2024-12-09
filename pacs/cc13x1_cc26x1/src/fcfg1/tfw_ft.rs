#[doc = "Register `TFW_FT` reader"]
pub type R = crate::R<TfwFtSpec>;
#[doc = "Register `TFW_FT` writer"]
pub type W = crate::W<TfwFtSpec>;
#[doc = "Field `REV` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type RevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfw_ft::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfw_ft::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfwFtSpec;
impl crate::RegisterSpec for TfwFtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfw_ft::R`](R) reader structure"]
impl crate::Readable for TfwFtSpec {}
#[doc = "`write(|w| ..)` method takes [`tfw_ft::W`](W) writer structure"]
impl crate::Writable for TfwFtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFW_FT to value 0"]
impl crate::Resettable for TfwFtSpec {
    const RESET_VALUE: u32 = 0;
}
