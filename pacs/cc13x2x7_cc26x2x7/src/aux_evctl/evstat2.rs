#[doc = "Register `EVSTAT2` reader"]
pub struct R(crate::R<EVSTAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT2` writer"]
pub struct W(crate::W<EVSTAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT2_SPEC>;
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
impl From<crate::W<EVSTAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MANUAL_EV` reader - 0:0\\]
Programmable event. See MANUAL for description."]
pub type MANUAL_EV_R = crate::BitReader<bool>;
#[doc = "Field `MANUAL_EV` writer - 0:0\\]
Programmable event. See MANUAL for description."]
pub type MANUAL_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AON_RTC_CH2` reader - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
pub type AON_RTC_CH2_R = crate::BitReader<bool>;
#[doc = "Field `AON_RTC_CH2` writer - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
pub type AON_RTC_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AON_RTC_CH2_DLY` reader - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
pub type AON_RTC_CH2_DLY_R = crate::BitReader<bool>;
#[doc = "Field `AON_RTC_CH2_DLY` writer - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
pub type AON_RTC_CH2_DLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AON_RTC_4KHZ` reader - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
pub type AON_RTC_4KHZ_R = crate::BitReader<bool>;
#[doc = "Field `AON_RTC_4KHZ` writer - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
pub type AON_RTC_4KHZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AON_BATMON_BAT_UPD` reader - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
pub type AON_BATMON_BAT_UPD_R = crate::BitReader<bool>;
#[doc = "Field `AON_BATMON_BAT_UPD` writer - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
pub type AON_BATMON_BAT_UPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AON_BATMON_TEMP_UPD` reader - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
pub type AON_BATMON_TEMP_UPD_R = crate::BitReader<bool>;
#[doc = "Field `AON_BATMON_TEMP_UPD` writer - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
pub type AON_BATMON_TEMP_UPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `SCLK_LF` reader - 6:6\\]
SCLK_LF clock"]
pub type SCLK_LF_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_LF` writer - 6:6\\]
SCLK_LF clock"]
pub type SCLK_LF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `PWR_DWN` reader - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
pub type PWR_DWN_R = crate::BitReader<bool>;
#[doc = "Field `PWR_DWN` writer - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
pub type PWR_DWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `MCU_ACTIVE` reader - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
pub type MCU_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `MCU_ACTIVE` writer - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
pub type MCU_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `VDDR_RECHARGE` reader - 9:9\\]
Event is high during VDDR recharge."]
pub type VDDR_RECHARGE_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_RECHARGE` writer - 9:9\\]
Event is high during VDDR recharge."]
pub type VDDR_RECHARGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `ACLK_REF` reader - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
pub type ACLK_REF_R = crate::BitReader<bool>;
#[doc = "Field `ACLK_REF` writer - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
pub type ACLK_REF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `MCU_EV` reader - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type MCU_EV_R = crate::BitReader<bool>;
#[doc = "Field `MCU_EV` writer - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type MCU_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `MCU_OBSMUX0` reader - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type MCU_OBSMUX0_R = crate::BitReader<bool>;
#[doc = "Field `MCU_OBSMUX0` writer - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type MCU_OBSMUX0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `MCU_OBSMUX1` reader - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type MCU_OBSMUX1_R = crate::BitReader<bool>;
#[doc = "Field `MCU_OBSMUX1` writer - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type MCU_OBSMUX1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AUX_COMPA` reader - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
pub type AUX_COMPA_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPA` writer - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `AUX_COMPB` reader - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
pub type AUX_COMPB_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPB` writer - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT2_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVSTAT2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Programmable event. See MANUAL for description."]
    #[inline(always)]
    pub fn manual_ev(&self) -> MANUAL_EV_R {
        MANUAL_EV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(&self) -> AON_RTC_CH2_DLY_R {
        AON_RTC_CH2_DLY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn aon_rtc_4khz(&self) -> AON_RTC_4KHZ_R {
        AON_RTC_4KHZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(&self) -> AON_BATMON_BAT_UPD_R {
        AON_BATMON_BAT_UPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(&self) -> AON_BATMON_TEMP_UPD_R {
        AON_BATMON_TEMP_UPD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF clock"]
    #[inline(always)]
    pub fn sclk_lf(&self) -> SCLK_LF_R {
        SCLK_LF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
    #[inline(always)]
    pub fn pwr_dwn(&self) -> PWR_DWN_R {
        PWR_DWN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
    #[inline(always)]
    pub fn mcu_active(&self) -> MCU_ACTIVE_R {
        MCU_ACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Event is high during VDDR recharge."]
    #[inline(always)]
    pub fn vddr_recharge(&self) -> VDDR_RECHARGE_R {
        VDDR_RECHARGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&self) -> ACLK_REF_R {
        ACLK_REF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&self) -> MCU_EV_R {
        MCU_EV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0_R {
        MCU_OBSMUX0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn mcu_obsmux1(&self) -> MCU_OBSMUX1_R {
        MCU_OBSMUX1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Programmable event. See MANUAL for description."]
    #[inline(always)]
    #[must_use]
    pub fn manual_ev(&mut self) -> MANUAL_EV_W<0> {
        MANUAL_EV_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W<1> {
        AON_RTC_CH2_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2_dly(&mut self) -> AON_RTC_CH2_DLY_W<2> {
        AON_RTC_CH2_DLY_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_4khz(&mut self) -> AON_RTC_4KHZ_W<3> {
        AON_RTC_4KHZ_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
    #[inline(always)]
    #[must_use]
    pub fn aon_batmon_bat_upd(&mut self) -> AON_BATMON_BAT_UPD_W<4> {
        AON_BATMON_BAT_UPD_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
    #[inline(always)]
    #[must_use]
    pub fn aon_batmon_temp_upd(&mut self) -> AON_BATMON_TEMP_UPD_W<5> {
        AON_BATMON_TEMP_UPD_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF clock"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf(&mut self) -> SCLK_LF_W<6> {
        SCLK_LF_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn(&mut self) -> PWR_DWN_W<7> {
        PWR_DWN_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_active(&mut self) -> MCU_ACTIVE_W<8> {
        MCU_ACTIVE_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Event is high during VDDR recharge."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_recharge(&mut self) -> VDDR_RECHARGE_W<9> {
        VDDR_RECHARGE_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
    #[inline(always)]
    #[must_use]
    pub fn aclk_ref(&mut self) -> ACLK_REF_W<10> {
        ACLK_REF_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ev(&mut self) -> MCU_EV_W<11> {
        MCU_EV_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> MCU_OBSMUX0_W<12> {
        MCU_OBSMUX0_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux1(&mut self) -> MCU_OBSMUX1_W<13> {
        MCU_OBSMUX1_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<14> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<15> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat2](index.html) module"]
pub struct EVSTAT2_SPEC;
impl crate::RegisterSpec for EVSTAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat2::R](R) reader structure"]
impl crate::Readable for EVSTAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat2::W](W) writer structure"]
impl crate::Writable for EVSTAT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSTAT2 to value 0"]
impl crate::Resettable for EVSTAT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
