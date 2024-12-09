#[doc = "Register `FLASH_ERA_PW` reader"]
pub type R = crate::R<FlashEraPwSpec>;
#[doc = "Register `FLASH_ERA_PW` writer"]
pub type W = crate::W<FlashEraPwSpec>;
#[doc = "Field `ERASE_PW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ErasePwR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn erase_pw(&self) -> ErasePwR {
        ErasePwR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_era_pw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_era_pw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashEraPwSpec;
impl crate::RegisterSpec for FlashEraPwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_era_pw::R`](R) reader structure"]
impl crate::Readable for FlashEraPwSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_era_pw::W`](W) writer structure"]
impl crate::Writable for FlashEraPwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ERA_PW to value 0x0fa0"]
impl crate::Resettable for FlashEraPwSpec {
    const RESET_VALUE: u32 = 0x0fa0;
}
