#[doc = "Register `EVFLAGS` reader"]
pub type R = crate::R<EvflagsSpec>;
#[doc = "Register `EVFLAGS` writer"]
pub type W = crate::W<EvflagsSpec>;
#[doc = "Field `CH0` reader - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `CH1` reader - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `CH2` reader - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. AUX_SCE can read the flag through AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and clear it using AUX_SYSIF:RTCEVCLR.RTC_CH2_EV_CLR."]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. AUX_SCE can read the flag through AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and clear it using AUX_SYSIF:RTCEVCLR.RTC_CH2_EV_CLR."]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. AUX_SCE can read the flag through AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and clear it using AUX_SYSIF:RTCEVCLR.RTC_CH2_EV_CLR."]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<EvflagsSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<EvflagsSpec> {
        Ch1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. AUX_SCE can read the flag through AUX_EVCTL:EVSTAT2.AON_RTC_CH2 and clear it using AUX_SYSIF:RTCEVCLR.RTC_CH2_EV_CLR."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<EvflagsSpec> {
        Ch2W::new(self, 16)
    }
}
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvflagsSpec;
impl crate::RegisterSpec for EvflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evflags::R`](R) reader structure"]
impl crate::Readable for EvflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`evflags::W`](W) writer structure"]
impl crate::Writable for EvflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFLAGS to value 0"]
impl crate::Resettable for EvflagsSpec {
    const RESET_VALUE: u32 = 0;
}
