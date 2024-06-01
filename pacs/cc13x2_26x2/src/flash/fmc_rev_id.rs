#[doc = "Register `FMC_REV_ID` reader"]
pub type R = crate::R<FmcRevIdSpec>;
#[doc = "Register `FMC_REV_ID` writer"]
pub type W = crate::W<FmcRevIdSpec>;
#[doc = "Field `CONFIG_CRC` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type ConfigCrcR = crate::FieldReader<u16>;
#[doc = "Field `CONFIG_CRC` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type ConfigCrcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MOD_VERSION` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type ModVersionR = crate::FieldReader<u32>;
#[doc = "Field `MOD_VERSION` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type ModVersionW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn config_crc(&self) -> ConfigCrcR {
        ConfigCrcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mod_version(&self) -> ModVersionR {
        ModVersionR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn config_crc(&mut self) -> ConfigCrcW<FmcRevIdSpec> {
        ConfigCrcW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mod_version(&mut self) -> ModVersionW<FmcRevIdSpec> {
        ModVersionW::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_rev_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_rev_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmcRevIdSpec;
impl crate::RegisterSpec for FmcRevIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_rev_id::R`](R) reader structure"]
impl crate::Readable for FmcRevIdSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc_rev_id::W`](W) writer structure"]
impl crate::Writable for FmcRevIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_REV_ID to value 0"]
impl crate::Resettable for FmcRevIdSpec {
    const RESET_VALUE: u32 = 0;
}
