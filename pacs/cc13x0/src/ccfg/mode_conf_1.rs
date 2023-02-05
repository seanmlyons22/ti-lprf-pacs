#[doc = "Register `MODE_CONF_1` reader"]
pub struct R(crate::R<MODE_CONF_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_CONF_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_CONF_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_CONF_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_CONF_1` writer"]
pub struct W(crate::W<MODE_CONF_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_CONF_1_SPEC>;
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
impl From<crate::W<MODE_CONF_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_CONF_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_MAX_START` reader - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
pub type XOSC_MAX_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSC_MAX_START` writer - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
pub type XOSC_MAX_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DELTA_IBIAS_OFFSET` reader - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
pub type DELTA_IBIAS_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA_IBIAS_OFFSET` writer - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
pub type DELTA_IBIAS_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `DELTA_IBIAS_INIT` reader - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
pub type DELTA_IBIAS_INIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA_IBIAS_INIT` writer - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
pub type DELTA_IBIAS_INIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALT_DCDC_IPEAK` reader - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
pub type ALT_DCDC_IPEAK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALT_DCDC_IPEAK` writer - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
pub type ALT_DCDC_IPEAK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ALT_DCDC_DITHER_EN` reader - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
pub type ALT_DCDC_DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALT_DCDC_DITHER_EN` writer - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
pub type ALT_DCDC_DITHER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MODE_CONF_1_SPEC, bool, O>;
#[doc = "Field `ALT_DCDC_VMIN` reader - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type ALT_DCDC_VMIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALT_DCDC_VMIN` writer - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type ALT_DCDC_VMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    pub fn xosc_max_start(&self) -> XOSC_MAX_START_R {
        XOSC_MAX_START_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline(always)]
    pub fn delta_ibias_offset(&self) -> DELTA_IBIAS_OFFSET_R {
        DELTA_IBIAS_OFFSET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline(always)]
    pub fn delta_ibias_init(&self) -> DELTA_IBIAS_INIT_R {
        DELTA_IBIAS_INIT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline(always)]
    pub fn alt_dcdc_ipeak(&self) -> ALT_DCDC_IPEAK_R {
        ALT_DCDC_IPEAK_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline(always)]
    pub fn alt_dcdc_dither_en(&self) -> ALT_DCDC_DITHER_EN_R {
        ALT_DCDC_DITHER_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn alt_dcdc_vmin(&self) -> ALT_DCDC_VMIN_R {
        ALT_DCDC_VMIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_max_start(&mut self) -> XOSC_MAX_START_W<0> {
        XOSC_MAX_START_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn delta_ibias_offset(&mut self) -> DELTA_IBIAS_OFFSET_W<8> {
        DELTA_IBIAS_OFFSET_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT"]
    #[inline(always)]
    #[must_use]
    pub fn delta_ibias_init(&mut self) -> DELTA_IBIAS_INIT_W<12> {
        DELTA_IBIAS_INIT_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)"]
    #[inline(always)]
    #[must_use]
    pub fn alt_dcdc_ipeak(&mut self) -> ALT_DCDC_IPEAK_W<16> {
        ALT_DCDC_IPEAK_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable"]
    #[inline(always)]
    #[must_use]
    pub fn alt_dcdc_dither_en(&mut self) -> ALT_DCDC_DITHER_EN_W<19> {
        ALT_DCDC_DITHER_EN_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    #[must_use]
    pub fn alt_dcdc_vmin(&mut self) -> ALT_DCDC_VMIN_W<20> {
        ALT_DCDC_VMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_conf_1](index.html) module"]
pub struct MODE_CONF_1_SPEC;
impl crate::RegisterSpec for MODE_CONF_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_conf_1::R](R) reader structure"]
impl crate::Readable for MODE_CONF_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_conf_1::W](W) writer structure"]
impl crate::Writable for MODE_CONF_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE_CONF_1 to value 0xfffb_ffff"]
impl crate::Resettable for MODE_CONF_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffb_ffff;
}
