#[doc = "Register `RAMCFG` reader"]
pub type R = crate::R<RamcfgSpec>;
#[doc = "Register `RAMCFG` writer"]
pub type W = crate::W<RamcfgSpec>;
#[doc = "3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusSramRetEn {
    #[doc = "15: Retention on for all banks SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2, SRAM:BANK3 and SRAM:BANK4"]
    RetFull = 15,
    #[doc = "7: Retention on for SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3"]
    RetLevel3 = 7,
    #[doc = "3: Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    RetLevel2 = 3,
    #[doc = "1: Retention on for SRAM:BANK0 and SRAM:BANK1"]
    RetLevel1 = 1,
    #[doc = "0: Retention is disabled"]
    RetNone = 0,
}
impl From<BusSramRetEn> for u8 {
    #[inline(always)]
    fn from(variant: BusSramRetEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusSramRetEn {
    type Ux = u8;
}
impl crate::IsEnum for BusSramRetEn {}
#[doc = "Field `BUS_SRAM_RET_EN` reader - 3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off"]
pub type BusSramRetEnR = crate::FieldReader<BusSramRetEn>;
impl BusSramRetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusSramRetEn> {
        match self.bits {
            15 => Some(BusSramRetEn::RetFull),
            7 => Some(BusSramRetEn::RetLevel3),
            3 => Some(BusSramRetEn::RetLevel2),
            1 => Some(BusSramRetEn::RetLevel1),
            0 => Some(BusSramRetEn::RetNone),
            _ => None,
        }
    }
    #[doc = "Retention on for all banks SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2, SRAM:BANK3 and SRAM:BANK4"]
    #[inline(always)]
    pub fn is_ret_full(&self) -> bool {
        *self == BusSramRetEn::RetFull
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3"]
    #[inline(always)]
    pub fn is_ret_level3(&self) -> bool {
        *self == BusSramRetEn::RetLevel3
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn is_ret_level2(&self) -> bool {
        *self == BusSramRetEn::RetLevel2
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn is_ret_level1(&self) -> bool {
        *self == BusSramRetEn::RetLevel1
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn is_ret_none(&self) -> bool {
        *self == BusSramRetEn::RetNone
    }
}
#[doc = "Field `BUS_SRAM_RET_EN` writer - 3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off"]
pub type BusSramRetEnW<'a, REG> = crate::FieldWriter<'a, REG, 4, BusSramRetEn>;
impl<'a, REG> BusSramRetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Retention on for all banks SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2, SRAM:BANK3 and SRAM:BANK4"]
    #[inline(always)]
    pub fn ret_full(self) -> &'a mut crate::W<REG> {
        self.variant(BusSramRetEn::RetFull)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3"]
    #[inline(always)]
    pub fn ret_level3(self) -> &'a mut crate::W<REG> {
        self.variant(BusSramRetEn::RetLevel3)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn ret_level2(self) -> &'a mut crate::W<REG> {
        self.variant(BusSramRetEn::RetLevel2)
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn ret_level1(self) -> &'a mut crate::W<REG> {
        self.variant(BusSramRetEn::RetLevel1)
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn ret_none(self) -> &'a mut crate::W<REG> {
        self.variant(BusSramRetEn::RetNone)
    }
}
#[doc = "Field `RESERVED4` reader - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AUX_SRAM_RET_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type AuxSramRetEnR = crate::BitReader;
#[doc = "Field `AUX_SRAM_RET_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type AuxSramRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_SRAM_PWR_OFF` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type AuxSramPwrOffR = crate::BitReader;
#[doc = "Field `AUX_SRAM_PWR_OFF` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type AuxSramPwrOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off"]
    #[inline(always)]
    pub fn bus_sram_ret_en(&self) -> BusSramRetEnR {
        BusSramRetEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_ret_en(&self) -> AuxSramRetEnR {
        AuxSramRetEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_pwr_off(&self) -> AuxSramPwrOffR {
        AuxSramPwrOffR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off"]
    #[inline(always)]
    #[must_use]
    pub fn bus_sram_ret_en(&mut self) -> BusSramRetEnW<RamcfgSpec> {
        BusSramRetEnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<RamcfgSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aux_sram_ret_en(&mut self) -> AuxSramRetEnW<RamcfgSpec> {
        AuxSramRetEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn aux_sram_pwr_off(&mut self) -> AuxSramPwrOffW<RamcfgSpec> {
        AuxSramPwrOffW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<RamcfgSpec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ramcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ramcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamcfgSpec;
impl crate::RegisterSpec for RamcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramcfg::R`](R) reader structure"]
impl crate::Readable for RamcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ramcfg::W`](W) writer structure"]
impl crate::Writable for RamcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMCFG to value 0x0001_000f"]
impl crate::Resettable for RamcfgSpec {
    const RESET_VALUE: u32 = 0x0001_000f;
}
