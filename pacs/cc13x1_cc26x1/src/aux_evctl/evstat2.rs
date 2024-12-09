#[doc = "Register `EVSTAT2` reader"]
pub type R = crate::R<Evstat2Spec>;
#[doc = "Register `EVSTAT2` writer"]
pub type W = crate::W<Evstat2Spec>;
#[doc = "Field `MANUAL_EV` reader - 0:0\\]
Programmable event. See MANUAL for description."]
pub type ManualEvR = crate::BitReader;
#[doc = "Field `AON_RTC_CH2` reader - 1:1\\]
AON_RTC:EVFLAGS.CH2."]
pub type AonRtcCh2R = crate::BitReader;
#[doc = "Field `AON_RTC_CH2_DLY` reader - 2:2\\]
AON_RTC:EVFLAGS.CH2 delayed by AON_RTC:CTL.EV_DELAY configuration."]
pub type AonRtcCh2DlyR = crate::BitReader;
#[doc = "Field `AON_RTC_4KHZ` reader - 3:3\\]
AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
pub type AonRtc4khzR = crate::BitReader;
#[doc = "Field `RESERVED4` reader - 5:4\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCLK_LF` reader - 6:6\\]
SCLK_LF clock"]
pub type SclkLfR = crate::BitReader;
#[doc = "Field `PWR_DWN` reader - 7:7\\]
Event is high while system(MCU, AUX, or JTAG domains) is in powerdown (uLDO power supply)."]
pub type PwrDwnR = crate::BitReader;
#[doc = "Field `MCU_ACTIVE` reader - 8:8\\]
Event is high while system(MCU, AUX, or JTAG domains) is active or transitions to active (GLDO or DCDC power supply state). Event is not high during VDDR recharge."]
pub type McuActiveR = crate::BitReader;
#[doc = "Field `VDDR_RECHARGE` reader - 9:9\\]
Event is high during VDDR recharge."]
pub type VddrRechargeR = crate::BitReader;
#[doc = "Field `ACLK_REF` reader - 10:10\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_SYSIF:TDCREFCLKCTL.REQ."]
pub type AclkRefR = crate::BitReader;
#[doc = "Field `MCU_EV` reader - 11:11\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type McuEvR = crate::BitReader;
#[doc = "Field `MCU_OBSMUX0` reader - 12:12\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
pub type McuObsmux0R = crate::BitReader;
#[doc = "Field `MCU_OBSMUX1` reader - 13:13\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub type McuObsmux1R = crate::BitReader;
#[doc = "Field `AUX_COMPA` reader - 14:14\\]
Comparator A output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPA_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxCompaR = crate::BitReader;
#[doc = "Field `AUX_COMPB` reader - 15:15\\]
Comparator B output. Configuration of AUX_SYSIF:EVSYNCRATE.AUX_COMPB_SYNC_RATE sets the synchronization rate for this event."]
pub type AuxCompbR = crate::BitReader;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
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
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 3) as u8)
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
impl W {}
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
