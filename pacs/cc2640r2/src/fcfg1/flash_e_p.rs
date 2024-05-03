#[doc = "Register `FLASH_E_P` reader"]
pub type R = crate::R<FlashEPSpec>;
#[doc = "Register `FLASH_E_P` writer"]
pub type W = crate::W<FlashEPSpec>;
#[doc = "Field `EVSU` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EvsuR = crate::FieldReader;
#[doc = "Field `EVSU` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EvsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PvsuR = crate::FieldReader;
#[doc = "Field `PVSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PvsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESU` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type EsuR = crate::FieldReader;
#[doc = "Field `ESU` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type EsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PSU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PsuR = crate::FieldReader;
#[doc = "Field `PSU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evsu(&self) -> EvsuR {
        EvsuR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvsu(&self) -> PvsuR {
        PvsuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esu(&self) -> EsuR {
        EsuR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psu(&self) -> PsuR {
        PsuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn evsu(&mut self) -> EvsuW<FlashEPSpec> {
        EvsuW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pvsu(&mut self) -> PvsuW<FlashEPSpec> {
        PvsuW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn esu(&mut self) -> EsuW<FlashEPSpec> {
        EsuW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn psu(&mut self) -> PsuW<FlashEPSpec> {
        PsuW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_e_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_e_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashEPSpec;
impl crate::RegisterSpec for FlashEPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_e_p::R`](R) reader structure"]
impl crate::Readable for FlashEPSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_e_p::W`](W) writer structure"]
impl crate::Writable for FlashEPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_E_P to value 0x1733_1a33"]
impl crate::Resettable for FlashEPSpec {
    const RESET_VALUE: u32 = 0x1733_1a33;
}
