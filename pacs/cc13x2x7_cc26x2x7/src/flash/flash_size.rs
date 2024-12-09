#[doc = "Register `FLASH_SIZE` reader"]
pub type R = crate::R<FlashSizeSpec>;
#[doc = "Register `FLASH_SIZE` writer"]
pub type W = crate::W<FlashSizeSpec>;
#[doc = "Field `SECTORS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type SectorsR = crate::FieldReader;
#[doc = "Field `SECTORS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type SectorsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectors(&self) -> SectorsR {
        SectorsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sectors(&mut self) -> SectorsW<FlashSizeSpec> {
        SectorsW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSizeSpec;
impl crate::RegisterSpec for FlashSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_size::R`](R) reader structure"]
impl crate::Readable for FlashSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_size::W`](W) writer structure"]
impl crate::Writable for FlashSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SIZE to value 0x58"]
impl crate::Resettable for FlashSizeSpec {
    const RESET_VALUE: u32 = 0x58;
}
