#[doc = "Register `FLASH_C_E_P_R` reader"]
pub type R = crate::R<FlashCEPRSpec>;
#[doc = "Register `FLASH_C_E_P_R` writer"]
pub type W = crate::W<FlashCEPRSpec>;
#[doc = "Field `CVSU` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type CvsuR = crate::FieldReader<u16>;
#[doc = "Field `A_EXEZ_SETUP` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type AExezSetupR = crate::FieldReader;
#[doc = "Field `PV_ACCESS` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type PvAccessR = crate::FieldReader;
#[doc = "Field `RVSU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RvsuR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cvsu(&self) -> CvsuR {
        CvsuR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn a_exez_setup(&self) -> AExezSetupR {
        AExezSetupR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pv_access(&self) -> PvAccessR {
        PvAccessR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsu(&self) -> RvsuR {
        RvsuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_c_e_p_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_c_e_p_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCEPRSpec;
impl crate::RegisterSpec for FlashCEPRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_c_e_p_r::R`](R) reader structure"]
impl crate::Readable for FlashCEPRSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_c_e_p_r::W`](W) writer structure"]
impl crate::Writable for FlashCEPRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_C_E_P_R to value 0x0a0a_2000"]
impl crate::Resettable for FlashCEPRSpec {
    const RESET_VALUE: u32 = 0x0a0a_2000;
}
