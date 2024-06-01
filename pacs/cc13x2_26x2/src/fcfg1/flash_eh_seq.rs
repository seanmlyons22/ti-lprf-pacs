#[doc = "Register `FLASH_EH_SEQ` reader"]
pub type R = crate::R<FlashEhSeqSpec>;
#[doc = "Register `FLASH_EH_SEQ` writer"]
pub type W = crate::W<FlashEhSeqSpec>;
#[doc = "Field `SM_FREQUENCY` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SmFrequencyR = crate::FieldReader<u16>;
#[doc = "Field `SM_FREQUENCY` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SmFrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VSTAT` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VstatR = crate::FieldReader;
#[doc = "Field `VSTAT` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VstatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQ` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type SeqR = crate::FieldReader;
#[doc = "Field `SEQ` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type SeqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EH` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type EhR = crate::FieldReader;
#[doc = "Field `EH` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type EhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sm_frequency(&self) -> SmFrequencyR {
        SmFrequencyR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat(&self) -> VstatR {
        VstatR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq(&self) -> SeqR {
        SeqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eh(&self) -> EhR {
        EhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sm_frequency(&mut self) -> SmFrequencyW<FlashEhSeqSpec> {
        SmFrequencyW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vstat(&mut self) -> VstatW<FlashEhSeqSpec> {
        VstatW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn seq(&mut self) -> SeqW<FlashEhSeqSpec> {
        SeqW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eh(&mut self) -> EhW<FlashEhSeqSpec> {
        EhW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_eh_seq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_eh_seq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashEhSeqSpec;
impl crate::RegisterSpec for FlashEhSeqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_eh_seq::R`](R) reader structure"]
impl crate::Readable for FlashEhSeqSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_eh_seq::W`](W) writer structure"]
impl crate::Writable for FlashEhSeqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_EH_SEQ to value 0x0200_f000"]
impl crate::Resettable for FlashEhSeqSpec {
    const RESET_VALUE: u32 = 0x0200_f000;
}
