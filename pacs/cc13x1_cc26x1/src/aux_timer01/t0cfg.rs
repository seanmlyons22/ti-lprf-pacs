#[doc = "Register `T0CFG` reader"]
pub type R = crate::R<T0cfgSpec>;
#[doc = "Register `T0CFG` writer"]
pub type W = crate::W<T0cfgSpec>;
#[doc = "0:0\\]
Timer 0 reload mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reload {
    #[doc = "1: Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    Cont = 1,
    #[doc = "0: Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    Man = 0,
}
impl From<Reload> for bool {
    #[inline(always)]
    fn from(variant: Reload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - 0:0\\]
Timer 0 reload mode."]
pub type ReloadR = crate::BitReader<Reload>;
impl ReloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reload {
        match self.bits {
            true => Reload::Cont,
            false => Reload::Man,
        }
    }
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Reload::Cont
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == Reload::Man
    }
}
#[doc = "Field `RELOAD` writer - 0:0\\]
Timer 0 reload mode."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG, Reload>;
impl<'a, REG> ReloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Cont)
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline(always)]
    pub fn man(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Man)
    }
}
#[doc = "1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "1: Use event set by TICK_SRC as source for prescaler."]
    Tick = 1,
    #[doc = "0: Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    Clk = 0,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            true => Mode::Tick,
            false => Mode::Clk,
        }
    }
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline(always)]
    pub fn is_tick(&self) -> bool {
        *self == Mode::Tick
    }
    #[doc = "Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == Mode::Clk
    }
}
#[doc = "Field `MODE` writer - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline(always)]
    pub fn tick(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Tick)
    }
    #[doc = "Use clock as source for prescaler. Note that AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the clock frequency."]
    #[inline(always)]
    pub fn clk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Clk)
    }
}
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PRE` reader - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
pub type PreR = crate::FieldReader;
#[doc = "Field `PRE` writer - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "13:8\\]
Select Timer 0 tick source from the synchronous event bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TickSrc {
    #[doc = "62: AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    AuxDacHoldActive = 62,
    #[doc = "61: AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    AuxSmphAutotakeDone = 61,
    #[doc = "60: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    AuxAdcFifoNotEmpty = 60,
    #[doc = "59: AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    AuxAdcFifoAlmostFull = 59,
    #[doc = "58: AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    AuxAdcIrq = 58,
    #[doc = "57: AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    AuxAdcDone = 57,
    #[doc = "56: AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    AuxIsrcResetN = 56,
    #[doc = "55: AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    AuxTdcDone = 55,
    #[doc = "54: No event."]
    NoEvent = 54,
    #[doc = "53: AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    AuxTimer1Ev = 53,
    #[doc = "47: AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    AuxCompb = 47,
    #[doc = "46: AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    AuxCompa = 46,
    #[doc = "45: AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    McuObsmux1 = 45,
    #[doc = "44: AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    McuObsmux0 = 44,
    #[doc = "43: AUX_EVCTL:EVSTAT2.MCU_EV"]
    McuEv = 43,
    #[doc = "42: AUX_EVCTL:EVSTAT2.ACLK_REF"]
    AclkRef = 42,
    #[doc = "41: AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    VddrRecharge = 41,
    #[doc = "40: AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    McuActive = 40,
    #[doc = "39: AUX_EVCTL:EVSTAT2.PWR_DWN"]
    PwrDwn = 39,
    #[doc = "38: AUX_EVCTL:EVSTAT2.SCLK_LF"]
    SclkLf = 38,
    #[doc = "35: AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    AonRtc4khz = 35,
    #[doc = "34: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AonRtcCh2Dly = 34,
    #[doc = "33: AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    AonRtcCh2 = 33,
    #[doc = "32: AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    ManualEv = 32,
    #[doc = "31: AUX_EVCTL:EVSTAT1.AUXIO31"]
    Auxio31 = 31,
    #[doc = "30: AUX_EVCTL:EVSTAT1.AUXIO30"]
    Auxio30 = 30,
    #[doc = "29: AUX_EVCTL:EVSTAT1.AUXIO29"]
    Auxio29 = 29,
    #[doc = "28: AUX_EVCTL:EVSTAT1.AUXIO28"]
    Auxio28 = 28,
    #[doc = "27: AUX_EVCTL:EVSTAT1.AUXIO27"]
    Auxio27 = 27,
    #[doc = "26: AUX_EVCTL:EVSTAT1.AUXIO26"]
    Auxio26 = 26,
    #[doc = "25: AUX_EVCTL:EVSTAT1.AUXIO25"]
    Auxio25 = 25,
    #[doc = "24: AUX_EVCTL:EVSTAT1.AUXIO24"]
    Auxio24 = 24,
    #[doc = "23: AUX_EVCTL:EVSTAT1.AUXIO23"]
    Auxio23 = 23,
    #[doc = "22: AUX_EVCTL:EVSTAT1.AUXIO22"]
    Auxio22 = 22,
    #[doc = "21: AUX_EVCTL:EVSTAT1.AUXIO21"]
    Auxio21 = 21,
    #[doc = "20: AUX_EVCTL:EVSTAT1.AUXIO20"]
    Auxio20 = 20,
    #[doc = "19: AUX_EVCTL:EVSTAT1.AUXIO19"]
    Auxio19 = 19,
    #[doc = "18: AUX_EVCTL:EVSTAT1.AUXIO18"]
    Auxio18 = 18,
    #[doc = "17: AUX_EVCTL:EVSTAT1.AUXIO17"]
    Auxio17 = 17,
    #[doc = "16: AUX_EVCTL:EVSTAT1.AUXIO16"]
    Auxio16 = 16,
    #[doc = "15: AUX_EVCTL:EVSTAT0.AUXIO15"]
    Auxio15 = 15,
    #[doc = "14: AUX_EVCTL:EVSTAT0.AUXIO14"]
    Auxio14 = 14,
    #[doc = "13: AUX_EVCTL:EVSTAT0.AUXIO13"]
    Auxio13 = 13,
    #[doc = "12: AUX_EVCTL:EVSTAT0.AUXIO12"]
    Auxio12 = 12,
    #[doc = "11: AUX_EVCTL:EVSTAT0.AUXIO11"]
    Auxio11 = 11,
    #[doc = "10: AUX_EVCTL:EVSTAT0.AUXIO10"]
    Auxio10 = 10,
    #[doc = "9: AUX_EVCTL:EVSTAT0.AUXIO9"]
    Auxio9 = 9,
    #[doc = "8: AUX_EVCTL:EVSTAT0.AUXIO8"]
    Auxio8 = 8,
    #[doc = "7: AUX_EVCTL:EVSTAT0.AUXIO7"]
    Auxio7 = 7,
    #[doc = "6: AUX_EVCTL:EVSTAT0.AUXIO6"]
    Auxio6 = 6,
    #[doc = "5: AUX_EVCTL:EVSTAT0.AUXIO5"]
    Auxio5 = 5,
    #[doc = "4: AUX_EVCTL:EVSTAT0.AUXIO4"]
    Auxio4 = 4,
    #[doc = "3: AUX_EVCTL:EVSTAT0.AUXIO3"]
    Auxio3 = 3,
    #[doc = "2: AUX_EVCTL:EVSTAT0.AUXIO2"]
    Auxio2 = 2,
    #[doc = "1: AUX_EVCTL:EVSTAT0.AUXIO1"]
    Auxio1 = 1,
    #[doc = "0: AUX_EVCTL:EVSTAT0.AUXIO0"]
    Auxio0 = 0,
}
impl From<TickSrc> for u8 {
    #[inline(always)]
    fn from(variant: TickSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TickSrc {
    type Ux = u8;
}
impl crate::IsEnum for TickSrc {}
#[doc = "Field `TICK_SRC` reader - 13:8\\]
Select Timer 0 tick source from the synchronous event bus."]
pub type TickSrcR = crate::FieldReader<TickSrc>;
impl TickSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TickSrc> {
        match self.bits {
            62 => Some(TickSrc::AuxDacHoldActive),
            61 => Some(TickSrc::AuxSmphAutotakeDone),
            60 => Some(TickSrc::AuxAdcFifoNotEmpty),
            59 => Some(TickSrc::AuxAdcFifoAlmostFull),
            58 => Some(TickSrc::AuxAdcIrq),
            57 => Some(TickSrc::AuxAdcDone),
            56 => Some(TickSrc::AuxIsrcResetN),
            55 => Some(TickSrc::AuxTdcDone),
            54 => Some(TickSrc::NoEvent),
            53 => Some(TickSrc::AuxTimer1Ev),
            47 => Some(TickSrc::AuxCompb),
            46 => Some(TickSrc::AuxCompa),
            45 => Some(TickSrc::McuObsmux1),
            44 => Some(TickSrc::McuObsmux0),
            43 => Some(TickSrc::McuEv),
            42 => Some(TickSrc::AclkRef),
            41 => Some(TickSrc::VddrRecharge),
            40 => Some(TickSrc::McuActive),
            39 => Some(TickSrc::PwrDwn),
            38 => Some(TickSrc::SclkLf),
            35 => Some(TickSrc::AonRtc4khz),
            34 => Some(TickSrc::AonRtcCh2Dly),
            33 => Some(TickSrc::AonRtcCh2),
            32 => Some(TickSrc::ManualEv),
            31 => Some(TickSrc::Auxio31),
            30 => Some(TickSrc::Auxio30),
            29 => Some(TickSrc::Auxio29),
            28 => Some(TickSrc::Auxio28),
            27 => Some(TickSrc::Auxio27),
            26 => Some(TickSrc::Auxio26),
            25 => Some(TickSrc::Auxio25),
            24 => Some(TickSrc::Auxio24),
            23 => Some(TickSrc::Auxio23),
            22 => Some(TickSrc::Auxio22),
            21 => Some(TickSrc::Auxio21),
            20 => Some(TickSrc::Auxio20),
            19 => Some(TickSrc::Auxio19),
            18 => Some(TickSrc::Auxio18),
            17 => Some(TickSrc::Auxio17),
            16 => Some(TickSrc::Auxio16),
            15 => Some(TickSrc::Auxio15),
            14 => Some(TickSrc::Auxio14),
            13 => Some(TickSrc::Auxio13),
            12 => Some(TickSrc::Auxio12),
            11 => Some(TickSrc::Auxio11),
            10 => Some(TickSrc::Auxio10),
            9 => Some(TickSrc::Auxio9),
            8 => Some(TickSrc::Auxio8),
            7 => Some(TickSrc::Auxio7),
            6 => Some(TickSrc::Auxio6),
            5 => Some(TickSrc::Auxio5),
            4 => Some(TickSrc::Auxio4),
            3 => Some(TickSrc::Auxio3),
            2 => Some(TickSrc::Auxio2),
            1 => Some(TickSrc::Auxio1),
            0 => Some(TickSrc::Auxio0),
            _ => None,
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn is_aux_dac_hold_active(&self) -> bool {
        *self == TickSrc::AuxDacHoldActive
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == TickSrc::AuxSmphAutotakeDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == TickSrc::AuxAdcFifoNotEmpty
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == TickSrc::AuxAdcFifoAlmostFull
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == TickSrc::AuxAdcIrq
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == TickSrc::AuxAdcDone
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn is_aux_isrc_reset_n(&self) -> bool {
        *self == TickSrc::AuxIsrcResetN
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == TickSrc::AuxTdcDone
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TickSrc::NoEvent
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == TickSrc::AuxTimer1Ev
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == TickSrc::AuxCompb
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == TickSrc::AuxCompa
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn is_mcu_obsmux1(&self) -> bool {
        *self == TickSrc::McuObsmux1
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_mcu_obsmux0(&self) -> bool {
        *self == TickSrc::McuObsmux0
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn is_mcu_ev(&self) -> bool {
        *self == TickSrc::McuEv
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn is_aclk_ref(&self) -> bool {
        *self == TickSrc::AclkRef
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn is_vddr_recharge(&self) -> bool {
        *self == TickSrc::VddrRecharge
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn is_mcu_active(&self) -> bool {
        *self == TickSrc::McuActive
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn is_pwr_dwn(&self) -> bool {
        *self == TickSrc::PwrDwn
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == TickSrc::SclkLf
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn is_aon_rtc_4khz(&self) -> bool {
        *self == TickSrc::AonRtc4khz
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == TickSrc::AonRtcCh2Dly
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == TickSrc::AonRtcCh2
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn is_manual_ev(&self) -> bool {
        *self == TickSrc::ManualEv
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn is_auxio31(&self) -> bool {
        *self == TickSrc::Auxio31
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn is_auxio30(&self) -> bool {
        *self == TickSrc::Auxio30
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn is_auxio29(&self) -> bool {
        *self == TickSrc::Auxio29
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn is_auxio28(&self) -> bool {
        *self == TickSrc::Auxio28
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn is_auxio27(&self) -> bool {
        *self == TickSrc::Auxio27
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == TickSrc::Auxio26
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == TickSrc::Auxio25
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == TickSrc::Auxio24
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == TickSrc::Auxio23
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == TickSrc::Auxio22
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == TickSrc::Auxio21
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == TickSrc::Auxio20
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == TickSrc::Auxio19
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn is_auxio18(&self) -> bool {
        *self == TickSrc::Auxio18
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn is_auxio17(&self) -> bool {
        *self == TickSrc::Auxio17
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn is_auxio16(&self) -> bool {
        *self == TickSrc::Auxio16
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn is_auxio15(&self) -> bool {
        *self == TickSrc::Auxio15
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn is_auxio14(&self) -> bool {
        *self == TickSrc::Auxio14
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn is_auxio13(&self) -> bool {
        *self == TickSrc::Auxio13
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn is_auxio12(&self) -> bool {
        *self == TickSrc::Auxio12
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn is_auxio11(&self) -> bool {
        *self == TickSrc::Auxio11
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn is_auxio10(&self) -> bool {
        *self == TickSrc::Auxio10
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn is_auxio9(&self) -> bool {
        *self == TickSrc::Auxio9
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn is_auxio8(&self) -> bool {
        *self == TickSrc::Auxio8
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        *self == TickSrc::Auxio7
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        *self == TickSrc::Auxio6
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        *self == TickSrc::Auxio5
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        *self == TickSrc::Auxio4
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        *self == TickSrc::Auxio3
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        *self == TickSrc::Auxio2
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        *self == TickSrc::Auxio1
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        *self == TickSrc::Auxio0
    }
}
#[doc = "Field `TICK_SRC` writer - 13:8\\]
Select Timer 0 tick source from the synchronous event bus."]
pub type TickSrcW<'a, REG> = crate::FieldWriter<'a, REG, 6, TickSrc>;
impl<'a, REG> TickSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxDacHoldActive)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxSmphAutotakeDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxAdcFifoNotEmpty)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxAdcFifoAlmostFull)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxAdcIrq)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxAdcDone)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxIsrcResetN)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxTdcDone)
    }
    #[doc = "No event."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::NoEvent)
    }
    #[doc = "AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxTimer1Ev)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxCompb)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AuxCompa)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::McuObsmux1)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::McuObsmux0)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::McuEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AclkRef)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::VddrRecharge)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::McuActive)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::PwrDwn)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::SclkLf)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AonRtc4khz)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AonRtcCh2Dly)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::AonRtcCh2)
    }
    #[doc = "AUX_EVCTL:EVSTAT2.MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::ManualEv)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO31"]
    #[inline(always)]
    pub fn auxio31(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio31)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO30"]
    #[inline(always)]
    pub fn auxio30(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio30)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO29"]
    #[inline(always)]
    pub fn auxio29(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio29)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO28"]
    #[inline(always)]
    pub fn auxio28(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio28)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO27"]
    #[inline(always)]
    pub fn auxio27(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio27)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO26"]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio26)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO25"]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio25)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO24"]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio24)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO23"]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio23)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO22"]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio22)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO21"]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio21)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO20"]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio20)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO19"]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio19)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO18"]
    #[inline(always)]
    pub fn auxio18(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio18)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO17"]
    #[inline(always)]
    pub fn auxio17(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio17)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO16"]
    #[inline(always)]
    pub fn auxio16(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio16)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO15"]
    #[inline(always)]
    pub fn auxio15(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio15)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO14"]
    #[inline(always)]
    pub fn auxio14(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio14)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO13"]
    #[inline(always)]
    pub fn auxio13(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio13)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO12"]
    #[inline(always)]
    pub fn auxio12(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio12)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO11"]
    #[inline(always)]
    pub fn auxio11(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio11)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO10"]
    #[inline(always)]
    pub fn auxio10(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio10)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO9"]
    #[inline(always)]
    pub fn auxio9(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio9)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO8"]
    #[inline(always)]
    pub fn auxio8(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio8)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO7"]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio7)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO6"]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio6)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO5"]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio5)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO4"]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio4)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO3"]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline(always)]
    pub fn auxio0(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrc::Auxio0)
    }
}
#[doc = "14:14\\]
Tick source polarity for Timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickSrcPol {
    #[doc = "1: Count on falling edges of TICK_SRC."]
    Fall = 1,
    #[doc = "0: Count on rising edges of TICK_SRC."]
    Rise = 0,
}
impl From<TickSrcPol> for bool {
    #[inline(always)]
    fn from(variant: TickSrcPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK_SRC_POL` reader - 14:14\\]
