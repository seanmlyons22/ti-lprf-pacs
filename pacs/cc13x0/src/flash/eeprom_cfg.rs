#[doc = "Register `EEPROM_CFG` reader"]
pub type R = crate::R<EepromCfgSpec>;
#[doc = "Register `EEPROM_CFG` writer"]
pub type W = crate::W<EepromCfgSpec>;
#[doc = "Field `AUTOSTART_GRACE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type AutostartGraceR = crate::FieldReader<u32>;
#[doc = "Field `AUTOSTART_GRACE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type AutostartGraceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn autostart_grace(&self) -> AutostartGraceR {
        AutostartGraceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn autostart_grace(&mut self) -> AutostartGraceW<EepromCfgSpec> {
        AutostartGraceW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeprom_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeprom_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EepromCfgSpec;
impl crate::RegisterSpec for EepromCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeprom_cfg::R`](R) reader structure"]
impl crate::Readable for EepromCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`eeprom_cfg::W`](W) writer structure"]
impl crate::Writable for EepromCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEPROM_CFG to value 0x0001_0000"]
impl crate::Resettable for EepromCfgSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
