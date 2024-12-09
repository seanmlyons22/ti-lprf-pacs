#[doc = "Register `C2CFG` reader"]
pub type R = crate::R<C2cfgSpec>;
#[doc = "Register `C2CFG` writer"]
pub type W = crate::W<C2cfgSpec>;
#[doc = "3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events. In every compare event the timer looks at the current value of CNTR. The corresponding output event will be set 1 timer period after CNTR = C2CC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact {
    #[doc = "15: Pulse on compare repeatedly. Channel function sequence: - Pulse enabled outputs when C2CC.VAL = CNTR.VAL. The output is high for two timer clock periods."]
    PulseOnCmp = 15,
    #[doc = "14: Toggle on compare repeatedly. Channel function sequence: - Toggle enabled outputs when C2CC.VAL = CNTR.VAL."]
    TglOnCmp = 14,
    #[doc = "13: Set on compare repeatedly. Channel function sequence: - Set enabled outputs when C2CC.VAL = CNTR.VAL."]
    SetOnCmp = 13,
    #[doc = "12: Clear on compare repeatedly. Channel function sequence: - Clear enabled outputs when C2CC.VAL = CNTR.VAL."]
    ClrOnCmp = 12,
    #[doc = "11: Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When C2CC.VAL $lt;= TGT.VAL: Duty cycle = C2CC.VAL / ( TGT.VAL + 1 ). When C2CC.VAL $gt; TGT.VAL: Duty cycle = 1. Enabled outputs are cleared when C2CC.VAL = 0 and CNTR.VAL = 0."]
    SetOn0TglOnCmp = 11,
    #[doc = "10: Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When C2CC.VAL $lt;= TGT.VAL: Duty cycle = 1 - ( C2CC.VAL / TGT.VAL ). When C2CC.VAL $gt; TGT.VAL: Duty cycle = 0. Enabled outputs are set when C2CC.VAL = 0 and CNTR.VAL = 0."]
    ClrOn0TglOnCmp = 10,
    #[doc = "9: Set on capture repeatedly. Channel function sequence: - Set enabled outputs on capture event and copy CNTR.VAL to C2CC.VAL."]
    SetOnCapt = 9,
    #[doc = "8: Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by INPUT relative to the signal edge given by EDGE. Set enabled outputs and RIS.C2CC when C2CC.VAL contains signal period and PC2CC.VAL contains signal pulse width. Notes: - Make sure to configure INPUT and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when C2CC.VAL contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - To observe a timeout event the RIS.TGT interrupt can be used, or another channel can be configured to SET_ON_CMP with compare value equal TGT. Signal property requirements: - Signal Period $gt;= 2 * ( 1 + PRECFG.TICKDIV ) * timer clock period. - Signal Period $lt;= MAX(CNTR) * (1 + PRECFG.TICKDIV ) * timer clock period. - Signal low and high phase $gt;= (1 + PRECFG.TICKDIV ) * timer clock period."]
    PerPulseWidthMeas = 8,
    #[doc = "7: Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. The output is high for two timer clock periods."]
    PulseOnCmpDis = 7,
    #[doc = "6: Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    TglOnCmpDis = 6,
    #[doc = "5: Set on compare, and then disable channel. Channel function sequence: - Set enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    SetOnCmpDis = 5,
    #[doc = "4: Clear on compare, and then disable channel. Channel function sequence: - Clear enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    ClrOnCmpDis = 4,
    #[doc = "3: Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. Enabled outputs are cleared when C2CC.VAL = 0 and CNTR.VAL = 0."]
    SetOn0TglOnCmpDis = 3,
    #[doc = "2: Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. Enabled outputs are set when C2CC.VAL = 0 and CNTR.VAL = 0."]
    ClrOn0TglOnCmpDis = 2,
    #[doc = "1: Set on capture, and then disable channel. Channel function sequence: - Set enabled outputs on capture event and copy CNTR.VAL to C2CC.VAL. - Disable channel. Primary use scenario is to select this function before starting the timer. Follow these steps to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no output enable. - Configure INPUT (optional). - Wait for three timer clock periods as defined in PRECFG before setting CCACT to SET_ON_CAPT_DIS. Output enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
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
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events. In every compare event the timer looks at the current value of CNTR. The corresponding output event will be set 1 timer period after CNTR = C2CC."]
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
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled outputs when C2CC.VAL = CNTR.VAL. The output is high for two timer clock periods."]
    #[inline(always)]
    pub fn is_pulse_on_cmp(&self) -> bool {
        *self == Ccact::PulseOnCmp
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled outputs when C2CC.VAL = CNTR.VAL."]
    #[inline(always)]
    pub fn is_tgl_on_cmp(&self) -> bool {
        *self == Ccact::TglOnCmp
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled outputs when C2CC.VAL = CNTR.VAL."]
    #[inline(always)]
    pub fn is_set_on_cmp(&self) -> bool {
        *self == Ccact::SetOnCmp
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled outputs when C2CC.VAL = CNTR.VAL."]
    #[inline(always)]
    pub fn is_clr_on_cmp(&self) -> bool {
        *self == Ccact::ClrOnCmp
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When C2CC.VAL $lt;= TGT.VAL: Duty cycle = C2CC.VAL / ( TGT.VAL + 1 ). When C2CC.VAL $gt; TGT.VAL: Duty cycle = 1. Enabled outputs are cleared when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp(&self) -> bool {
        *self == Ccact::SetOn0TglOnCmp
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When C2CC.VAL $lt;= TGT.VAL: Duty cycle = 1 - ( C2CC.VAL / TGT.VAL ). When C2CC.VAL $gt; TGT.VAL: Duty cycle = 0. Enabled outputs are set when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp(&self) -> bool {
        *self == Ccact::ClrOn0TglOnCmp
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled outputs on capture event and copy CNTR.VAL to C2CC.VAL."]
    #[inline(always)]
    pub fn is_set_on_capt(&self) -> bool {
        *self == Ccact::SetOnCapt
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by INPUT relative to the signal edge given by EDGE. Set enabled outputs and RIS.C2CC when C2CC.VAL contains signal period and PC2CC.VAL contains signal pulse width. Notes: - Make sure to configure INPUT and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when C2CC.VAL contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - To observe a timeout event the RIS.TGT interrupt can be used, or another channel can be configured to SET_ON_CMP with compare value equal TGT. Signal property requirements: - Signal Period $gt;= 2 * ( 1 + PRECFG.TICKDIV ) * timer clock period. - Signal Period $lt;= MAX(CNTR) * (1 + PRECFG.TICKDIV ) * timer clock period. - Signal low and high phase $gt;= (1 + PRECFG.TICKDIV ) * timer clock period."]
    #[inline(always)]
    pub fn is_per_pulse_width_meas(&self) -> bool {
        *self == Ccact::PerPulseWidthMeas
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. The output is high for two timer clock periods."]
    #[inline(always)]
    pub fn is_pulse_on_cmp_dis(&self) -> bool {
        *self == Ccact::PulseOnCmpDis
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    #[inline(always)]
    pub fn is_tgl_on_cmp_dis(&self) -> bool {
        *self == Ccact::TglOnCmpDis
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    #[inline(always)]
    pub fn is_set_on_cmp_dis(&self) -> bool {
        *self == Ccact::SetOnCmpDis
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    #[inline(always)]
    pub fn is_clr_on_cmp_dis(&self) -> bool {
        *self == Ccact::ClrOnCmpDis
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. Enabled outputs are cleared when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn is_set_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == Ccact::SetOn0TglOnCmpDis
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. Enabled outputs are set when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn is_clr_on_0_tgl_on_cmp_dis(&self) -> bool {
        *self == Ccact::ClrOn0TglOnCmpDis
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled outputs on capture event and copy CNTR.VAL to C2CC.VAL. - Disable channel. Primary use scenario is to select this function before starting the timer. Follow these steps to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no output enable. - Configure INPUT (optional). - Wait for three timer clock periods as defined in PRECFG before setting CCACT to SET_ON_CAPT_DIS. Output enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
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
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events. In every compare event the timer looks at the current value of CNTR. The corresponding output event will be set 1 timer period after CNTR = C2CC."]
pub type CcactW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccact, crate::Safe>;
impl<'a, REG> CcactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulse on compare repeatedly. Channel function sequence: - Pulse enabled outputs when C2CC.VAL = CNTR.VAL. The output is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::PulseOnCmp)
    }
    #[doc = "Toggle on compare repeatedly. Channel function sequence: - Toggle enabled outputs when C2CC.VAL = CNTR.VAL."]
    #[inline(always)]
    pub fn tgl_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::TglOnCmp)
    }
    #[doc = "Set on compare repeatedly. Channel function sequence: - Set enabled outputs when C2CC.VAL = CNTR.VAL."]
    #[inline(always)]
    pub fn set_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCmp)
    }
    #[doc = "Clear on compare repeatedly. Channel function sequence: - Clear enabled outputs when C2CC.VAL = CNTR.VAL."]
    #[inline(always)]
    pub fn clr_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOnCmp)
    }
    #[doc = "Set on zero, toggle on compare repeatedly. Channel function sequence: - Set enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. Set CTL.MODE to UP_PER for edge-aligned PWM generation. Duty cycle is given by: When C2CC.VAL $lt;= TGT.VAL: Duty cycle = C2CC.VAL / ( TGT.VAL + 1 ). When C2CC.VAL $gt; TGT.VAL: Duty cycle = 1. Enabled outputs are cleared when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOn0TglOnCmp)
    }
    #[doc = "Clear on zero, toggle on compare repeatedly. Channel function sequence: - Clear enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. Set CTL.MODE to UPDWN_PER for center-aligned PWM generation. Duty cycle is given by: When C2CC.VAL $lt;= TGT.VAL: Duty cycle = 1 - ( C2CC.VAL / TGT.VAL ). When C2CC.VAL $gt; TGT.VAL: Duty cycle = 0. Enabled outputs are set when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOn0TglOnCmp)
    }
    #[doc = "Set on capture repeatedly. Channel function sequence: - Set enabled outputs on capture event and copy CNTR.VAL to C2CC.VAL."]
    #[inline(always)]
    pub fn set_on_capt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCapt)
    }
    #[doc = "Period and pulse width measurement. Continuously capture period and pulse width of the signal selected by INPUT relative to the signal edge given by EDGE. Set enabled outputs and RIS.C2CC when C2CC.VAL contains signal period and PC2CC.VAL contains signal pulse width. Notes: - Make sure to configure INPUT and CCACT when CTL.MODE equals DIS, then set CTL.MODE to UP_ONCE or UP_PER. - The counter restarts in the selected timer mode when C2CC.VAL contains the signal period. - If more than one channel uses this function, the channels will perform this function one at a time. The channel with lowest number has priority and performs the function first. Next measurement starts when current measurement completes successfully or times out. A timeout occurs when counter equals target. - To observe a timeout event the RIS.TGT interrupt can be used, or another channel can be configured to SET_ON_CMP with compare value equal TGT. Signal property requirements: - Signal Period $gt;= 2 * ( 1 + PRECFG.TICKDIV ) * timer clock period. - Signal Period $lt;= MAX(CNTR) * (1 + PRECFG.TICKDIV ) * timer clock period. - Signal low and high phase $gt;= (1 + PRECFG.TICKDIV ) * timer clock period."]
    #[inline(always)]
    pub fn per_pulse_width_meas(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::PerPulseWidthMeas)
    }
    #[doc = "Pulse on compare, and then disable channel. Channel function sequence: - Pulse enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. The output is high for two timer clock periods."]
    #[inline(always)]
    pub fn pulse_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::PulseOnCmpDis)
    }
    #[doc = "Toggle on compare, and then disable channel. Channel function sequence: - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    #[inline(always)]
    pub fn tgl_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::TglOnCmpDis)
    }
    #[doc = "Set on compare, and then disable channel. Channel function sequence: - Set enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    #[inline(always)]
    pub fn set_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOnCmpDis)
    }
    #[doc = "Clear on compare, and then disable channel. Channel function sequence: - Clear enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel."]
    #[inline(always)]
    pub fn clr_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOnCmpDis)
    }
    #[doc = "Set on zero, toggle on compare, and then disable channel. Channel function sequence: - Set enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. Enabled outputs are cleared when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn set_on_0_tgl_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::SetOn0TglOnCmpDis)
    }
    #[doc = "Clear on zero, toggle on compare, and then disable channel. Channel function sequence: - Clear enabled outputs when CNTR.VAL = 0. - Toggle enabled outputs when C2CC.VAL = CNTR.VAL. - Disable channel. Enabled outputs are set when C2CC.VAL = 0 and CNTR.VAL = 0."]
    #[inline(always)]
    pub fn clr_on_0_tgl_on_cmp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact::ClrOn0TglOnCmpDis)
    }
    #[doc = "Set on capture, and then disable channel. Channel function sequence: - Set enabled outputs on capture event and copy CNTR.VAL to C2CC.VAL. - Disable channel. Primary use scenario is to select this function before starting the timer. Follow these steps to select this function while CTL.MODE is different from DIS: - Set CCACT to SET_ON_CAPT with no output enable. - Configure INPUT (optional). - Wait for three timer clock periods as defined in PRECFG before setting CCACT to SET_ON_CAPT_DIS. Output enable is optional. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
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
#[doc = "5:4\\]
Determines the edge that triggers the channel input event. This happens post filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edge {
    #[doc = "3: Input event is triggered at both edges."]
    Both = 3,
    #[doc = "2: Input event is triggered at falling edge."]
    Fall = 2,
    #[doc = "1: Input event is triggered at rising edge."]
    Rise = 1,
    #[doc = "0: Input is turned off."]
    None = 0,
}
impl From<Edge> for u8 {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edge {
    type Ux = u8;
}
impl crate::IsEnum for Edge {}
#[doc = "Field `EDGE` reader - 5:4\\]
Determines the edge that triggers the channel input event. This happens post filter."]
pub type EdgeR = crate::FieldReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            3 => Edge::Both,
            2 => Edge::Fall,
            1 => Edge::Rise,
            0 => Edge::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Input event is triggered at both edges."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Edge::Both
    }
    #[doc = "Input event is triggered at falling edge."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Edge::Fall
    }
    #[doc = "Input event is triggered at rising edge."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Edge::Rise
    }
    #[doc = "Input is turned off."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Edge::None
    }
}
#[doc = "Field `EDGE` writer - 5:4\\]
Determines the edge that triggers the channel input event. This happens post filter."]
pub type EdgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edge, crate::Safe>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input event is triggered at both edges."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Both)
    }
    #[doc = "Input event is triggered at falling edge."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Fall)
    }
    #[doc = "Input event is triggered at rising edge."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Rise)
    }
    #[doc = "Input is turned off."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::None)
    }
}
#[doc = "6:6\\]
Select channel input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    #[doc = "1: IO controller"]
    Io = 1,
    #[doc = "0: Event fabric"]
    Ev = 0,
}
impl From<Input> for bool {
    #[inline(always)]
    fn from(variant: Input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT` reader - 6:6\\]
Select channel input."]
pub type InputR = crate::BitReader<Input>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input {
        match self.bits {
            true => Input::Io,
            false => Input::Ev,
        }
    }
    #[doc = "IO controller"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Input::Io
    }
    #[doc = "Event fabric"]
    #[inline(always)]
    pub fn is_ev(&self) -> bool {
        *self == Input::Ev
    }
}
#[doc = "Field `INPUT` writer - 6:6\\]
Select channel input."]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, Input>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO controller"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Input::Io)
    }
    #[doc = "Event fabric"]
    #[inline(always)]
    pub fn ev(self) -> &'a mut crate::W<REG> {
        self.variant(Input::Ev)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "8:8\\]
Output 0 enable. When 0 $lt; CCACT $lt; 8, OUT0 becomes zero after a capture or compare event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out0 {
    #[doc = "1: Channel 2 controls output 0."]
    En = 1,
    #[doc = "0: Channel 2 does not control output 0."]
    Dis = 0,
}
impl From<Out0> for bool {
    #[inline(always)]
    fn from(variant: Out0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT0` reader - 8:8\\]
Output 0 enable. When 0 $lt; CCACT $lt; 8, OUT0 becomes zero after a capture or compare event."]
pub type Out0R = crate::BitReader<Out0>;
impl Out0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out0 {
        match self.bits {
            true => Out0::En,
            false => Out0::Dis,
        }
    }
    #[doc = "Channel 2 controls output 0."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Out0::En
    }
    #[doc = "Channel 2 does not control output 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Out0::Dis
    }
}
#[doc = "Field `OUT0` writer - 8:8\\]
Output 0 enable. When 0 $lt; CCACT $lt; 8, OUT0 becomes zero after a capture or compare event."]
pub type Out0W<'a, REG> = crate::BitWriter<'a, REG, Out0>;
impl<'a, REG> Out0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 controls output 0."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::En)
    }
    #[doc = "Channel 2 does not control output 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Dis)
    }
}
#[doc = "9:9\\]
Output 1 enable. When 0 $lt; CCACT $lt; 8, OUT1 becomes zero after a capture or compare event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out1 {
    #[doc = "1: Channel 2 controls output 1."]
    En = 1,
    #[doc = "0: Channel 2 does not control output 1."]
    Dis = 0,
}
impl From<Out1> for bool {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT1` reader - 9:9\\]
Output 1 enable. When 0 $lt; CCACT $lt; 8, OUT1 becomes zero after a capture or compare event."]
pub type Out1R = crate::BitReader<Out1>;
impl Out1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out1 {
        match self.bits {
            true => Out1::En,
            false => Out1::Dis,
        }
    }
    #[doc = "Channel 2 controls output 1."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Out1::En
    }
    #[doc = "Channel 2 does not control output 1."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Out1::Dis
    }
}
#[doc = "Field `OUT1` writer - 9:9\\]
Output 1 enable. When 0 $lt; CCACT $lt; 8, OUT1 becomes zero after a capture or compare event."]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG, Out1>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 controls output 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::En)
    }
    #[doc = "Channel 2 does not control output 1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Dis)
    }
}
#[doc = "10:10\\]
Output 2 enable. When 0 $lt; CCACT $lt; 8, OUT2 becomes zero after a capture or compare event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out2 {
    #[doc = "1: Channel 2 controls output 2."]
    En = 1,
    #[doc = "0: Channel 2 does not control output 2."]
    Dis = 0,
}
impl From<Out2> for bool {
    #[inline(always)]
    fn from(variant: Out2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT2` reader - 10:10\\]
