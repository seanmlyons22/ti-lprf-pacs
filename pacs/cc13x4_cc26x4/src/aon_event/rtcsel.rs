#[doc = "Register `RTCSEL` reader"]
pub type R = crate::R<RtcselSpec>;
#[doc = "Register `RTCSEL` writer"]
pub type W = crate::W<RtcselSpec>;
#[doc = "5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RtcCh1CaptEv {
    #[doc = "63: 0"]
    None = 63,
    #[doc = "56: 0"]
    AuxCompbAsyncN = 56,
    #[doc = "55: 0"]
    AuxCompbAsync = 55,
    #[doc = "54: 0"]
    BatmonVolt = 54,
    #[doc = "53: 0"]
    BatmonTemp = 53,
    #[doc = "52: 0"]
    AuxTimer1Ev = 52,
    #[doc = "51: 0"]
    AuxTimer0Ev = 51,
    #[doc = "50: 0"]
    AuxTdcDone = 50,
    #[doc = "49: 0"]
    AuxAdcDone = 49,
    #[doc = "48: 0"]
    AuxCompb = 48,
    #[doc = "47: 0"]
    AuxCompa = 47,
    #[doc = "46: 0"]
    AuxSwev2 = 46,
    #[doc = "45: 0"]
    AuxSwev1 = 45,
    #[doc = "44: 0"]
    AuxSwev0 = 44,
    #[doc = "43: 0"]
    Jtag = 43,
    #[doc = "42: 0"]
    RtcUpd = 42,
    #[doc = "41: 0"]
    RtcCombDly = 41,
    #[doc = "40: 0"]
    RtcCh2Dly = 40,
    #[doc = "39: 0"]
    RtcCh1Dly = 39,
    #[doc = "38: 0"]
    RtcCh0Dly = 38,
    #[doc = "37: 0"]
    RtcCh2 = 37,
    #[doc = "36: 0"]
    RtcCh1 = 36,
    #[doc = "35: 0"]
    RtcCh0 = 35,
    #[doc = "32: 0"]
    Pad = 32,
    #[doc = "9: 0"]
    BatmonCombined = 9,
    #[doc = "8: 0"]
    BatmonTempLl = 8,
    #[doc = "7: 0"]
    BatmonTempUl = 7,
    #[doc = "6: 0"]
    BatmonBattLl = 6,
    #[doc = "5: 0"]
    BatmonBattUl = 5,
    #[doc = "4: 0"]
    AuxTimer2Ev3 = 4,
    #[doc = "3: 0"]
    AuxTimer2Ev2 = 3,
    #[doc = "2: 0"]
    AuxTimer2Ev1 = 2,
    #[doc = "1: 0"]
    AuxTimer2Ev0 = 1,
    #[doc = "0: Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    IoevRtc = 0,
}
impl From<RtcCh1CaptEv> for u8 {
    #[inline(always)]
    fn from(variant: RtcCh1CaptEv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RtcCh1CaptEv {
    type Ux = u8;
}
impl crate::IsEnum for RtcCh1CaptEv {}
#[doc = "Field `RTC_CH1_CAPT_EV` reader - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
pub type RtcCh1CaptEvR = crate::FieldReader<RtcCh1CaptEv>;
impl RtcCh1CaptEvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RtcCh1CaptEv> {
        match self.bits {
            63 => Some(RtcCh1CaptEv::None),
            56 => Some(RtcCh1CaptEv::AuxCompbAsyncN),
            55 => Some(RtcCh1CaptEv::AuxCompbAsync),
            54 => Some(RtcCh1CaptEv::BatmonVolt),
            53 => Some(RtcCh1CaptEv::BatmonTemp),
            52 => Some(RtcCh1CaptEv::AuxTimer1Ev),
            51 => Some(RtcCh1CaptEv::AuxTimer0Ev),
            50 => Some(RtcCh1CaptEv::AuxTdcDone),
            49 => Some(RtcCh1CaptEv::AuxAdcDone),
            48 => Some(RtcCh1CaptEv::AuxCompb),
            47 => Some(RtcCh1CaptEv::AuxCompa),
            46 => Some(RtcCh1CaptEv::AuxSwev2),
            45 => Some(RtcCh1CaptEv::AuxSwev1),
            44 => Some(RtcCh1CaptEv::AuxSwev0),
            43 => Some(RtcCh1CaptEv::Jtag),
            42 => Some(RtcCh1CaptEv::RtcUpd),
            41 => Some(RtcCh1CaptEv::RtcCombDly),
            40 => Some(RtcCh1CaptEv::RtcCh2Dly),
            39 => Some(RtcCh1CaptEv::RtcCh1Dly),
            38 => Some(RtcCh1CaptEv::RtcCh0Dly),
            37 => Some(RtcCh1CaptEv::RtcCh2),
            36 => Some(RtcCh1CaptEv::RtcCh1),
            35 => Some(RtcCh1CaptEv::RtcCh0),
            32 => Some(RtcCh1CaptEv::Pad),
            9 => Some(RtcCh1CaptEv::BatmonCombined),
            8 => Some(RtcCh1CaptEv::BatmonTempLl),
            7 => Some(RtcCh1CaptEv::BatmonTempUl),
            6 => Some(RtcCh1CaptEv::BatmonBattLl),
            5 => Some(RtcCh1CaptEv::BatmonBattUl),
            4 => Some(RtcCh1CaptEv::AuxTimer2Ev3),
            3 => Some(RtcCh1CaptEv::AuxTimer2Ev2),
            2 => Some(RtcCh1CaptEv::AuxTimer2Ev1),
            1 => Some(RtcCh1CaptEv::AuxTimer2Ev0),
            0 => Some(RtcCh1CaptEv::IoevRtc),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RtcCh1CaptEv::None
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompbAsyncN
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompbAsync
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_volt(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonVolt
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonTemp
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer1Ev
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer0Ev
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTdcDone
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == RtcCh1CaptEv::AuxAdcDone
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompb
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == RtcCh1CaptEv::AuxCompa
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev2(&self) -> bool {
        *self == RtcCh1CaptEv::AuxSwev2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == RtcCh1CaptEv::AuxSwev1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == RtcCh1CaptEv::AuxSwev0
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_jtag(&self) -> bool {
        *self == RtcCh1CaptEv::Jtag
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_upd(&self) -> bool {
        *self == RtcCh1CaptEv::RtcUpd
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCombDly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh2Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh1Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh0Dly
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == RtcCh1CaptEv::RtcCh0
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_pad(&self) -> bool {
        *self == RtcCh1CaptEv::Pad
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_combined(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonCombined
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp_ll(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonTempLl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_temp_ul(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonTempUl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_batt_ll(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonBattLl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_batmon_batt_ul(&self) -> bool {
        *self == RtcCh1CaptEv::BatmonBattUl
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer2Ev3
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer2Ev2
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer2Ev1
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == RtcCh1CaptEv::AuxTimer2Ev0
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    #[inline(always)]
    pub fn is_ioev_rtc(&self) -> bool {
        *self == RtcCh1CaptEv::IoevRtc
    }
}
#[doc = "Field `RTC_CH1_CAPT_EV` writer - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
pub type RtcCh1CaptEvW<'a, REG> = crate::FieldWriter<'a, REG, 6, RtcCh1CaptEv>;
impl<'a, REG> RtcCh1CaptEvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::None)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async_n(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompbAsyncN)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb_async(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompbAsync)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_volt(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonVolt)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonTemp)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer1Ev)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer0Ev)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTdcDone)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxAdcDone)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompb)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxCompa)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxSwev2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxSwev1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxSwev0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn jtag(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Jtag)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcUpd)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_comb_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCombDly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh2Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh1Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0_dly(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh0Dly)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn rtc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::RtcCh0)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn pad(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::Pad)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_combined(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonCombined)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ll(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonTempLl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_temp_ul(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonTempUl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ll(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonBattLl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn batmon_batt_ul(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::BatmonBattUl)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer2Ev3)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer2Ev2)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer2Ev1)
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::AuxTimer2Ev0)
    }
    #[doc = "Edge detect IO event from the DIO(s) which have enabled contribution to IOEV_RTC in \\[MCU_IOC:IOCFGx.IOEV_RTC_EN\\]"]
    #[inline(always)]
    pub fn ioev_rtc(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCh1CaptEv::IoevRtc)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    pub fn rtc_ch1_capt_ev(&self) -> RtcCh1CaptEvR {
        RtcCh1CaptEvR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ch1_capt_ev(&mut self) -> RtcCh1CaptEvW<RtcselSpec> {
        RtcCh1CaptEvW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<RtcselSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcselSpec;
impl crate::RegisterSpec for RtcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsel::R`](R) reader structure"]
impl crate::Readable for RtcselSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcsel::W`](W) writer structure"]
impl crate::Writable for RtcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSEL to value 0x3f"]
impl crate::Resettable for RtcselSpec {
    const RESET_VALUE: u32 = 0x3f;
}
