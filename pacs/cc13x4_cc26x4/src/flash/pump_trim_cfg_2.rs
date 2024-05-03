#[doc = "Register `PUMP_TRIM_CFG_2` reader"]
pub type R = crate::R<PumpTrimCfg2Spec>;
#[doc = "Register `PUMP_TRIM_CFG_2` writer"]
pub type W = crate::W<PumpTrimCfg2Spec>;
#[doc = "Field `VINHICCORCT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VinhiccorctR = crate::FieldReader;
#[doc = "Field `VINHICCORCT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VinhiccorctW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VINLOWCCORCT` reader - 8:4\\]
Internal. Only to be used through TI provided API."]
pub type VinlowccorctR = crate::FieldReader;
#[doc = "Field `VINLOWCCORCT` writer - 8:4\\]
Internal. Only to be used through TI provided API."]
pub type VinlowccorctW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREADCT` reader - 13:9\\]
Internal. Only to be used through TI provided API."]
pub type VreadctR = crate::FieldReader;
#[doc = "Field `VREADCT` writer - 13:9\\]
Internal. Only to be used through TI provided API."]
pub type VreadctW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VSLCT` reader - 19:14\\]
Internal. Only to be used through TI provided API."]
pub type VslctR = crate::FieldReader;
#[doc = "Field `VSLCT` writer - 19:14\\]
Internal. Only to be used through TI provided API."]
pub type VslctW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VWLCT` reader - 25:20\\]
Internal. Only to be used through TI provided API."]
pub type VwlctR = crate::FieldReader;
#[doc = "Field `VWLCT` writer - 25:20\\]
Internal. Only to be used through TI provided API."]
pub type VwlctW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinhiccorct(&self) -> VinhiccorctR {
        VinhiccorctR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinlowccorct(&self) -> VinlowccorctR {
        VinlowccorctR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreadct(&self) -> VreadctR {
        VreadctR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:19 - 19:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vslct(&self) -> VslctR {
        VslctR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - 25:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct(&self) -> VwlctR {
        VwlctR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinhiccorct(&mut self) -> VinhiccorctW<PumpTrimCfg2Spec> {
        VinhiccorctW::new(self, 0)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinlowccorct(&mut self) -> VinlowccorctW<PumpTrimCfg2Spec> {
        VinlowccorctW::new(self, 4)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vreadct(&mut self) -> VreadctW<PumpTrimCfg2Spec> {
        VreadctW::new(self, 9)
    }
    #[doc = "Bits 14:19 - 19:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vslct(&mut self) -> VslctW<PumpTrimCfg2Spec> {
        VslctW::new(self, 14)
    }
    #[doc = "Bits 20:25 - 25:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vwlct(&mut self) -> VwlctW<PumpTrimCfg2Spec> {
        VwlctW::new(self, 20)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<PumpTrimCfg2Spec> {
        Reserved6W::new(self, 26)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pump_trim_cfg_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pump_trim_cfg_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PumpTrimCfg2Spec;
impl crate::RegisterSpec for PumpTrimCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pump_trim_cfg_2::R`](R) reader structure"]
impl crate::Readable for PumpTrimCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pump_trim_cfg_2::W`](W) writer structure"]
impl crate::Writable for PumpTrimCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUMP_TRIM_CFG_2 to value 0x0055_9400"]
impl crate::Resettable for PumpTrimCfg2Spec {
    const RESET_VALUE: u32 = 0x0055_9400;
}
