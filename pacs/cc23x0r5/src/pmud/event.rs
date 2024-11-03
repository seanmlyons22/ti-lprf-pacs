#[doc = "Register `EVENT` reader"]
pub type R = crate::R<EventSpec>;
#[doc = "Register `EVENT` writer"]
pub type W = crate::W<EventSpec>;
#[doc = "Field `BATT_OVER_UL` reader - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BattOverUlR = crate::BitReader;
#[doc = "Field `BATT_OVER_UL` writer - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BattOverUlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BATT_BELOW_LL` reader - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BattBelowLlR = crate::BitReader;
#[doc = "Field `BATT_BELOW_LL` writer - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BattBelowLlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP_OVER_UL` reader - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TempOverUlR = crate::BitReader;
#[doc = "Field `TEMP_OVER_UL` writer - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TempOverUlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP_BELOW_LL` reader - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TempBelowLlR = crate::BitReader;
#[doc = "Field `TEMP_BELOW_LL` writer - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TempBelowLlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BATT_UPDATE` reader - 4:4\\]
Alias to BATUPD.STAT"]
pub type BattUpdateR = crate::BitReader;
#[doc = "Field `BATT_UPDATE` writer - 4:4\\]
Alias to BATUPD.STAT"]
pub type BattUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP_UPDATE` reader - 5:5\\]
Alias to TEMPUPD.STAT"]
pub type TempUpdateR = crate::BitReader;
#[doc = "Field `TEMP_UPDATE` writer - 5:5\\]
Alias to TEMPUPD.STAT"]
pub type TempUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_over_ul(&self) -> BattOverUlR {
        BattOverUlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_below_ll(&self) -> BattBelowLlR {
        BattBelowLlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_over_ul(&self) -> TempOverUlR {
        TempOverUlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_below_ll(&self) -> TempBelowLlR {
        TempBelowLlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Alias to BATUPD.STAT"]
    #[inline(always)]
    pub fn batt_update(&self) -> BattUpdateR {
        BattUpdateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Alias to TEMPUPD.STAT"]
    #[inline(always)]
    pub fn temp_update(&self) -> TempUpdateR {
        TempUpdateR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn batt_over_ul(&mut self) -> BattOverUlW<EventSpec> {
        BattOverUlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn batt_below_ll(&mut self) -> BattBelowLlW<EventSpec> {
        BattBelowLlW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp_over_ul(&mut self) -> TempOverUlW<EventSpec> {
        TempOverUlW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp_below_ll(&mut self) -> TempBelowLlW<EventSpec> {
        TempBelowLlW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Alias to BATUPD.STAT"]
    #[inline(always)]
    #[must_use]
    pub fn batt_update(&mut self) -> BattUpdateW<EventSpec> {
        BattUpdateW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Alias to TEMPUPD.STAT"]
    #[inline(always)]
    #[must_use]
    pub fn temp_update(&mut self) -> TempUpdateW<EventSpec> {
        TempUpdateW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<EventSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventSpec;
impl crate::RegisterSpec for EventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event::R`](R) reader structure"]
impl crate::Readable for EventSpec {}
#[doc = "`write(|w| ..)` method takes [`event::W`](W) writer structure"]
impl crate::Writable for EventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENT to value 0"]
impl crate::Resettable for EventSpec {
    const RESET_VALUE: u32 = 0;
}
