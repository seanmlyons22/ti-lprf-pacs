#[doc = "Register `FLASH_V` reader"]
pub type R = crate::R<FlashVSpec>;
#[doc = "Register `FLASH_V` writer"]
pub type W = crate::W<FlashVSpec>;
#[doc = "Field `V_READ` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type VReadR = crate::FieldReader;
#[doc = "Field `V_READ` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type VReadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VWL_P` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type VwlPR = crate::FieldReader;
#[doc = "Field `VWL_P` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type VwlPW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VSL_P` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type VslPR = crate::FieldReader;
#[doc = "Field `VSL_P` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type VslPW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_read(&self) -> VReadR {
        VReadR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwl_p(&self) -> VwlPR {
        VwlPR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&self) -> VslPR {
        VslPR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn v_read(&mut self) -> VReadW<FlashVSpec> {
        VReadW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vwl_p(&mut self) -> VwlPW<FlashVSpec> {
        VwlPW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vsl_p(&mut self) -> VslPW<FlashVSpec> {
        VslPW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashVSpec;
impl crate::RegisterSpec for FlashVSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_v::R`](R) reader structure"]
impl crate::Readable for FlashVSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_v::W`](W) writer structure"]
impl crate::Writable for FlashVSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_V to value 0"]
impl crate::Resettable for FlashVSpec {
    const RESET_VALUE: u32 = 0;
}
