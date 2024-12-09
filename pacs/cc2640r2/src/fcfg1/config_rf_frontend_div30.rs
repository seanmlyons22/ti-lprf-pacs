#[doc = "Register `CONFIG_RF_FRONTEND_DIV30` reader"]
pub type R = crate::R<ConfigRfFrontendDiv30Spec>;
#[doc = "Register `CONFIG_RF_FRONTEND_DIV30` writer"]
pub type W = crate::W<ConfigRfFrontendDiv30Spec>;
#[doc = "Field `RFLDO_TRIM_OUTPUT` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type RfldoTrimOutputR = crate::FieldReader;
#[doc = "Field `CTL_PA0_TRIM` reader - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type CtlPa0TrimR = crate::FieldReader;
#[doc = "Field `IFAMP_TRIM` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type IfampTrimR = crate::FieldReader;
#[doc = "Field `LNA_IB` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type LnaIbR = crate::FieldReader;
#[doc = "Field `IFAMP_IB` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type IfampIbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RfldoTrimOutputR {
        RfldoTrimOutputR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CtlPa0TrimR {
        CtlPa0TrimR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IfampTrimR {
        IfampTrimR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&self) -> LnaIbR {
        LnaIbR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IfampIbR {
        IfampIbR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_rf_frontend_div30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_rf_frontend_div30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigRfFrontendDiv30Spec;
impl crate::RegisterSpec for ConfigRfFrontendDiv30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_rf_frontend_div30::R`](R) reader structure"]
impl crate::Readable for ConfigRfFrontendDiv30Spec {}
#[doc = "`write(|w| ..)` method takes [`config_rf_frontend_div30::W`](W) writer structure"]
impl crate::Writable for ConfigRfFrontendDiv30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_RF_FRONTEND_DIV30 to value 0xffff_ffff"]
impl crate::Resettable for ConfigRfFrontendDiv30Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
