#[doc = "Register `AUXCFG` reader"]
pub type R = crate::R<AuxcfgSpec>;
#[doc = "Register `AUXCFG` writer"]
pub type W = crate::W<AuxcfgSpec>;
#[doc = "Field `RAM_RET_EN` reader - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
pub type RamRetEnR = crate::BitReader;
#[doc = "Field `RAM_RET_EN` writer - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
pub type RamRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
    #[inline(always)]
    pub fn ram_ret_en(&self) -> RamRetEnR {
        RamRetEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode"]
    #[inline(always)]
    #[must_use]
    pub fn ram_ret_en(&mut self) -> RamRetEnW<AuxcfgSpec> {
        RamRetEnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AuxcfgSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxcfgSpec;
impl crate::RegisterSpec for AuxcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxcfg::R`](R) reader structure"]
impl crate::Readable for AuxcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`auxcfg::W`](W) writer structure"]
impl crate::Writable for AuxcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXCFG to value 0x01"]
impl crate::Resettable for AuxcfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
