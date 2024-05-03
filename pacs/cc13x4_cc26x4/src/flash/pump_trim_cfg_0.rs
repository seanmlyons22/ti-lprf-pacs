#[doc = "Register `PUMP_TRIM_CFG_0` reader"]
pub type R = crate::R<PumpTrimCfg0Spec>;
#[doc = "Register `PUMP_TRIM_CFG_0` writer"]
pub type W = crate::W<PumpTrimCfg0Spec>;
#[doc = "Field `VHVCT_ERS` reader - 9:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvctErsR = crate::FieldReader<u16>;
#[doc = "Field `VHVCT_ERS` writer - 9:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvctErsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VHVCT_PGM` reader - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPgmR = crate::FieldReader<u16>;
#[doc = "Field `VHVCT_PGM` writer - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPgmW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VHVCT_PV` reader - 29:20\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPvR = crate::FieldReader<u16>;
#[doc = "Field `VHVCT_PV` writer - 29:20\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPvW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RESERVED2` reader - 31:30\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 31:30\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_ers(&self) -> VhvctErsR {
        VhvctErsR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pgm(&self) -> VhvctPgmR {
        VhvctPgmR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - 29:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&self) -> VhvctPvR {
        VhvctPvR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_ers(&mut self) -> VhvctErsW<PumpTrimCfg0Spec> {
        VhvctErsW::new(self, 0)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_pgm(&mut self) -> VhvctPgmW<PumpTrimCfg0Spec> {
        VhvctPgmW::new(self, 10)
    }
    #[doc = "Bits 20:29 - 29:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_pv(&mut self) -> VhvctPvW<PumpTrimCfg0Spec> {
        VhvctPvW::new(self, 20)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<PumpTrimCfg0Spec> {
        Reserved2W::new(self, 30)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pump_trim_cfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pump_trim_cfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PumpTrimCfg0Spec;
impl crate::RegisterSpec for PumpTrimCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pump_trim_cfg_0::R`](R) reader structure"]
impl crate::Readable for PumpTrimCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pump_trim_cfg_0::W`](W) writer structure"]
impl crate::Writable for PumpTrimCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUMP_TRIM_CFG_0 to value 0x00b4_c53a"]
impl crate::Resettable for PumpTrimCfg0Spec {
    const RESET_VALUE: u32 = 0x00b4_c53a;
}
