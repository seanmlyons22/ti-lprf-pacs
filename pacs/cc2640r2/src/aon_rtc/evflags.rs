#[doc = "Register `EVFLAGS` reader"]
pub struct R(crate::R<EVFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVFLAGS` writer"]
pub struct W(crate::W<EVFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVFLAGS_SPEC>;
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
impl From<crate::W<EVFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAGS_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVFLAGS_SPEC, u8, u8, 7, O>;
#[doc = "Field `CH1` reader - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAGS_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVFLAGS_SPEC, u8, u8, 7, O>;
#[doc = "Field `CH2` reader - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance. AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2."]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance. AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2."]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAGS_SPEC, bool, O>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVFLAGS_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance. AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<8> {
        CH1_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance. AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<16> {
        CH2_W::new(self)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> RESERVED17_W<17> {
        RESERVED17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflags](index.html) module"]
pub struct EVFLAGS_SPEC;
impl crate::RegisterSpec for EVFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evflags::R](R) reader structure"]
impl crate::Readable for EVFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evflags::W](W) writer structure"]
impl crate::Writable for EVFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVFLAGS to value 0"]
impl crate::Resettable for EVFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