Tick source polarity for Timer 0."]
pub type TickSrcPolR = crate::BitReader<TickSrcPol>;
impl TickSrcPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TickSrcPol {
        match self.bits {
            true => TickSrcPol::Fall,
            false => TickSrcPol::Rise,
        }
    }
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TickSrcPol::Fall
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == TickSrcPol::Rise
    }
}
#[doc = "Field `TICK_SRC_POL` writer - 14:14\\]
Tick source polarity for Timer 0."]
pub type TickSrcPolW<'a, REG> = crate::BitWriter<'a, REG, TickSrcPol>;
impl<'a, REG> TickSrcPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrcPol::Fall)
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(TickSrcPol::Rise)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Timer 0 reload mode."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    pub fn tick_src(&self) -> TickSrcR {
        TickSrcR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Tick source polarity for Timer 0."]
    #[inline(always)]
    pub fn tick_src_pol(&self) -> TickSrcPolR {
        TickSrcPolR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Timer 0 reload mode."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> ReloadW<T0cfgSpec> {
        ReloadW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<T0cfgSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<T0cfgSpec> {
        PreW::new(self, 4)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    #[must_use]
    pub fn tick_src(&mut self) -> TickSrcW<T0cfgSpec> {
        TickSrcW::new(self, 8)
    }
    #[doc = "Bit 14 - 14:14\\]
Tick source polarity for Timer 0."]
    #[inline(always)]
    #[must_use]
    pub fn tick_src_pol(&mut self) -> TickSrcPolW<T0cfgSpec> {
        TickSrcPolW::new(self, 14)
    }
}
#[doc = "Timer 0 Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0cfgSpec;
impl crate::RegisterSpec for T0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0cfg::R`](R) reader structure"]
impl crate::Readable for T0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`t0cfg::W`](W) writer structure"]
impl crate::Writable for T0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0CFG to value 0"]
impl crate::Resettable for T0cfgSpec {
    const RESET_VALUE: u32 = 0;
}
