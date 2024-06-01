#[doc = "Register `MISC_TRIM` reader"]
pub type R = crate::R<MiscTrimSpec>;
#[doc = "Register `MISC_TRIM` writer"]
pub type W = crate::W<MiscTrimSpec>;
#[doc = "Field `TEMPVSLOPE` reader - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TempvslopeR = crate::FieldReader;
#[doc = "Field `TEMPVSLOPE` writer - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TempvslopeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIM_RECHARGE_COMP_REFLEVEL` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type TrimRechargeCompReflevelR = crate::FieldReader;
#[doc = "Field `TRIM_RECHARGE_COMP_REFLEVEL` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type TrimRechargeCompReflevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM_RECHARGE_COMP_OFFSET` reader - 16:12\\]
Internal. Only to be used through TI provided API."]
pub type TrimRechargeCompOffsetR = crate::FieldReader;
#[doc = "Field `TRIM_RECHARGE_COMP_OFFSET` writer - 16:12\\]
Internal. Only to be used through TI provided API."]
pub type TrimRechargeCompOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&self) -> TempvslopeR {
        TempvslopeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_reflevel(&self) -> TrimRechargeCompReflevelR {
        TrimRechargeCompReflevelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:16 - 16:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_offset(&self) -> TrimRechargeCompOffsetR {
        TrimRechargeCompOffsetR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    #[must_use]
    pub fn tempvslope(&mut self) -> TempvslopeW<MiscTrimSpec> {
        TempvslopeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_recharge_comp_reflevel(&mut self) -> TrimRechargeCompReflevelW<MiscTrimSpec> {
        TrimRechargeCompReflevelW::new(self, 8)
    }
    #[doc = "Bits 12:16 - 16:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_recharge_comp_offset(&mut self) -> TrimRechargeCompOffsetW<MiscTrimSpec> {
        TrimRechargeCompOffsetW::new(self, 12)
    }
}
#[doc = "Miscellaneous Trim Parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscTrimSpec;
impl crate::RegisterSpec for MiscTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_trim::R`](R) reader structure"]
impl crate::Readable for MiscTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_trim::W`](W) writer structure"]
impl crate::Writable for MiscTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_TRIM to value 0xfffe_003b"]
impl crate::Resettable for MiscTrimSpec {
    const RESET_VALUE: u32 = 0xfffe_003b;
}
