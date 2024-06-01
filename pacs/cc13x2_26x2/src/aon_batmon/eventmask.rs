#[doc = "Register `EVENTMASK` reader"]
pub type R = crate::R<EventmaskSpec>;
#[doc = "Register `EVENTMASK` writer"]
pub type W = crate::W<EventmaskSpec>;
#[doc = "Field `BATT_OVER_UL_MASK` reader - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
pub type BattOverUlMaskR = crate::BitReader;
#[doc = "Field `BATT_OVER_UL_MASK` writer - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
pub type BattOverUlMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BATT_BELOW_LL_MASK` reader - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
pub type BattBelowLlMaskR = crate::BitReader;
#[doc = "Field `BATT_BELOW_LL_MASK` writer - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
pub type BattBelowLlMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP_OVER_UL_MASK` reader - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
pub type TempOverUlMaskR = crate::BitReader;
#[doc = "Field `TEMP_OVER_UL_MASK` writer - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
pub type TempOverUlMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP_BELOW_LL_MASK` reader - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
pub type TempBelowLlMaskR = crate::BitReader;
#[doc = "Field `TEMP_BELOW_LL_MASK` writer - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
pub type TempBelowLlMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BATT_UPDATE_MASK` reader - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
pub type BattUpdateMaskR = crate::BitReader;
#[doc = "Field `BATT_UPDATE_MASK` writer - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
pub type BattUpdateMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP_UPDATE_MASK` reader - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
pub type TempUpdateMaskR = crate::BitReader;
#[doc = "Field `TEMP_UPDATE_MASK` writer - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
pub type TempUpdateMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn batt_over_ul_mask(&self) -> BattOverUlMaskR {
        BattOverUlMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn batt_below_ll_mask(&self) -> BattBelowLlMaskR {
        BattBelowLlMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn temp_over_ul_mask(&self) -> TempOverUlMaskR {
        TempOverUlMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn temp_below_ll_mask(&self) -> TempBelowLlMaskR {
        TempBelowLlMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn batt_update_mask(&self) -> BattUpdateMaskR {
        BattUpdateMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    pub fn temp_update_mask(&self) -> TempUpdateMaskR {
        TempUpdateMaskR::new(((self.bits >> 5) & 1) != 0)
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
1: EVENT.BATT_OVER_UL contributes to combined event from BATMON 0: EVENT.BATT_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn batt_over_ul_mask(&mut self) -> BattOverUlMaskW<EventmaskSpec> {
        BattOverUlMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: EVENT.BATT_BELOW_LL contributes to combined event from BATMON 0: EVENT.BATT_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn batt_below_ll_mask(&mut self) -> BattBelowLlMaskW<EventmaskSpec> {
        BattBelowLlMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1: EVENT.TEMP_OVER_UL contributes to combined event from BATMON 0: EVENT.TEMP_OVER_UL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn temp_over_ul_mask(&mut self) -> TempOverUlMaskW<EventmaskSpec> {
        TempOverUlMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1: EVENT.TEMP_BELOW_LL contributes to combined event from BATMON 0: EVENT.TEMP_BELOW_LL does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn temp_below_ll_mask(&mut self) -> TempBelowLlMaskW<EventmaskSpec> {
        TempBelowLlMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1: EVENT.BATT_UPDATE contributes to combined event from BATMON 0: EVENT.BATT_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn batt_update_mask(&mut self) -> BattUpdateMaskW<EventmaskSpec> {
        BattUpdateMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1: EVENT.TEMP_UPDATE contributes to combined event from BATMON 0: EVENT.TEMP_UPDATE does not contribute to combined event from BATMON"]
    #[inline(always)]
    #[must_use]
    pub fn temp_update_mask(&mut self) -> TempUpdateMaskW<EventmaskSpec> {
        TempUpdateMaskW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<EventmaskSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Event Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eventmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eventmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventmaskSpec;
impl crate::RegisterSpec for EventmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventmask::R`](R) reader structure"]
impl crate::Readable for EventmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`eventmask::W`](W) writer structure"]
impl crate::Writable for EventmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTMASK to value 0"]
impl crate::Resettable for EventmaskSpec {
    const RESET_VALUE: u32 = 0;
}
