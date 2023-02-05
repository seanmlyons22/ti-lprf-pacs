#[doc = "Register `CH1EVCFG` reader"]
pub struct R(crate::R<CH1EVCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1EVCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1EVCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1EVCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1EVCFG` writer"]
pub struct W(crate::W<CH1EVCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1EVCFG_SPEC>;
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
impl From<crate::W<CH1EVCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1EVCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCACT` reader - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
pub type CCACT_R = crate::FieldReader<u8, CCACT_A>;
#[doc = "3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCACT_A {
    #[doc = "15: Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH1CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    PULSE_ON_CMP = 15,
    #[doc = "14: Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE."]
    TGL_ON_CMP = 14,
    #[doc = "13: Set on compare repeatedly. Channel function sequence: - Set enabled events when CH1CC.VALUE = CNTR.VALUE."]
    SET_ON_CMP = 13,
    #[doc = "12: Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH1CC.VALUE = CNTR.VALUE."]
    CLR_ON_CMP = 12,
    #[doc = "11: Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH1CC.VALUE <= TARGET.VALUE: Duty cycle = CH1CC.VALUE / ( TARGET.VALUE + 1 ). When CH1CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP = 11,
    #[doc = "10: Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH1CC.VALUE <= TARGET.VALUE: Duty cycle = 1 - ( CH1CC.VALUE / TARGET.VALUE ). When CH1CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP = 10,
    #[doc = "9: Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH1CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH1CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT = 9,
    #[doc = "8: Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH1CCFG.CAPT_SRC relative to the signal edge given by CH1CCFG.EDGE. Set enabled events when CH1CC.VALUE contains signal period and CH1PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH1CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH1CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    PER_PULSE_WIDTH_MEAS = 8,
    #[doc = "7: Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    PULSE_ON_CMP_DIS = 7,
    #[doc = "6: Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel."]
    TGL_ON_CMP_DIS = 6,
    #[doc = "5: Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel."]
    SET_ON_CMP_DIS = 5,
    #[doc = "4: Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel."]
    CLR_ON_CMP_DIS = 4,
    #[doc = "3: Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    SET_ON_0_TGL_ON_CMP_DIS = 3,
    #[doc = "2: Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    CLR_ON_0_TGL_ON_CMP_DIS = 2,
    #[doc = "1: Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH1CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH1CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SET_ON_CAPT_DIS = 1,
    #[doc = "0: Disable channel."]
    DIS = 0,
}
impl From<CCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: CCACT_A) -> Self {
        variant as _
    }
}
impl CCACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCACT_A {
        match self.bits {
            15 => CCACT_A::PULSE_ON_CMP,
            14 => CCACT_A::TGL_ON_CMP,
            13 => CCACT_A::SET_ON_CMP,
            12 => CCACT_A::CLR_ON_CMP,
            11 => CCACT_A::SET_ON_0_TGL_ON_CMP,
            10 => CCACT_A::CLR_ON_0_TGL_ON_CMP,
            9 => CCACT_A::SET_ON_CAPT,
            8 => CCACT_A::PER_PULSE_WIDTH_MEAS,
            7 => CCACT_A::PULSE_ON_CMP_DIS,
            6 => CCACT_A::TGL_ON_CMP_DIS,
            5 => CCACT_A::SET_ON_CMP_DIS,
            4 => CCACT_A::CLR_ON_CMP_DIS,
            3 => CCACT_A::SET_ON_0_TGL_ON_CMP_DIS,
            2 => CCACT_A::CLR_ON_0_TGL_ON_CMP_DIS,
            1 => CCACT_A::SET_ON_CAPT_DIS,
            0 => CCACT_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE_ON_CMP`"]
    #[inline(always)]
    pub fn is_pulse_on_cmp(&self) -> bool {
        *self == CCACT_A::PULSE_ON_CMP
    }
    #[doc = "Checks if the value of the field is `TGL_ON_CMP`"]
    #[inline(always)]
    pub fn is_tgl_on_cmp(&self) -> bool {
        *self == CCACT_A::TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_CMP`"]
    #[inline(always)]
    pub fn is_set_on_cmp(&self) -> bool {
        *self == CCACT_A::SET_ON_CMP
    }
    #[doc = "Checks if the value of the field is `CLR_ON_CMP`"]
    #[inline(always)]
    pub fn is_clr_on_cmp(&self) -> bool {
        *self == CCACT_A::CLR_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_0_TGL_ON_CMP`"]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp(&self) -> bool {
        *self == CCACT_A::SET_ON_0_TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `CLR_ON_0_TGL_ON_CMP`"]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp(&self) -> bool {
        *self == CCACT_A::CLR_ON_0_TGL_ON_CMP
    }
    #[doc = "Checks if the value of the field is `SET_ON_CAPT`"]
    #[inline(always)]
    pub fn is_set_on_capt(&self) -> bool {
        *self == CCACT_A::SET_ON_CAPT
    }
    #[doc = "Checks if the value of the field is `PER_PULSE_WIDTH_MEAS`"]
    #[inline(always)]
    pub fn is_per_pulse_width_meas(&self) -> bool {
        *self == CCACT_A::PER_PULSE_WIDTH_MEAS
    }
    #[doc = "Checks if the value of the field is `PULSE_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_pulse_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::PULSE_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `TGL_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_set_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::SET_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `CLR_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_clr_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::CLR_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_0_TGL_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::SET_ON_0_TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `CLR_ON_0_TGL_ON_CMP_DIS`"]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == CCACT_A::CLR_ON_0_TGL_ON_CMP_DIS
    }
    #[doc = "Checks if the value of the field is `SET_ON_CAPT_DIS`"]
    #[inline(always)]
    pub fn is_set_on_capt_dis(&self) -> bool {
        *self == CCACT_A::SET_ON_CAPT_DIS
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CCACT_A::DIS
    }
}
#[doc = "Field `CCACT` writer - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
pub type CCACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CH1EVCFG_SPEC, u8, CCACT_A, 4, O>;
impl<'a, const O: u8> CCACT_W<'a, O> {
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH1CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::PULSE_ON_CMP)
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::TGL_ON_CMP)
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled events when CH1CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn set_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CMP)
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH1CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn clr_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_CMP)
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH1CC.VALUE <= TARGET.VALUE: Duty cycle = CH1CC.VALUE / ( TARGET.VALUE + 1 ). When CH1CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_0_TGL_ON_CMP)
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH1CC.VALUE <= TARGET.VALUE: Duty cycle = 1 - ( CH1CC.VALUE / TARGET.VALUE ). When CH1CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_0_TGL_ON_CMP)
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH1CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH1CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn set_on_capt(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CAPT)
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH1CCFG.CAPT_SRC relative to the signal edge given by CH1CCFG.EDGE. Set enabled events when CH1CC.VALUE contains signal period and CH1PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH1CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH1CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period <= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    #[inline(always)]
    pub fn per_pulse_width_meas(self) -> &'a mut W {
        self.variant(CCACT_A::PER_PULSE_WIDTH_MEAS)
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::PULSE_ON_CMP_DIS)
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::TGL_ON_CMP_DIS)
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn set_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CMP_DIS)
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn clr_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_CMP_DIS)
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_0_TGL_ON_CMP_DIS)
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH1CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH1CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp_dis(self) -> &'a mut W {
        self.variant(CCACT_A::CLR_ON_0_TGL_ON_CMP_DIS)
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH1CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH1CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn set_on_capt_dis(self) -> &'a mut W {
        self.variant(CCACT_A::SET_ON_CAPT_DIS)
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CCACT_A::DIS)
    }
}
#[doc = "Field `EV0_GEN` reader - 4:4\\]
Event 0 enable. 0: Channel 1 does not control event 0. 1: Channel 1 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
pub type EV0_GEN_R = crate::BitReader<bool>;
#[doc = "Field `EV0_GEN` writer - 4:4\\]
Event 0 enable. 0: Channel 1 does not control event 0. 1: Channel 1 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
pub type EV0_GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1EVCFG_SPEC, bool, O>;
#[doc = "Field `EV1_GEN` reader - 5:5\\]
Event 1 enable. 0: Channel 1 does not control event 1. 1: Channel 1 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
pub type EV1_GEN_R = crate::BitReader<bool>;
#[doc = "Field `EV1_GEN` writer - 5:5\\]
Event 1 enable. 0: Channel 1 does not control event 1. 1: Channel 1 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
pub type EV1_GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1EVCFG_SPEC, bool, O>;
#[doc = "Field `EV2_GEN` reader - 6:6\\]
Event 2 enable. 0: Channel 1 does not control event 2. 1: Channel 1 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
pub type EV2_GEN_R = crate::BitReader<bool>;
#[doc = "Field `EV2_GEN` writer - 6:6\\]
Event 2 enable. 0: Channel 1 does not control event 2. 1: Channel 1 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
pub type EV2_GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1EVCFG_SPEC, bool, O>;
#[doc = "Field `EV3_GEN` reader - 7:7\\]
Event 3 enable. 0: Channel 1 does not control event 3. 1: Channel 1 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
pub type EV3_GEN_R = crate::BitReader<bool>;
#[doc = "Field `EV3_GEN` writer - 7:7\\]
Event 3 enable. 0: Channel 1 does not control event 3. 1: Channel 1 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
pub type EV3_GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1EVCFG_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1EVCFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline(always)]
    pub fn ccact(&self) -> CCACT_R {
        CCACT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Event 0 enable. 0: Channel 1 does not control event 0. 1: Channel 1 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev0_gen(&self) -> EV0_GEN_R {
        EV0_GEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event 1 enable. 0: Channel 1 does not control event 1. 1: Channel 1 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev1_gen(&self) -> EV1_GEN_R {
        EV1_GEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Event 2 enable. 0: Channel 1 does not control event 2. 1: Channel 1 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev2_gen(&self) -> EV2_GEN_R {
        EV2_GEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event 3 enable. 0: Channel 1 does not control event 3. 1: Channel 1 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev3_gen(&self) -> EV3_GEN_R {
        EV3_GEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline(always)]
    #[must_use]
    pub fn ccact(&mut self) -> CCACT_W<0> {
        CCACT_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Event 0 enable. 0: Channel 1 does not control event 0. 1: Channel 1 controls event 0. When 0 < CCACT < 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_gen(&mut self) -> EV0_GEN_W<4> {
        EV0_GEN_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Event 1 enable. 0: Channel 1 does not control event 1. 1: Channel 1 controls event 1. When 0 < CCACT < 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_gen(&mut self) -> EV1_GEN_W<5> {
        EV1_GEN_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Event 2 enable. 0: Channel 1 does not control event 2. 1: Channel 1 controls event 2. When 0 < CCACT < 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev2_gen(&mut self) -> EV2_GEN_W<6> {
        EV2_GEN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Event 3 enable. 0: Channel 1 does not control event 3. 1: Channel 1 controls event 3. When 0 < CCACT < 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev3_gen(&mut self) -> EV3_GEN_W<7> {
        EV3_GEN_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1evcfg](index.html) module"]
pub struct CH1EVCFG_SPEC;
impl crate::RegisterSpec for CH1EVCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1evcfg::R](R) reader structure"]
impl crate::Readable for CH1EVCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1evcfg::W](W) writer structure"]
impl crate::Writable for CH1EVCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1EVCFG to value 0"]
impl crate::Resettable for CH1EVCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
