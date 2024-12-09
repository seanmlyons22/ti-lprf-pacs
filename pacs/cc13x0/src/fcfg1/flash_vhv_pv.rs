#[doc = "Register `FLASH_VHV_PV` reader"]
pub type R = crate::R<FlashVhvPvSpec>;
#[doc = "Register `FLASH_VHV_PV` writer"]
pub type W = crate::W<FlashVhvPvSpec>;
#[doc = "Field `VINH` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VinhR = crate::FieldReader;
#[doc = "Field `VCG2P5` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type Vcg2p5R = crate::FieldReader;
#[doc = "Field `VHV_PV` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvPvR = crate::FieldReader;
#[doc = "Field `RESERVED0` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `TRIM13_PV` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PvR = crate::FieldReader;
#[doc = "Field `RESERVED1` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinh(&self) -> VinhR {
        VinhR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5(&self) -> Vcg2p5R {
        Vcg2p5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_pv(&self) -> VhvPvR {
        VhvPvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&self) -> Trim13PvR {
        Trim13PvR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_vhv_pv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_vhv_pv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashVhvPvSpec;
impl crate::RegisterSpec for FlashVhvPvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_vhv_pv::R`](R) reader structure"]
impl crate::Readable for FlashVhvPvSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_vhv_pv::W`](W) writer structure"]
impl crate::Writable for FlashVhvPvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_VHV_PV to value 0x0008_0001"]
impl crate::Resettable for FlashVhvPvSpec {
    const RESET_VALUE: u32 = 0x0008_0001;
}
