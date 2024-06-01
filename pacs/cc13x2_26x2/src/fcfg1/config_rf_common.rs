#[doc = "Register `CONFIG_RF_COMMON` reader"]
pub type R = crate::R<ConfigRfCommonSpec>;
#[doc = "Register `CONFIG_RF_COMMON` writer"]
pub type W = crate::W<ConfigRfCommonSpec>;
#[doc = "Field `DACTRIM` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DactrimR = crate::FieldReader;
#[doc = "Field `DACTRIM` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DactrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `QUANTCTLTHRES` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QuantctlthresR = crate::FieldReader;
#[doc = "Field `QUANTCTLTHRES` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QuantctlthresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RFLDO_TRIM_OUTPUT` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RfldoTrimOutputR = crate::FieldReader;
#[doc = "Field `RFLDO_TRIM_OUTPUT` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RfldoTrimOutputW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTL_PA_20DBM_TRIM` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type CtlPa20dbmTrimR = crate::FieldReader;
#[doc = "Field `CTL_PA_20DBM_TRIM` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type CtlPa20dbmTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA20DBMTRIMCOMPLETE_N` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type Pa20dbmtrimcompleteNR = crate::BitReader;
#[doc = "Field `PA20DBMTRIMCOMPLETE_N` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type Pa20dbmtrimcompleteNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLDO_TRIM_OUTPUT` reader - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputR = crate::FieldReader;
#[doc = "Field `SLDO_TRIM_OUTPUT` writer - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DISABLE_CORNER_CAP` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type DisableCornerCapR = crate::BitReader;
#[doc = "Field `DISABLE_CORNER_CAP` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type DisableCornerCapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&self) -> DactrimR {
        DactrimR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QuantctlthresR {
        QuantctlthresR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RfldoTrimOutputR {
        RfldoTrimOutputR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa_20dbm_trim(&self) -> CtlPa20dbmTrimR {
        CtlPa20dbmTrimR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pa20dbmtrimcomplete_n(&self) -> Pa20dbmtrimcompleteNR {
        Pa20dbmtrimcompleteNR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sldo_trim_output(&self) -> SldoTrimOutputR {
        SldoTrimOutputR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable_corner_cap(&self) -> DisableCornerCapR {
        DisableCornerCapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dactrim(&mut self) -> DactrimW<ConfigRfCommonSpec> {
        DactrimW::new(self, 0)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn quantctlthres(&mut self) -> QuantctlthresW<ConfigRfCommonSpec> {
        QuantctlthresW::new(self, 6)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rfldo_trim_output(&mut self) -> RfldoTrimOutputW<ConfigRfCommonSpec> {
        RfldoTrimOutputW::new(self, 9)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctl_pa_20dbm_trim(&mut self) -> CtlPa20dbmTrimW<ConfigRfCommonSpec> {
        CtlPa20dbmTrimW::new(self, 16)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pa20dbmtrimcomplete_n(&mut self) -> Pa20dbmtrimcompleteNW<ConfigRfCommonSpec> {
        Pa20dbmtrimcompleteNW::new(self, 21)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sldo_trim_output(&mut self) -> SldoTrimOutputW<ConfigRfCommonSpec> {
        SldoTrimOutputW::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn disable_corner_cap(&mut self) -> DisableCornerCapW<ConfigRfCommonSpec> {
        DisableCornerCapW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_rf_common::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_rf_common::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigRfCommonSpec;
impl crate::RegisterSpec for ConfigRfCommonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_rf_common::R`](R) reader structure"]
impl crate::Readable for ConfigRfCommonSpec {}
#[doc = "`write(|w| ..)` method takes [`config_rf_common::W`](W) writer structure"]
impl crate::Writable for ConfigRfCommonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_RF_COMMON to value 0x81c0_014d"]
impl crate::Resettable for ConfigRfCommonSpec {
    const RESET_VALUE: u32 = 0x81c0_014d;
}
