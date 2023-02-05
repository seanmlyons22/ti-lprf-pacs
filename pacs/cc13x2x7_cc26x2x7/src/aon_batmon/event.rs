#[doc = "Register `EVENT` reader"]
pub struct R(crate::R<EVENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENT` writer"]
pub struct W(crate::W<EVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BATT_OVER_UL` reader - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BATT_OVER_UL_R = crate::BitReader<bool>;
#[doc = "Field `BATT_OVER_UL` writer - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BATT_OVER_UL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `BATT_BELOW_LL` reader - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BATT_BELOW_LL_R = crate::BitReader<bool>;
#[doc = "Field `BATT_BELOW_LL` writer - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type BATT_BELOW_LL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `TEMP_OVER_UL` reader - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TEMP_OVER_UL_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_OVER_UL` writer - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TEMP_OVER_UL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `TEMP_BELOW_LL` reader - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TEMP_BELOW_LL_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_BELOW_LL` writer - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
pub type TEMP_BELOW_LL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `BATT_UPDATE` reader - 4:4\\]
Alias to BATUPD.STAT"]
pub type BATT_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `BATT_UPDATE` writer - 4:4\\]
Alias to BATUPD.STAT"]
pub type BATT_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `TEMP_UPDATE` reader - 5:5\\]
Alias to TEMPUPD.STAT"]
pub type TEMP_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_UPDATE` writer - 5:5\\]
Alias to TEMPUPD.STAT"]
pub type TEMP_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVENT_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_over_ul(&self) -> BATT_OVER_UL_R {
        BATT_OVER_UL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_below_ll(&self) -> BATT_BELOW_LL_R {
        BATT_BELOW_LL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_over_ul(&self) -> TEMP_OVER_UL_R {
        TEMP_OVER_UL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_below_ll(&self) -> TEMP_BELOW_LL_R {
        TEMP_BELOW_LL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Alias to BATUPD.STAT"]
    #[inline(always)]
    pub fn batt_update(&self) -> BATT_UPDATE_R {
        BATT_UPDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Alias to TEMPUPD.STAT"]
    #[inline(always)]
    pub fn temp_update(&self) -> TEMP_UPDATE_R {
        TEMP_UPDATE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn batt_over_ul(&mut self) -> BATT_OVER_UL_W<0> {
        BATT_OVER_UL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn batt_below_ll(&mut self) -> BATT_BELOW_LL_W<1> {
        BATT_BELOW_LL_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp_over_ul(&mut self) -> TEMP_OVER_UL_W<2> {
        TEMP_OVER_UL_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp_below_ll(&mut self) -> TEMP_BELOW_LL_W<3> {
        TEMP_BELOW_LL_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Alias to BATUPD.STAT"]
    #[inline(always)]
    #[must_use]
    pub fn batt_update(&mut self) -> BATT_UPDATE_W<4> {
        BATT_UPDATE_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Alias to TEMPUPD.STAT"]
    #[inline(always)]
    #[must_use]
    pub fn temp_update(&mut self) -> TEMP_UPDATE_W<5> {
        TEMP_UPDATE_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event](index.html) module"]
pub struct EVENT_SPEC;
impl crate::RegisterSpec for EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event::R](R) reader structure"]
impl crate::Readable for EVENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [event::W](W) writer structure"]
impl crate::Writable for EVENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENT to value 0"]
impl crate::Resettable for EVENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
