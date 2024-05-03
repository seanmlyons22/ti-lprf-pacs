#[doc = "Register `EVSTAT2` reader"]
pub type R = crate::R<Evstat2Spec>;
#[doc = "Register `EVSTAT2` writer"]
pub type W = crate::W<Evstat2Spec>;
#[doc = "Field `MANUAL_EV` reader - 0:0\\]
Programmable event. See MANUAL for description."]
pub type ManualEvR = crate::BitReader;
#[doc = "Field `MANUAL_EV` writer - 0:0\\]
Programmable event. See MANUAL for description."]
pub type ManualEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_RTC_CH2` reader - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
pub type AonRtcCh2R = crate::BitReader;
#[doc = "Field `AON_RTC_CH2` writer - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
pub type AonRtcCh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_RTC_CH2_DLY` reader - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
pub type AonRtcCh2DlyR = crate::BitReader;
#[doc = "Field `AON_RTC_CH2_DLY` writer - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
pub type AonRtcCh2DlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_RTC_4KHZ` reader - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
pub type AonRtc4khzR = crate::BitReader;
#[doc = "Field `AON_RTC_4KHZ` writer - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
pub type AonRtc4khzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_BATMON_BAT_UPD` reader - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
pub type AonBatmonBatUpdR = crate::BitReader;
#[doc = "Field `AON_BATMON_BAT_UPD` writer - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
pub type AonBatmonBatUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_BATMON_TEMP_UPD` reader - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
pub type AonBatmonTempUpdR = crate::BitReader;
#[doc = "Field `AON_BATMON_TEMP_UPD` writer - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
pub type AonBatmonTempUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_LF` reader - 6:6\\]
SCLK_LF clock"]
pub type SclkLfR = crate::BitReader;
#[doc = "Field `SCLK_LF` writer - 6:6\\]
SCLK_LF clock"]
pub type SclkLfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR_DWN` reader - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
pub type PwrDwnR = crate::BitReader;
#[doc = "Field `PWR_DWN` writer - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
pub type PwrDwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_ACTIVE` reader - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
pub type McuActiveR = crate::BitReader;
#[doc = "Field `MCU_ACTIVE` writer - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
pub type McuActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDR_RECHARGE` reader - 9:9\\]
Event is high during VDDR recharge."]
pub type VddrRechargeR = crate::BitReader;
#[doc = "Field `VDDR_RECHARGE` writer - 9:9\\]
Event is high during VDDR recharge."]
pub type VddrRechargeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_REF` reader - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
pub type AclkRefR = crate::BitReader;
#[doc = "Field `ACLK_REF` writer - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
pub type AclkRefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_EV` reader - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type McuEvR = crate::BitReader;
#[doc = "Field `MCU_EV` writer - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type McuEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_OBSMUX0` reader - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type McuObsmux0R = crate::BitReader;
#[doc = "Field `MCU_OBSMUX0` writer - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type McuObsmux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_OBSMUX1` reader - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type McuObsmux1R = crate::BitReader;
#[doc = "Field `MCU_OBSMUX1` writer - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type McuObsmux1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPA` reader - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxCompaR = crate::BitReader;
#[doc = "Field `AUX_COMPA` writer - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxCompaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_COMPB` reader - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxCompbR = crate::BitReader;
#[doc = "Field `AUX_COMPB` writer - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxCompbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Programmable event. See MANUAL for description."]
    #[inline(always)]
    pub fn manual_ev(&self) -> ManualEvR {
        ManualEvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AonRtcCh2R {
        AonRtcCh2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(&self) -> AonRtcCh2DlyR {
        AonRtcCh2DlyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    pub fn aon_rtc_4khz(&self) -> AonRtc4khzR {
        AonRtc4khzR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(&self) -> AonBatmonBatUpdR {
        AonBatmonBatUpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(&self) -> AonBatmonTempUpdR {
        AonBatmonTempUpdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF clock"]
    #[inline(always)]
    pub fn sclk_lf(&self) -> SclkLfR {
        SclkLfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
    #[inline(always)]
    pub fn pwr_dwn(&self) -> PwrDwnR {
        PwrDwnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
    #[inline(always)]
    pub fn mcu_active(&self) -> McuActiveR {
        McuActiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Event is high during VDDR recharge."]
    #[inline(always)]
    pub fn vddr_recharge(&self) -> VddrRechargeR {
        VddrRechargeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&self) -> AclkRefR {
        AclkRefR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&self) -> McuEvR {
        McuEvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> McuObsmux0R {
        McuObsmux0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn mcu_obsmux1(&self) -> McuObsmux1R {
        McuObsmux1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AuxCompaR {
        AuxCompaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AuxCompbR {
        AuxCompbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Programmable event. See MANUAL for description."]
    #[inline(always)]
    #[must_use]
    pub fn manual_ev(&mut self) -> ManualEvW<Evstat2Spec> {
        ManualEvW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2(&mut self) -> AonRtcCh2W<Evstat2Spec> {
        AonRtcCh2W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2_dly(&mut self) -> AonRtcCh2DlyW<Evstat2Spec> {
        AonRtcCh2DlyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_4khz(&mut self) -> AonRtc4khzW<Evstat2Spec> {
        AonRtc4khzW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:BAT."]
    #[inline(always)]
    #[must_use]
    pub fn aon_batmon_bat_upd(&mut self) -> AonBatmonBatUpdW<Evstat2Spec> {
        AonBatmonBatUpdW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Event is high for two SCLK_MF clock periods when there is an update of AON_BATMON:TEMP."]
    #[inline(always)]
    #[must_use]
    pub fn aon_batmon_temp_upd(&mut self) -> AonBatmonTempUpdW<Evstat2Spec> {
        AonBatmonTempUpdW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_LF clock"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf(&mut self) -> SclkLfW<Evstat2Spec> {
        SclkLfW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn(&mut self) -> PwrDwnW<Evstat2Spec> {
        PwrDwnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_active(&mut self) -> McuActiveW<Evstat2Spec> {
        McuActiveW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Event is high during VDDR recharge."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_recharge(&mut self) -> VddrRechargeW<Evstat2Spec> {
        VddrRechargeW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
    #[inline(always)]
    #[must_use]
    pub fn aclk_ref(&mut self) -> AclkRefW<Evstat2Spec> {
        AclkRefW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ev(&mut self) -> McuEvW<Evstat2Spec> {
        McuEvW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> McuObsmux0W<Evstat2Spec> {
        McuObsmux0W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux1(&mut self) -> McuObsmux1W<Evstat2Spec> {
        McuObsmux1W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AuxCompaW<Evstat2Spec> {
        AuxCompaW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AuxCompbW<Evstat2Spec> {
        AuxCompbW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Evstat2Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat2Spec;
impl crate::RegisterSpec for Evstat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat2::R`](R) reader structure"]
impl crate::Readable for Evstat2Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat2::W`](W) writer structure"]
impl crate::Writable for Evstat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT2 to value 0"]
impl crate::Resettable for Evstat2Spec {
    const RESET_VALUE: u32 = 0;
}