Output 2 enable. When 0 $lt; CCACT $lt; 8, OUT2 becomes zero after a capture or compare event."]
pub type Out2R = crate::BitReader<Out2>;
impl Out2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out2 {
        match self.bits {
            true => Out2::En,
            false => Out2::Dis,
        }
    }
    #[doc = "Channel 2 controls output 2."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Out2::En
    }
    #[doc = "Channel 2 does not control output 2."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Out2::Dis
    }
}
#[doc = "Field `OUT2` writer - 10:10\\]
Output 2 enable. When 0 $lt; CCACT $lt; 8, OUT2 becomes zero after a capture or compare event."]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG, Out2>;
impl<'a, REG> Out2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 controls output 2."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::En)
    }
    #[doc = "Channel 2 does not control output 2."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Dis)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events. In every compare event the timer looks at the current value of CNTR. The corresponding output event will be set 1 timer period after CNTR = C2CC."]
    #[inline(always)]
    pub fn ccact(&self) -> CcactR {
        CcactR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Determines the edge that triggers the channel input event. This happens post filter."]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Select channel input."]
    #[inline(always)]
    pub fn input(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Output 0 enable. When 0 $lt; CCACT $lt; 8, OUT0 becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Output 1 enable. When 0 $lt; CCACT $lt; 8, OUT1 becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Output 2 enable. When 0 $lt; CCACT $lt; 8, OUT2 becomes zero after a capture or compare event."]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Capture-Compare action. Capture-Compare action defines 15 different channel functions that utilize capture, compare, and zero events. In every compare event the timer looks at the current value of CNTR. The corresponding output event will be set 1 timer period after CNTR = C2CC."]
    #[inline(always)]
    #[must_use]
    pub fn ccact(&mut self) -> CcactW<C2cfgSpec> {
        CcactW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Determines the edge that triggers the channel input event. This happens post filter."]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<C2cfgSpec> {
        EdgeW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Select channel input."]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> InputW<C2cfgSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Output 0 enable. When 0 $lt; CCACT $lt; 8, OUT0 becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<C2cfgSpec> {
        Out0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Output 1 enable. When 0 $lt; CCACT $lt; 8, OUT1 becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<C2cfgSpec> {
        Out1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Output 2 enable. When 0 $lt; CCACT $lt; 8, OUT2 becomes zero after a capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<C2cfgSpec> {
        Out2W::new(self, 10)
    }
}
#[doc = "Channel 2 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2cfgSpec;
impl crate::RegisterSpec for C2cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cfg::R`](R) reader structure"]
impl crate::Readable for C2cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`c2cfg::W`](W) writer structure"]
impl crate::Writable for C2cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CFG to value 0"]
impl crate::Resettable for C2cfgSpec {
    const RESET_VALUE: u32 = 0;
}
