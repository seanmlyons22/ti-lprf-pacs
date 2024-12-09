#[doc = "Register `FLASH_P_R_PV` reader"]
pub type R = crate::R<FlashPRPvSpec>;
#[doc = "Register `FLASH_P_R_PV` writer"]
pub type W = crate::W<FlashPRPvSpec>;
#[doc = "Field `PVH2` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Pvh2R = crate::FieldReader;
#[doc = "Field `PVH` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PvhR = crate::FieldReader;
#[doc = "Field `RH` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type RhR = crate::FieldReader;
#[doc = "Field `PH` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PhR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh2(&self) -> Pvh2R {
        Pvh2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh(&self) -> PvhR {
        PvhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rh(&self) -> RhR {
        RhR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ph(&self) -> PhR {
        PhR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_p_r_pv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_p_r_pv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashPRPvSpec;
impl crate::RegisterSpec for FlashPRPvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_p_r_pv::R`](R) reader structure"]
impl crate::Readable for FlashPRPvSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_p_r_pv::W`](W) writer structure"]
impl crate::Writable for FlashPRPvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_P_R_PV to value 0x02c1_0200"]
impl crate::Resettable for FlashPRPvSpec {
    const RESET_VALUE: u32 = 0x02c1_0200;
}
