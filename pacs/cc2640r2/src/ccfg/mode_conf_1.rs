#[doc = "Register `MODE_CONF_1` reader"]
pub type R = crate::R<ModeConf1Spec>;
#[doc = "Register `MODE_CONF_1` writer"]
pub type W = crate::W<ModeConf1Spec>;
#[doc = "Field `XOSC_MAX_START` reader - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
pub type XoscMaxStartR = crate::FieldReader;
#[doc = "Field `XOSC_MAX_START` writer - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
pub type XoscMaxStartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELTA_IBIAS_OFFSET` reader - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
pub type DeltaIbiasOffsetR = crate::FieldReader;
#[doc = "Field `DELTA_IBIAS_OFFSET` writer - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
pub type DeltaIbiasOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DELTA_IBIAS_INIT` reader - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
pub type DeltaIbiasInitR = crate::FieldReader;
#[doc = "Field `DELTA_IBIAS_INIT` writer - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
pub type DeltaIbiasInitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALT_DCDC_IPEAK` reader - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
pub type AltDcdcIpeakR = crate::FieldReader;
#[doc = "Field `ALT_DCDC_IPEAK` writer - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
pub type AltDcdcIpeakW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ALT_DCDC_DITHER_EN` reader - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
pub type AltDcdcDitherEnR = crate::BitReader;
#[doc = "Field `ALT_DCDC_DITHER_EN` writer - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
pub type AltDcdcDitherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALT_DCDC_VMIN` reader - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type AltDcdcVminR = crate::FieldReader;
#[doc = "Field `ALT_DCDC_VMIN` writer - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type AltDcdcVminW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    pub fn xosc_max_start(&self) -> XoscMaxStartR {
        XoscMaxStartR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline(always)]
    pub fn delta_ibias_offset(&self) -> DeltaIbiasOffsetR {
        DeltaIbiasOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline(always)]
    pub fn delta_ibias_init(&self) -> DeltaIbiasInitR {
        DeltaIbiasInitR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline(always)]
    pub fn alt_dcdc_ipeak(&self) -> AltDcdcIpeakR {
        AltDcdcIpeakR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline(always)]
    pub fn alt_dcdc_dither_en(&self) -> AltDcdcDitherEnR {
        AltDcdcDitherEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn alt_dcdc_vmin(&self) -> AltDcdcVminR {
        AltDcdcVminR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_max_start(&mut self) -> XoscMaxStartW<ModeConf1Spec> {
        XoscMaxStartW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn delta_ibias_offset(&mut self) -> DeltaIbiasOffsetW<ModeConf1Spec> {
        DeltaIbiasOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline(always)]
    #[must_use]
    pub fn delta_ibias_init(&mut self) -> DeltaIbiasInitW<ModeConf1Spec> {
        DeltaIbiasInitW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline(always)]
    #[must_use]
    pub fn alt_dcdc_ipeak(&mut self) -> AltDcdcIpeakW<ModeConf1Spec> {
        AltDcdcIpeakW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline(always)]
    #[must_use]
    pub fn alt_dcdc_dither_en(&mut self) -> AltDcdcDitherEnW<ModeConf1Spec> {
        AltDcdcDitherEnW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    #[must_use]
    pub fn alt_dcdc_vmin(&mut self) -> AltDcdcVminW<ModeConf1Spec> {
        AltDcdcVminW::new(self, 20)
    }
}
#[doc = "Mode Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_conf_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_conf_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeConf1Spec;
impl crate::RegisterSpec for ModeConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_conf_1::R`](R) reader structure"]
impl crate::Readable for ModeConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`mode_conf_1::W`](W) writer structure"]
impl crate::Writable for ModeConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE_CONF_1 to value 0xfffb_ffff"]
impl crate::Resettable for ModeConf1Spec {
    const RESET_VALUE: u32 = 0xfffb_ffff;
}
