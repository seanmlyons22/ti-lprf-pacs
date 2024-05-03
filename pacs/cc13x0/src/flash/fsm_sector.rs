#[doc = "Register `FSM_SECTOR` reader"]
pub type R = crate::R<FsmSectorSpec>;
#[doc = "Register `FSM_SECTOR` writer"]
pub type W = crate::W<FsmSectorSpec>;
#[doc = "Field `SEC_OUT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SecOutR = crate::FieldReader;
#[doc = "Field `SEC_OUT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SecOutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECTOR` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SectorR = crate::FieldReader;
#[doc = "Field `SECTOR` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SectorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FSM_SECTOR_EXTENSION` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FsmSectorExtensionR = crate::FieldReader;
#[doc = "Field `FSM_SECTOR_EXTENSION` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FsmSectorExtensionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SECT_ERASED` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SectErasedR = crate::FieldReader<u16>;
#[doc = "Field `SECT_ERASED` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SectErasedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_out(&self) -> SecOutR {
        SecOutR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sector(&self) -> SectorR {
        SectorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector_extension(&self) -> FsmSectorExtensionR {
        FsmSectorExtensionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sect_erased(&self) -> SectErasedR {
        SectErasedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sec_out(&mut self) -> SecOutW<FsmSectorSpec> {
        SecOutW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sector(&mut self) -> SectorW<FsmSectorSpec> {
        SectorW::new(self, 4)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_sector_extension(&mut self) -> FsmSectorExtensionW<FsmSectorSpec> {
        FsmSectorExtensionW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sect_erased(&mut self) -> SectErasedW<FsmSectorSpec> {
        SectErasedW::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sector::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sector::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmSectorSpec;
impl crate::RegisterSpec for FsmSectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_sector::R`](R) reader structure"]
impl crate::Readable for FsmSectorSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_sector::W`](W) writer structure"]
impl crate::Writable for FsmSectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_SECTOR to value 0xffff_0000"]
impl crate::Resettable for FsmSectorSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
