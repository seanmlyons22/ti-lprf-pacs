#[doc = "Register `PUMP_TRIM_CFG_1` reader"]
pub type R = crate::R<PumpTrimCfg1Spec>;
#[doc = "Register `PUMP_TRIM_CFG_1` writer"]
pub type W = crate::W<PumpTrimCfg1Spec>;
#[doc = "Field `FOSCCT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FoscctR = crate::FieldReader;
#[doc = "Field `FOSCCT` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FoscctW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IREFCT` reader - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IrefctR = crate::FieldReader;
#[doc = "Field `IREFCT` writer - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IrefctW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IREFTCCT` reader - 14:10\\]
Internal. Only to be used through TI provided API."]
pub type IreftcctR = crate::FieldReader;
#[doc = "Field `IREFTCCT` writer - 14:10\\]
Internal. Only to be used through TI provided API."]
pub type IreftcctW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IREFVRDCT` reader - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type IrefvrdctR = crate::FieldReader;
#[doc = "Field `IREFVRDCT` writer - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type IrefvrdctW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VCGCT` reader - 24:20\\]
Internal. Only to be used through TI provided API."]
pub type VcgctR = crate::FieldReader;
#[doc = "Field `VCGCT` writer - 24:20\\]
Internal. Only to be used through TI provided API."]
pub type VcgctW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VINHCT` reader - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type VinhctR = crate::FieldReader;
#[doc = "Field `VINHCT` writer - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type VinhctW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VINHICCORCTLSB` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type VinhiccorctlsbR = crate::BitReader;
#[doc = "Field `VINHICCORCTLSB` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type VinhiccorctlsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn foscct(&self) -> FoscctR {
        FoscctR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn irefct(&self) -> IrefctR {
        IrefctR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ireftcct(&self) -> IreftcctR {
        IreftcctR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn irefvrdct(&self) -> IrefvrdctR {
        IrefvrdctR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 24:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcgct(&self) -> VcgctR {
        VcgctR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinhct(&self) -> VinhctR {
        VinhctR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinhiccorctlsb(&self) -> VinhiccorctlsbR {
        VinhiccorctlsbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn foscct(&mut self) -> FoscctW<PumpTrimCfg1Spec> {
        FoscctW::new(self, 0)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn irefct(&mut self) -> IrefctW<PumpTrimCfg1Spec> {
        IrefctW::new(self, 6)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ireftcct(&mut self) -> IreftcctW<PumpTrimCfg1Spec> {
        IreftcctW::new(self, 10)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn irefvrdct(&mut self) -> IrefvrdctW<PumpTrimCfg1Spec> {
        IrefvrdctW::new(self, 15)
    }
    #[doc = "Bits 20:24 - 24:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vcgct(&mut self) -> VcgctW<PumpTrimCfg1Spec> {
        VcgctW::new(self, 20)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinhct(&mut self) -> VinhctW<PumpTrimCfg1Spec> {
        VinhctW::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vinhiccorctlsb(&mut self) -> VinhiccorctlsbW<PumpTrimCfg1Spec> {
        VinhiccorctlsbW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pump_trim_cfg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pump_trim_cfg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PumpTrimCfg1Spec;
impl crate::RegisterSpec for PumpTrimCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pump_trim_cfg_1::R`](R) reader structure"]
impl crate::Readable for PumpTrimCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pump_trim_cfg_1::W`](W) writer structure"]
impl crate::Writable for PumpTrimCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUMP_TRIM_CFG_1 to value 0x1920_0000"]
impl crate::Resettable for PumpTrimCfg1Spec {
    const RESET_VALUE: u32 = 0x1920_0000;
}
