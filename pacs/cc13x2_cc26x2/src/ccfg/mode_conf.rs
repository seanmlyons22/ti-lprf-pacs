#[doc = "Register `MODE_CONF` reader"]
pub struct R(crate::R<MODE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_CONF` writer"]
pub struct W(crate::W<MODE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_CONF_SPEC>;
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
impl From<crate::W<MODE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDR_CAP` reader - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
pub type VDDR_CAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_CAP` writer - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
pub type VDDR_CAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `XOSC_CAPARRAY_DELTA` reader - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
pub type XOSC_CAPARRAY_DELTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSC_CAPARRAY_DELTA` writer - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
pub type XOSC_CAPARRAY_DELTA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `HF_COMP` reader - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_R = crate::BitReader<bool>;
#[doc = "Field `HF_COMP` writer - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `XOSC_CAP_MOD` reader - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
pub type XOSC_CAP_MOD_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_CAP_MOD` writer - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
pub type XOSC_CAP_MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `XOSC_FREQ` reader - 19:18\\]
Selects which high frequency oscillator is used (required for radio usage)."]
pub type XOSC_FREQ_R = crate::FieldReader<u8, XOSC_FREQ_A>;
#[doc = "19:18\\]
Selects which high frequency oscillator is used (required for radio usage).\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XOSC_FREQ_A {
    #[doc = "3: 24 MHz XOSC_HF. Not supported."]
    _24M = 3,
    #[doc = "2: 48 MHz XOSC_HF"]
    _48M = 2,
    #[doc = "1: Internal high precision oscillator."]
    HPOSC = 1,
    #[doc = "0: External 48 MHz TCXO. Refer to MODE_CONF_1.TCXO_MAX_START and MODE_CONF_1.TCXO_TYPE bit fields for additional configuration of TCXO."]
    TCXO = 0,
}
impl From<XOSC_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: XOSC_FREQ_A) -> Self {
        variant as _
    }
}
impl XOSC_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XOSC_FREQ_A {
        match self.bits {
            3 => XOSC_FREQ_A::_24M,
            2 => XOSC_FREQ_A::_48M,
            1 => XOSC_FREQ_A::HPOSC,
            0 => XOSC_FREQ_A::TCXO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == XOSC_FREQ_A::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        *self == XOSC_FREQ_A::_48M
    }
    #[doc = "Checks if the value of the field is `HPOSC`"]
    #[inline(always)]
    pub fn is_hposc(&self) -> bool {
        *self == XOSC_FREQ_A::HPOSC
    }
    #[doc = "Checks if the value of the field is `TCXO`"]
    #[inline(always)]
    pub fn is_tcxo(&self) -> bool {
        *self == XOSC_FREQ_A::TCXO
    }
}
#[doc = "Field `XOSC_FREQ` writer - 19:18\\]
Selects which high frequency oscillator is used (required for radio usage)."]
pub type XOSC_FREQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODE_CONF_SPEC, u8, XOSC_FREQ_A, 2, O>;
impl<'a, const O: u8> XOSC_FREQ_W<'a, O> {
    #[doc = "24 MHz XOSC_HF. Not supported."]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::_24M)
    }
    #[doc = "48 MHz XOSC_HF"]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::_48M)
    }
    #[doc = "Internal high precision oscillator."]
    #[inline(always)]
    pub fn hposc(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::HPOSC)
    }
    #[doc = "External 48 MHz TCXO. Refer to MODE_CONF_1.TCXO_MAX_START and MODE_CONF_1.TCXO_TYPE bit fields for additional configuration of TCXO."]
    #[inline(always)]
    pub fn tcxo(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::TCXO)
    }
}
#[doc = "Field `RTC_COMP` reader - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RTC_COMP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COMP` writer - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type RTC_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `VDDR_TRIM_SLEEP_TC` reader - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
pub type VDDR_TRIM_SLEEP_TC_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_TRIM_SLEEP_TC` writer - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
pub type VDDR_TRIM_SLEEP_TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `SCLK_LF_OPTION` reader - 23:22\\]
Select source for SCLK_LF."]
pub type SCLK_LF_OPTION_R = crate::FieldReader<u8, SCLK_LF_OPTION_A>;
#[doc = "23:22\\]
Select source for SCLK_LF.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLK_LF_OPTION_A {
    #[doc = "3: Low frequency RCOSC (default)"]
    RCOSC_LF = 3,
    #[doc = "2: 32.768 kHz low frequency XOSC"]
    XOSC_LF = 2,
    #[doc = "1: External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the SetupTrimDevice() driverlib boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    EXTERNAL_LF = 1,
    #[doc = "0: 31.25 kHz clock derived from 48 MHz XOSC or HPOSC. The RTC tick speed AON_RTC:SUBSECINC is updated to 0x8637BD, corresponding to a 31.25 kHz clock (done in the SetupTrimDevice() driverlib boot function). The device must be blocked from entering Standby mode when using this clock source."]
    XOSC_HF_DLF = 0,
}
impl From<SCLK_LF_OPTION_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_OPTION_A) -> Self {
        variant as _
    }
}
impl SCLK_LF_OPTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_OPTION_A {
        match self.bits {
            3 => SCLK_LF_OPTION_A::RCOSC_LF,
            2 => SCLK_LF_OPTION_A::XOSC_LF,
            1 => SCLK_LF_OPTION_A::EXTERNAL_LF,
            0 => SCLK_LF_OPTION_A::XOSC_HF_DLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCOSC_LF`"]
    #[inline(always)]
    pub fn is_rcosc_lf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::RCOSC_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_LF`"]
    #[inline(always)]
    pub fn is_xosc_lf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::XOSC_LF
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_LF`"]
    #[inline(always)]
    pub fn is_external_lf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::EXTERNAL_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_HF_DLF`"]
    #[inline(always)]
    pub fn is_xosc_hf_dlf(&self) -> bool {
        *self == SCLK_LF_OPTION_A::XOSC_HF_DLF
    }
}
#[doc = "Field `SCLK_LF_OPTION` writer - 23:22\\]
Select source for SCLK_LF."]
pub type SCLK_LF_OPTION_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODE_CONF_SPEC, u8, SCLK_LF_OPTION_A, 2, O>;
impl<'a, const O: u8> SCLK_LF_OPTION_W<'a, O> {
    #[doc = "Low frequency RCOSC (default)"]
    #[inline(always)]
    pub fn rcosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::RCOSC_LF)
    }
    #[doc = "32.768 kHz low frequency XOSC"]
    #[inline(always)]
    pub fn xosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::XOSC_LF)
    }
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the SetupTrimDevice() driverlib boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    #[inline(always)]
    pub fn external_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::EXTERNAL_LF)
    }
    #[doc = "31.25 kHz clock derived from 48 MHz XOSC or HPOSC. The RTC tick speed AON_RTC:SUBSECINC is updated to 0x8637BD, corresponding to a 31.25 kHz clock (done in the SetupTrimDevice() driverlib boot function). The device must be blocked from entering Standby mode when using this clock source."]
    #[inline(always)]
    pub fn xosc_hf_dlf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::XOSC_HF_DLF)
    }
}
#[doc = "Field `VDDS_BOD_LEVEL` reader - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum PA output power on CC13xx). 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default)."]
pub type VDDS_BOD_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `VDDS_BOD_LEVEL` writer - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum PA output power on CC13xx). 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default)."]
pub type VDDS_BOD_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `VDDR_EXT_LOAD` reader - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_LOAD_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_EXT_LOAD` writer - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VDDR_EXT_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `DCDC_ACTIVE` reader - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DCDC_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_ACTIVE` writer - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DCDC_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `DCDC_RECHARGE` reader - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DCDC_RECHARGE_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_RECHARGE` writer - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub type DCDC_RECHARGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CONF_SPEC, bool, O>;
#[doc = "Field `VDDR_TRIM_SLEEP_DELTA` reader - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
pub type VDDR_TRIM_SLEEP_DELTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_TRIM_SLEEP_DELTA` writer - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
pub type VDDR_TRIM_SLEEP_DELTA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE_CONF_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline(always)]
    pub fn vddr_cap(&self) -> VDDR_CAP_R {
        VDDR_CAP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    pub fn xosc_caparray_delta(&self) -> XOSC_CAPARRAY_DELTA_R {
        XOSC_CAPARRAY_DELTA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp(&self) -> HF_COMP_R {
        HF_COMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline(always)]
    pub fn xosc_cap_mod(&self) -> XOSC_CAP_MOD_R {
        XOSC_CAP_MOD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Selects which high frequency oscillator is used (required for radio usage)."]
    #[inline(always)]
    pub fn xosc_freq(&self) -> XOSC_FREQ_R {
        XOSC_FREQ_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp(&self) -> RTC_COMP_R {
        RTC_COMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    pub fn vddr_trim_sleep_tc(&self) -> VDDR_TRIM_SLEEP_TC_R {
        VDDR_TRIM_SLEEP_TC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Select source for SCLK_LF."]
    #[inline(always)]
    pub fn sclk_lf_option(&self) -> SCLK_LF_OPTION_R {
        SCLK_LF_OPTION_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum PA output power on CC13xx). 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default)."]
    #[inline(always)]
    pub fn vdds_bod_level(&self) -> VDDS_BOD_LEVEL_R {
        VDDS_BOD_LEVEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_load(&self) -> VDDR_EXT_LOAD_R {
        VDDR_EXT_LOAD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_recharge(&self) -> DCDC_RECHARGE_R {
        DCDC_RECHARGE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline(always)]
    pub fn vddr_trim_sleep_delta(&self) -> VDDR_TRIM_SLEEP_DELTA_R {
        VDDR_TRIM_SLEEP_DELTA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline(always)]
    #[must_use]
    pub fn vddr_cap(&mut self) -> VDDR_CAP_W<0> {
        VDDR_CAP_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_caparray_delta(&mut self) -> XOSC_CAPARRAY_DELTA_W<8> {
        XOSC_CAPARRAY_DELTA_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp(&mut self) -> HF_COMP_W<16> {
        HF_COMP_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_cap_mod(&mut self) -> XOSC_CAP_MOD_W<17> {
        XOSC_CAP_MOD_W::new(self)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Selects which high frequency oscillator is used (required for radio usage)."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_freq(&mut self) -> XOSC_FREQ_W<18> {
        XOSC_FREQ_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_comp(&mut self) -> RTC_COMP_W<20> {
        RTC_COMP_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_sleep_tc(&mut self) -> VDDR_TRIM_SLEEP_TC_W<21> {
        VDDR_TRIM_SLEEP_TC_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Select source for SCLK_LF."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_option(&mut self) -> SCLK_LF_OPTION_W<22> {
        SCLK_LF_OPTION_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum PA output power on CC13xx). 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default)."]
    #[inline(always)]
    #[must_use]
    pub fn vdds_bod_level(&mut self) -> VDDS_BOD_LEVEL_W<24> {
        VDDS_BOD_LEVEL_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_load(&mut self) -> VDDR_EXT_LOAD_W<25> {
        VDDR_EXT_LOAD_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_active(&mut self) -> DCDC_ACTIVE_W<26> {
        DCDC_ACTIVE_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_recharge(&mut self) -> DCDC_RECHARGE_W<27> {
        DCDC_RECHARGE_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_sleep_delta(&mut self) -> VDDR_TRIM_SLEEP_DELTA_W<28> {
        VDDR_TRIM_SLEEP_DELTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_conf](index.html) module"]
pub struct MODE_CONF_SPEC;
impl crate::RegisterSpec for MODE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_conf::R](R) reader structure"]
impl crate::Readable for MODE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_conf::W](W) writer structure"]
impl crate::Writable for MODE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE_CONF to value 0xffff_ffff"]
impl crate::Resettable for MODE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
