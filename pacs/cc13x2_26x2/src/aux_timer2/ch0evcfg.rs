#[doc = "Register `CH0EVCFG` reader"]
pub type R = crate::R<Ch0evcfgSpec>;
#[doc = "Register `CH0EVCFG` writer"]
pub type W = crate::W<Ch0evcfgSpec>;
#[doc = "3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact {
    #[doc = "15: Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    PulseOnCmp = 15,
    #[doc = "14: Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE."]
    TglOnCmp = 14,
    #[doc = "13: Set on compare repeatedly. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE."]
    SetOnCmp = 13,
    #[doc = "12: Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE."]
    ClrOnCmp = 12,
    #[doc = "11: Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE &lt;= TARGET.VALUE: Duty cycle = CH0CC.VALUE / ( TARGET.VALUE + 1 ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    SetOn0TglOnCmp = 11,
    #[doc = "10: Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE &lt;= TARGET.VALUE: Duty cycle = 1 - ( CH0CC.VALUE / TARGET.VALUE ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    ClrOn0TglOnCmp = 10,
    #[doc = "9: Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SetOnCapt = 9,
    #[doc = "8: Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH0CCFG.CAPT_SRC relative to the signal edge given by CH0CCFG.EDGE. Set enabled events when CH0CC.VALUE contains signal period and CH0PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH0CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH0CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period &lt;= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    PerPulseWidthMeas = 8,
    #[doc = "7: Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    PulseOnCmpDis = 7,
    #[doc = "6: Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    TglOnCmpDis = 6,
    #[doc = "5: Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    SetOnCmpDis = 5,
    #[doc = "4: Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    ClrOnCmpDis = 4,
    #[doc = "3: Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    SetOn0TglOnCmpDis = 3,
    #[doc = "2: Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    ClrOn0TglOnCmpDis = 2,
    #[doc = "1: Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    SetOnCaptDis = 1,
    #[doc = "0: Disable channel."]
    Dis = 0,
}
impl From<Ccact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact {}
#[doc = "Field `CCACT` reader - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
pub type CcactR = crate::FieldReader<Ccact>;
impl CcactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact {
        match self.bits {
            15 => Ccact::PulseOnCmp,
            14 => Ccact::TglOnCmp,
            13 => Ccact::SetOnCmp,
            12 => Ccact::ClrOnCmp,
            11 => Ccact::SetOn0TglOnCmp,
            10 => Ccact::ClrOn0TglOnCmp,
            9 => Ccact::SetOnCapt,
            8 => Ccact::PerPulseWidthMeas,
            7 => Ccact::PulseOnCmpDis,
            6 => Ccact::TglOnCmpDis,
            5 => Ccact::SetOnCmpDis,
            4 => Ccact::ClrOnCmpDis,
            3 => Ccact::SetOn0TglOnCmpDis,
            2 => Ccact::ClrOn0TglOnCmpDis,
            1 => Ccact::SetOnCaptDis,
            0 => Ccact::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn is_pulse_on_cmp(&self) -> bool {
        *self == Ccact::PulseOnCmp
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn is_tgl_on_cmp(&self) -> bool {
        *self == Ccact::TglOnCmp
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn is_set_on_cmp(&self) -> bool {
        *self == Ccact::SetOnCmp
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn is_clr_on_cmp(&self) -> bool {
        *self == Ccact::ClrOnCmp
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE &lt;= TARGET.VALUE: Duty cycle = CH0CC.VALUE / ( TARGET.VALUE + 1 ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp(&self) -> bool {
        *self == Ccact::SetOn0TglOnCmp
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE &lt;= TARGET.VALUE: Duty cycle = 1 - ( CH0CC.VALUE / TARGET.VALUE ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp(&self) -> bool {
        *self == Ccact::ClrOn0TglOnCmp
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn is_set_on_capt(&self) -> bool {
        *self == Ccact::SetOnCapt
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH0CCFG.CAPT_SRC relative to the signal edge given by CH0CCFG.EDGE. Set enabled events when CH0CC.VALUE contains signal period and CH0PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH0CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH0CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period &lt;= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    #[inline(always)]
    pub fn is_per_pulse_width_meas(&self) -> bool {
        *self == Ccact::PerPulseWidthMeas
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn is_pulse_on_cmp_dis(&self) -> bool {
        *self == Ccact::PulseOnCmpDis
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn is_tgl_on_cmp_dis(&self) -> bool {
        *self == Ccact::TglOnCmpDis
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn is_set_on_cmp_dis(&self) -> bool {
        *self == Ccact::SetOnCmpDis
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn is_clr_on_cmp_dis(&self) -> bool {
        *self == Ccact::ClrOnCmpDis
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == Ccact::SetOn0TglOnCmpDis
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == Ccact::ClrOn0TglOnCmpDis
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn is_set_on_capt_dis(&self) -> bool {
        *self == Ccact::SetOnCaptDis
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ccact::Dis
    }
}
#[doc = "Field `CCACT` writer - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
pub type CcactW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccact, crate::Safe>;
impl<'a, REG> CcactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::PulseOnCmp)
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn tgl_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::TglOnCmp)
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn set_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCmp)
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE."]
    #[inline(always)]
    pub fn clr_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOnCmp)
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE &lt;= TARGET.VALUE: Duty cycle = CH0CC.VALUE / ( TARGET.VALUE + 1 ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 1. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOn0TglOnCmp)
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When CH0CC.VALUE &lt;= TARGET.VALUE: Duty cycle = 1 - ( CH0CC.VALUE / TARGET.VALUE ). When CH0CC.VALUE > TARGET.VALUE: Duty cycle = 0. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOn0TglOnCmp)
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Select this function with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you enable events. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn set_on_capt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCapt)
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by CH0CCFG.CAPT_SRC relative to the signal edge given by CH0CCFG.EDGE. Set enabled events when CH0CC.VALUE contains signal period and CH0PCC.VALUE contains signal pulse width. Notes: - Make sure that you configure CH0CCFG.CAPT_SRC and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when CH0CC.VALUE contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - If you want to observe a timeout event configure another channel to SET_ON_CAPT. Signal property requirements: - Signal Period >= 2 * ( 1 + PRECFG.CLKDIV ) * timer clock period. - Signal Period &lt;= 65535 * (1 + PRECFG.CLKDIV ) * timer clock period. - Signal low and high phase >= (1 + PRECFG.CLKDIV ) * timer clock period."]
    #[inline(always)]
    pub fn per_pulse_width_meas(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::PerPulseWidthMeas)
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. The event is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::PulseOnCmpDis)
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn tgl_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::TglOnCmpDis)
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn set_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCmpDis)
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel."]
    #[inline(always)]
    pub fn clr_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOnCmpDis)
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are cleared when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOn0TglOnCmpDis)
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled events when CNTR.VALUE = 0. - Toggle enabled events when CH0CC.VALUE = CNTR.VALUE. - Disable channel. Enabled events are set when CH0CC.VALUE = 0 and CNTR.VALUE = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOn0TglOnCmpDis)
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled events on capture event and copy CNTR.VALUE to CH0CC.VALUE. - Disable channel. Primary use scenario is to select this function before you start the timer. Follow these steps if you need to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no event enable. - Configure CH0CCFG (optional). - Wait for three timer clock periods as defined in PRECFG before you set CCACT to SET_ON_CAPT_DIS. Event enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub fn set_on_capt_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCaptDis)
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::Dis)
    }
}
#[doc = "Field `EV0_GEN` reader - 4:4\\]
Event 0 enable. 0: Channel 0 does not control event 0. 1: Channel 0 controls event 0. When 0 &lt; CCACT &lt; 8, EV0_GEN becomes zero after a capture or compare event."]
pub type Ev0GenR = crate::BitReader;
#[doc = "Field `EV0_GEN` writer - 4:4\\]
Event 0 enable. 0: Channel 0 does not control event 0. 1: Channel 0 controls event 0. When 0 &lt; CCACT &lt; 8, EV0_GEN becomes zero after a capture or compare event."]
pub type Ev0GenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV1_GEN` reader - 5:5\\]
Event 1 enable. 0: Channel 0 does not control event 1. 1: Channel 0 controls event 1. When 0 &lt; CCACT &lt; 8, EV1_GEN becomes zero after a capture or compare event."]
pub type Ev1GenR = crate::BitReader;
#[doc = "Field `EV1_GEN` writer - 5:5\\]
Event 1 enable. 0: Channel 0 does not control event 1. 1: Channel 0 controls event 1. When 0 &lt; CCACT &lt; 8, EV1_GEN becomes zero after a capture or compare event."]
pub type Ev1GenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV2_GEN` reader - 6:6\\]
Event 2 enable. 0: Channel 0 does not control event 2. 1: Channel 0 controls event 2. When 0 &lt; CCACT &lt; 8, EV2_GEN becomes zero after a capture or compare event."]
pub type Ev2GenR = crate::BitReader;
#[doc = "Field `EV2_GEN` writer - 6:6\\]
Event 2 enable. 0: Channel 0 does not control event 2. 1: Channel 0 controls event 2. When 0 &lt; CCACT &lt; 8, EV2_GEN becomes zero after a capture or compare event."]
pub type Ev2GenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV3_GEN` reader - 7:7\\]
Event 3 enable. 0: Channel 0 does not control event 3. 1: Channel 0 controls event 3. When 0 &lt; CCACT &lt; 8, EV3_GEN becomes zero after a capture or compare event."]
pub type Ev3GenR = crate::BitReader;
#[doc = "Field `EV3_GEN` writer - 7:7\\]
Event 3 enable. 0: Channel 0 does not control event 3. 1: Channel 0 controls event 3. When 0 &lt; CCACT &lt; 8, EV3_GEN becomes zero after a capture or compare event."]
pub type Ev3GenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline(always)]
    pub fn ccact(&self) -> CcactR {
        CcactR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Event 0 enable. 0: Channel 0 does not control event 0. 1: Channel 0 controls event 0. When 0 &lt; CCACT &lt; 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev0_gen(&self) -> Ev0GenR {
        Ev0GenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event 1 enable. 0: Channel 0 does not control event 1. 1: Channel 0 controls event 1. When 0 &lt; CCACT &lt; 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev1_gen(&self) -> Ev1GenR {
        Ev1GenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Event 2 enable. 0: Channel 0 does not control event 2. 1: Channel 0 controls event 2. When 0 &lt; CCACT &lt; 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev2_gen(&self) -> Ev2GenR {
        Ev2GenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event 3 enable. 0: Channel 0 does not control event 3. 1: Channel 0 controls event 3. When 0 &lt; CCACT &lt; 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn ev3_gen(&self) -> Ev3GenR {
        Ev3GenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events."]
    #[inline(always)]
    #[must_use]
    pub fn ccact(&mut self) -> CcactW<Ch0evcfgSpec> {
        CcactW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Event 0 enable. 0: Channel 0 does not control event 0. 1: Channel 0 controls event 0. When 0 &lt; CCACT &lt; 8, EV0_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_gen(&mut self) -> Ev0GenW<Ch0evcfgSpec> {
        Ev0GenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Event 1 enable. 0: Channel 0 does not control event 1. 1: Channel 0 controls event 1. When 0 &lt; CCACT &lt; 8, EV1_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_gen(&mut self) -> Ev1GenW<Ch0evcfgSpec> {
        Ev1GenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Event 2 enable. 0: Channel 0 does not control event 2. 1: Channel 0 controls event 2. When 0 &lt; CCACT &lt; 8, EV2_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev2_gen(&mut self) -> Ev2GenW<Ch0evcfgSpec> {
        Ev2GenW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Event 3 enable. 0: Channel 0 does not control event 3. 1: Channel 0 controls event 3. When 0 &lt; CCACT &lt; 8, EV3_GEN becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn ev3_gen(&mut self) -> Ev3GenW<Ch0evcfgSpec> {
        Ev3GenW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Ch0evcfgSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0evcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0evcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0evcfgSpec;
impl crate::RegisterSpec for Ch0evcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0evcfg::R`](R) reader structure"]
impl crate::Readable for Ch0evcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0evcfg::W`](W) writer structure"]
impl crate::Writable for Ch0evcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0EVCFG to value 0"]
impl crate::Resettable for Ch0evcfgSpec {
    const RESET_VALUE: u32 = 0;
}
