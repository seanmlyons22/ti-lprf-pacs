#[doc = "Register `GPT3BCAPTSEL` reader"]
pub type R = crate::R<Gpt3bcaptselSpec>;
#[doc = "Register `GPT3BCAPTSEL` writer"]
pub type W = crate::W<Gpt3bcaptselSpec>;
#[doc = "7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 92"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "126: Interrupt event from I2C1"]
    I2c1Irq = 126,
    #[doc = "125: UART3 combined interrupt, interrupt flags are found here UART3:MIS"]
    Uart3Comb = 125,
    #[doc = "124: UART2 combined interrupt, interrupt flags are found here UART2:MIS"]
    Uart2Comb = 124,
    #[doc = "123: SPI3 combined interrupt, interrupt flags are found here SPI3:MIS"]
    Spi3Comb = 123,
    #[doc = "122: SPI2 combined interrupt, interrupt flags are found here SPI2:MIS"]
    Spi2Comb = 122,
    #[doc = "121: Always asserted"]
    AlwaysActive = 121,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AonRtcUpd = 119,
    #[doc = "115: AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    AuxAdcIrq = 115,
    #[doc = "114: Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    AuxObsmux0 = 114,
    #[doc = "113: AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    AuxAdcFifoAlmostFull = 113,
    #[doc = "112: AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    AuxAdcDone = 112,
    #[doc = "111: Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AuxSmphAutotakeDone = 111,
    #[doc = "110: AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    AuxTimer1Ev = 110,
    #[doc = "109: AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    AuxTimer0Ev = 109,
    #[doc = "108: AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AuxTdcDone = 108,
    #[doc = "107: AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AuxCompb = 107,
    #[doc = "106: AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AuxCompa = 106,
    #[doc = "105: AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    AuxAonWuEv = 105,
    #[doc = "92: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PortEvent7 = 92,
    #[doc = "91: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PortEvent6 = 91,
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
    #[doc = "60: AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    AuxTimer2Pulse = 60,
    #[doc = "59: AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    AuxTimer2Ev3 = 59,
    #[doc = "58: AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    AuxTimer2Ev2 = 58,
    #[doc = "57: AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    AuxTimer2Ev1 = 57,
    #[doc = "56: AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    AuxTimer2Ev0 = 56,
    #[doc = "37: UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    Uart1Comb = 37,
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    Uart0Comb = 36,
    #[doc = "35: SPI1 combined interrupt, interrupt flags are found here SPI1:MIS"]
    Spi1Comb = 35,
    #[doc = "34: SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    Spi0Comb = 34,
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
    #[doc = "6: Combined event from Oscillator control"]
    OscComb = 6,
    #[doc = "5: Combined event from battery monitor"]
    BatmonComb = 5,
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
#[doc = "Field `EV` reader - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            126 => Some(Ev::I2c1Irq),
            125 => Some(Ev::Uart3Comb),
            124 => Some(Ev::Uart2Comb),
            123 => Some(Ev::Spi3Comb),
            122 => Some(Ev::Spi2Comb),
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
            92 => Some(Ev::PortEvent7),
            91 => Some(Ev::PortEvent6),
            68 => Some(Ev::Gpt3bCmp),
            67 => Some(Ev::Gpt3aCmp),
            66 => Some(Ev::Gpt2bCmp),
            65 => Some(Ev::Gpt2aCmp),
            64 => Some(Ev::Gpt1bCmp),
            63 => Some(Ev::Gpt1aCmp),
            62 => Some(Ev::Gpt0bCmp),
            61 => Some(Ev::Gpt0aCmp),
            60 => Some(Ev::AuxTimer2Pulse),
            59 => Some(Ev::AuxTimer2Ev3),
            58 => Some(Ev::AuxTimer2Ev2),
            57 => Some(Ev::AuxTimer2Ev1),
            56 => Some(Ev::AuxTimer2Ev0),
            37 => Some(Ev::Uart1Comb),
            36 => Some(Ev::Uart0Comb),
            35 => Some(Ev::Spi1Comb),
            34 => Some(Ev::Spi0Comb),
            30 => Some(Ev::RfcCpe1),
            27 => Some(Ev::RfcCpe0),
            26 => Some(Ev::RfcHwComb),
            25 => Some(Ev::RfcCmdAck),
            21 => Some(Ev::Flash),
            11 => Some(Ev::AuxComb),
            9 => Some(Ev::I2cIrq),
            7 => Some(Ev::AonRtcComb),
            6 => Some(Ev::OscComb),
            5 => Some(Ev::BatmonComb),
            4 => Some(Ev::AonGpioEdge),
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Interrupt event from I2C1"]
    #[inline(always)]
    pub fn is_i2c1_irq(&self) -> bool {
        *self == Ev::I2c1Irq
    }
    #[doc = "UART3 combined interrupt, interrupt flags are found here UART3:MIS"]
    #[inline(always)]
    pub fn is_uart3_comb(&self) -> bool {
        *self == Ev::Uart3Comb
    }
    #[doc = "UART2 combined interrupt, interrupt flags are found here UART2:MIS"]
    #[inline(always)]
    pub fn is_uart2_comb(&self) -> bool {
        *self == Ev::Uart2Comb
    }
    #[doc = "SPI3 combined interrupt, interrupt flags are found here SPI3:MIS"]
    #[inline(always)]
    pub fn is_spi3_comb(&self) -> bool {
        *self == Ev::Spi3Comb
    }
    #[doc = "SPI2 combined interrupt, interrupt flags are found here SPI2:MIS"]
    #[inline(always)]
    pub fn is_spi2_comb(&self) -> bool {
        *self == Ev::Spi2Comb
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
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == Ev::AuxAdcIrq
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn is_aux_obsmux0(&self) -> bool {
        *self == Ev::AuxObsmux0
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == Ev::AuxAdcFifoAlmostFull
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == Ev::AuxAdcDone
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == Ev::AuxSmphAutotakeDone
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == Ev::AuxTimer1Ev
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == Ev::AuxTimer0Ev
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
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
    #[doc = "AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    #[inline(always)]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == Ev::AuxAonWuEv
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event7(&self) -> bool {
        *self == Ev::PortEvent7
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event6(&self) -> bool {
        *self == Ev::PortEvent6
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
    #[doc = "AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == Ev::AuxTimer2Pulse
    }
    #[doc = "AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == Ev::AuxTimer2Ev3
    }
    #[doc = "AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == Ev::AuxTimer2Ev2
    }
    #[doc = "AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == Ev::AuxTimer2Ev1
    }
    #[doc = "AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == Ev::AuxTimer2Ev0
    }
    #[doc = "UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    #[inline(always)]
    pub fn is_uart1_comb(&self) -> bool {
        *self == Ev::Uart1Comb
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == Ev::Uart0Comb
    }
    #[doc = "SPI1 combined interrupt, interrupt flags are found here SPI1:MIS"]
    #[inline(always)]
    pub fn is_spi1_comb(&self) -> bool {
        *self == Ev::Spi1Comb
    }
    #[doc = "SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    #[inline(always)]
    pub fn is_spi0_comb(&self) -> bool {
        *self == Ev::Spi0Comb
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
    #[doc = "Combined event from Oscillator control"]
    #[inline(always)]
    pub fn is_osc_comb(&self) -> bool {
        *self == Ev::OscComb
    }
    #[doc = "Combined event from battery monitor"]
    #[inline(always)]
    pub fn is_batmon_comb(&self) -> bool {
        *self == Ev::BatmonComb
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
#[doc = "Field `EV` writer - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt event from I2C1"]
    #[inline(always)]
    pub fn i2c1_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::I2c1Irq)
    }
    #[doc = "UART3 combined interrupt, interrupt flags are found here UART3:MIS"]
    #[inline(always)]
    pub fn uart3_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart3Comb)
    }
    #[doc = "UART2 combined interrupt, interrupt flags are found here UART2:MIS"]
    #[inline(always)]
    pub fn uart2_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart2Comb)
    }
    #[doc = "SPI3 combined interrupt, interrupt flags are found here SPI3:MIS"]
    #[inline(always)]
    pub fn spi3_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Spi3Comb)
    }
    #[doc = "SPI2 combined interrupt, interrupt flags are found here SPI2:MIS"]
    #[inline(always)]
    pub fn spi2_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Spi2Comb)
    }
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
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAdcIrq)
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn aux_obsmux0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxObsmux0)
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAdcFifoAlmostFull)
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAdcDone)
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxSmphAutotakeDone)
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer1Ev)
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer0Ev)
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
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
    #[doc = "AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    #[inline(always)]
    pub fn aux_aon_wu_ev(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxAonWuEv)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    #[inline(always)]
    pub fn port_event7(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent7)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    #[inline(always)]
    pub fn port_event6(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent6)
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
    #[doc = "AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer2Pulse)
    }
    #[doc = "AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer2Ev3)
    }
    #[doc = "AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer2Ev2)
    }
    #[doc = "AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer2Ev1)
    }
    #[doc = "AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxTimer2Ev0)
    }
    #[doc = "UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    #[inline(always)]
    pub fn uart1_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart1Comb)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0Comb)
    }
    #[doc = "SPI1 combined interrupt, interrupt flags are found here SPI1:MIS"]
    #[inline(always)]
    pub fn spi1_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Spi1Comb)
    }
    #[doc = "SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    #[inline(always)]
    pub fn spi0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Spi0Comb)
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
    #[doc = "Combined event from Oscillator control"]
    #[inline(always)]
    pub fn osc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::OscComb)
    }
    #[doc = "Combined event from battery monitor"]
    #[inline(always)]
    pub fn batmon_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::BatmonComb)
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
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Gpt3bcaptselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for GPT3 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt3bcaptsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt3bcaptsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpt3bcaptselSpec;
impl crate::RegisterSpec for Gpt3bcaptselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpt3bcaptsel::R`](R) reader structure"]
impl crate::Readable for Gpt3bcaptselSpec {}
#[doc = "`write(|w| ..)` method takes [`gpt3bcaptsel::W`](W) writer structure"]
impl crate::Writable for Gpt3bcaptselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPT3BCAPTSEL to value 0x5c"]
impl crate::Resettable for Gpt3bcaptselSpec {
    const RESET_VALUE: u32 = 0x5c;
}
