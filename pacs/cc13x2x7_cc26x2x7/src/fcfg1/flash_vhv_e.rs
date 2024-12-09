#[doc = "Register `FLASH_VHV_E` reader"]
pub type R = crate::R<FlashVhvESpec>;
#[doc = "Register `FLASH_VHV_E` writer"]
pub type W = crate::W<FlashVhvESpec>;
#[doc = "Field `VHV_E_STEP_HIGHT` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvEStepHightR = crate::FieldReader<u16>;
#[doc = "Field `VHV_E_START` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvEStartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_step_hight(&self) -> VhvEStepHightR {
        VhvEStepHightR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_start(&self) -> VhvEStartR {
        VhvEStartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_vhv_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_vhv_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashVhvESpec;
impl crate::RegisterSpec for FlashVhvESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_vhv_e::R`](R) reader structure"]
impl crate::Readable for FlashVhvESpec {}
#[doc = "`write(|w| ..)` method takes [`flash_vhv_e::W`](W) writer structure"]
impl crate::Writable for FlashVhvESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_VHV_E to value 0x01"]
impl crate::Resettable for FlashVhvESpec {
    const RESET_VALUE: u32 = 0x01;
}
