#[doc = "Register `MCUCFG` reader"]
pub type R = crate::R<McucfgSpec>;
#[doc = "Register `MCUCFG` writer"]
pub type W = crate::W<McucfgSpec>;
#[doc = "3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SramRetEn {
    #[doc = "15: Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    RetFull = 15,
    #[doc = "7: Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    RetLevel3 = 7,
    #[doc = "3: Retention on for SRAM:BANK0 and SRAM:BANK1"]
    RetLevel2 = 3,
    #[doc = "1: Retention on for SRAM:BANK0"]
    RetLevel1 = 1,
    #[doc = "0: Retention is disabled"]
    RetNone = 0,
}
impl From<SramRetEn> for u8 {
    #[inline(always)]
    fn from(variant: SramRetEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SramRetEn {
    type Ux = u8;
}
impl crate::IsEnum for SramRetEn {}
#[doc = "Field `SRAM_RET_EN` reader - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
pub type SramRetEnR = crate::FieldReader<SramRetEn>;
impl SramRetEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SramRetEn> {
        match self.bits {
            15 => Some(SramRetEn::RetFull),
            7 => Some(SramRetEn::RetLevel3),
            3 => Some(SramRetEn::RetLevel2),
            1 => Some(SramRetEn::RetLevel1),
            0 => Some(SramRetEn::RetNone),
            _ => None,
        }
    }
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    #[inline(always)]
    pub fn is_ret_full(&self) -> bool {
        *self == SramRetEn::RetFull
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn is_ret_level3(&self) -> bool {
        *self == SramRetEn::RetLevel3
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn is_ret_level2(&self) -> bool {
        *self == SramRetEn::RetLevel2
    }
    #[doc = "Retention on for SRAM:BANK0"]
    #[inline(always)]
    pub fn is_ret_level1(&self) -> bool {
        *self == SramRetEn::RetLevel1
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn is_ret_none(&self) -> bool {
        *self == SramRetEn::RetNone
    }
}
#[doc = "Field `SRAM_RET_EN` writer - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
pub type SramRetEnW<'a, REG> = crate::FieldWriter<'a, REG, 4, SramRetEn>;
impl<'a, REG> SramRetEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    #[inline(always)]
    pub fn ret_full(self) -> &'a mut crate::W<REG> {
        self.variant(SramRetEn::RetFull)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn ret_level3(self) -> &'a mut crate::W<REG> {
        self.variant(SramRetEn::RetLevel3)
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn ret_level2(self) -> &'a mut crate::W<REG> {
        self.variant(SramRetEn::RetLevel2)
    }
    #[doc = "Retention on for SRAM:BANK0"]
    #[inline(always)]
    pub fn ret_level1(self) -> &'a mut crate::W<REG> {
        self.variant(SramRetEn::RetLevel1)
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn ret_none(self) -> &'a mut crate::W<REG> {
        self.variant(SramRetEn::RetNone)
    }
}
#[doc = "Field `RESERVED4` reader - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FIXED_WU_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FixedWuEnR = crate::BitReader;
#[doc = "Field `FIXED_WU_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FixedWuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIRT_OFF` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type VirtOffR = crate::BitReader;
#[doc = "Field `VIRT_OFF` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type VirtOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline(always)]
    pub fn sram_ret_en(&self) -> SramRetEnR {
        SramRetEnR::new((self.bits & 0x0f) as u8)
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
    pub fn fixed_wu_en(&self) -> FixedWuEnR {
        FixedWuEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn virt_off(&self) -> VirtOffR {
        VirtOffR::new(((self.bits >> 17) & 1) != 0)
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
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ret_en(&mut self) -> SramRetEnW<McucfgSpec> {
        SramRetEnW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<McucfgSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fixed_wu_en(&mut self) -> FixedWuEnW<McucfgSpec> {
        FixedWuEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn virt_off(&mut self) -> VirtOffW<McucfgSpec> {
        VirtOffW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<McucfgSpec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McucfgSpec;
impl crate::RegisterSpec for McucfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcucfg::R`](R) reader structure"]
impl crate::Readable for McucfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcucfg::W`](W) writer structure"]
impl crate::Writable for McucfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUCFG to value 0x0f"]
impl crate::Resettable for McucfgSpec {
    const RESET_VALUE: u32 = 0x0f;
}
