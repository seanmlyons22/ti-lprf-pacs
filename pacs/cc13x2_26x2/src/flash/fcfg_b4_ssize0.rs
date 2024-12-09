#[doc = "Register `FCFG_B4_SSIZE0` reader"]
pub type R = crate::R<FcfgB4Ssize0Spec>;
#[doc = "Register `FCFG_B4_SSIZE0` writer"]
pub type W = crate::W<FcfgB4Ssize0Spec>;
#[doc = "Field `B4_SECT_SIZE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type B4SectSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_sect_size(&self) -> B4SectSizeR {
        B4SectSizeR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_ssize0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_ssize0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB4Ssize0Spec;
impl crate::RegisterSpec for FcfgB4Ssize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b4_ssize0::R`](R) reader structure"]
impl crate::Readable for FcfgB4Ssize0Spec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b4_ssize0::W`](W) writer structure"]
impl crate::Writable for FcfgB4Ssize0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B4_SSIZE0 to value 0"]
impl crate::Resettable for FcfgB4Ssize0Spec {
    const RESET_VALUE: u32 = 0;
}