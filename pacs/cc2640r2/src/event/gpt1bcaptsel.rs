#[doc = "Register `GPT1BCAPTSEL` reader"]
pub type R = crate::R<Gpt1bcaptselSpec>;
#[doc = "Register `GPT1BCAPTSEL` writer"]
pub type W = crate::W<Gpt1bcaptselSpec>;
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 88"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "121: Always asserted"]
    AlwaysActive = 121,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AonRtcUpd = 119,
    #[doc = "115: AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    AuxAdcIrq = 115,
    #[doc = "114: Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0"]
    AuxObsmux0 = 114,
    #[doc = "113: AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL"]
    AuxAdcFifoAlmostFull = 113,
    #[doc = "112: AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE"]
    AuxAdcDone = 112,
    #[doc = "111: Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AuxSmphAutotakeDone = 111,
    #[doc = "110: AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV"]
    AuxTimer1Ev = 110,
    #[doc = "109: AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV"]
    AuxTimer0Ev = 109,
    #[doc = "108: AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AuxTdcDone = 108,
    #[doc = "107: AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AuxCompb = 107,
    #[doc = "106: AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AuxCompa = 106,
    #[doc = "105: AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    AuxAonWuEv = 105,
    #[doc = "88: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PortEvent3 = 88,
    #[doc = "87: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PortEvent2 = 87,
    #[doc = "68: GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    Gpt3bCmp = 68,
    #[doc = "67: GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    Gpt3aCmp = 67,
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    Gpt2bCmp = 66,
    #[doc = "65: GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    Gpt2aCmp = 65,
    #[doc = "64: GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    Gpt1bCmp = 64,
    #[doc = "63: GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    Gpt1aCmp = 63,
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    Gpt0bCmp = 62,
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    Gpt0aCmp = 61,
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    Uart0Comb = 36,
    #[doc = "35: SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    Ssi1Comb = 35,
    #[doc = "34: SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    Ssi0Comb = 34,
    #[doc = "30: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RfcCpe1 = 30,
    #[doc = "27: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RfcCpe0 = 27,
    #[doc = "26: Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RfcHwComb = 26,
    #[doc = "25: RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RfcCmdAck = 25,
    #[doc = "21: FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    Flash = 21,
    #[doc = "11: AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AuxComb = 11,
    #[doc = "9: Interrupt event from I2C"]
    I2cIrq = 9,
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AonRtcComb = 7,
    #[doc = "4: Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    AonGpioEdge = 4,
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            121 => Some(Ev::AlwaysActive),
            119 => Some(Ev::AonRtcUpd),
            115 => Some(Ev::AuxAdcIrq),
            114 => Some(Ev::AuxObsmux0),
            113 => Some(Ev::AuxAdcFifoAlmostFull),
            112 => Some(Ev::AuxAdcDone),
            111 => Some(Ev::AuxSmphAutotakeDone),
            110 => Some(Ev::AuxTimer1Ev),
            109 => Some(Ev::AuxTimer0Ev),
            108 => Some(Ev::AuxTdcDone),
            107 => Some(Ev::AuxCompb),
            106 => Some(Ev::AuxCompa),
            105 => Some(Ev::AuxAonWuEv),
            88 => Some(Ev::PortEvent3),
            87 => Some(Ev::PortEvent2),
            68 => Some(Ev::Gpt3bCmp),
            67 => Some(Ev::Gpt3aCmp),
            66 => Some(Ev::Gpt2bCmp),
            65 => Some(Ev::Gpt2aCmp),
            64 => Some(Ev::Gpt1bCmp),
            63 => Some(Ev::Gpt1aCmp),
            62 => Some(Ev::Gpt0bCmp),
            61 => Some(Ev::Gpt0aCmp),
            36 => Some(Ev::Uart0Comb),
            35 => Some(Ev::Ssi1Comb),
            34 => Some(Ev::Ssi0Comb),
            30 => Some(Ev::RfcCpe1),
            27 => Some(Ev::RfcCpe0),
            26 => Some(Ev::RfcHwComb),
            25 => Some(Ev::RfcCmdAck),
            21 => Some(Ev::Flash),
            11 => Some(Ev::AuxComb),
            9 => Some(Ev::I2cIrq),
            7 => Some(Ev::AonRtcComb),
            4 => Some(Ev::AonGpioEdge),
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == Ev::AlwaysActive
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == Ev::AonRtcUpd
    }
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == Ev::AuxAdcIrq
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0"]
    #[inline(always)]
    pub fn is_aux_obsmux0(&self) -> bool {
        *self == Ev::AuxObsmux0
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == Ev::AuxAdcFifoAlmostFull
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Ev::AuxAdcDone
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == Ev::AuxSmphAutotakeDone
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Ev::AuxTimer1Ev
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Ev::AuxTimer0Ev
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == Ev::AuxTdcDone
    }
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == Ev::AuxCompb
    }
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == Ev::AuxCompa
    }
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    #[inline(always)]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == Ev::AuxAonWuEv
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event3(&self) -> bool {
        *self == Ev::PortEvent3
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event2(&self) -> bool {
        *self == Ev::PortEvent2
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == Ev::Gpt3bCmp
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == Ev::Gpt3aCmp
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == Ev::Gpt2bCmp
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == Ev::Gpt2aCmp
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt1b_cmp(&self) -> bool {
        *self == Ev::Gpt1bCmp
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == Ev::Gpt1aCmp
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == Ev::Gpt0bCmp
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == Ev::Gpt0aCmp
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == Ev::Uart0Comb
    }
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn is_ssi1_comb(&self) -> bool {
        *self == Ev::Ssi1Comb
    }
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    #[inline(always)]
    pub fn is_ssi0_comb(&self) -> bool {
        *self == Ev::Ssi0Comb
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    #[inline(always)]
    pub fn is_rfc_cpe_1(&self) -> bool {
        *self == Ev::RfcCpe1
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    #[inline(always)]
    pub fn is_rfc_cpe_0(&self) -> bool {
        *self == Ev::RfcCpe0
    }
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline(always)]
    pub fn is_rfc_hw_comb(&self) -> bool {
        *self == Ev::RfcHwComb
    }
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    #[inline(always)]
    pub fn is_rfc_cmd_ack(&self) -> bool {
        *self == Ev::RfcCmdAck
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == Ev::Flash
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn is_aux_comb(&self) -> bool {
        *self == Ev::AuxComb
    }
    #[doc = "Interrupt event from I2C"]
    #[inline(always)]
    pub fn is_i2c_irq(&self) -> bool {
        *self == Ev::I2cIrq
    }
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == Ev::AonRtcComb
    }
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    #[inline(always)]
    pub fn is_aon_gpio_edge(&self) -> bool {
        *self == Ev::AonGpioEdge
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ev::None
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 7, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AlwaysActive)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonRtcUpd)
    }
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAdcIrq)
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0"]
    #[inline(always)]
    pub fn aux_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxObsmux0)
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAdcFifoAlmostFull)
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAdcDone)
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxSmphAutotakeDone)
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer1Ev)
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer0Ev)
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTdcDone)
    }
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxCompb)
    }
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxCompa)
    }
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    #[inline(always)]
    pub fn aux_aon_wu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAonWuEv)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    #[inline(always)]
    pub fn port_event3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent3)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    #[inline(always)]
    pub fn port_event2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent2)
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt3b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3bCmp)
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt3a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3aCmp)
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt2b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2bCmp)
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt2a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2aCmp)
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt1b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1bCmp)
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt1a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1aCmp)
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt0b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0bCmp)
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0aCmp)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0Comb)
    }
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn ssi1_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi1Comb)
    }
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    #[inline(always)]
    pub fn ssi0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi0Comb)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    #[inline(always)]
    pub fn rfc_cpe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::RfcCpe1)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    #[inline(always)]
    pub fn rfc_cpe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::RfcCpe0)
    }
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline(always)]
    pub fn rfc_hw_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::RfcHwComb)
    }
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    #[inline(always)]
    pub fn rfc_cmd_ack(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::RfcCmdAck)
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Flash)
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxComb)
    }
    #[doc = "Interrupt event from I2C"]
    #[inline(always)]
    pub fn i2c_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::I2cIrq)
    }
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline(always)]
    pub fn aon_rtc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonRtcComb)
    }
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    #[inline(always)]
    pub fn aon_gpio_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonGpioEdge)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::None)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Gpt1bcaptselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for GPT1 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt1bcaptsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt1bcaptsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpt1bcaptselSpec;
impl crate::RegisterSpec for Gpt1bcaptselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpt1bcaptsel::R`](R) reader structure"]
impl crate::Readable for Gpt1bcaptselSpec {}
#[doc = "`write(|w| ..)` method takes [`gpt1bcaptsel::W`](W) writer structure"]
impl crate::Writable for Gpt1bcaptselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPT1BCAPTSEL to value 0x58"]
impl crate::Resettable for Gpt1bcaptselSpec {
    const RESET_VALUE: u32 = 0x58;
}
