#[doc = "Register `UDMACH14BSEL` reader"]
pub type R = crate::R<Udmach14bselSpec>;
#[doc = "Register `UDMACH14BSEL` writer"]
pub type W = crate::W<Udmach14bselSpec>;
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "121: Always asserted"]
    AlwaysActive = 121,
    #[doc = "120: CPU halted"]
    CpuHalted = 120,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AonRtcUpd = 119,
    #[doc = "118: DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AuxDmabreq = 118,
    #[doc = "117: DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AuxDmasreq = 117,
    #[doc = "116: DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AuxSwDmabreq = 116,
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
    #[doc = "104: TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    TrngIrq = 104,
    #[doc = "103: Software event 3, triggered by SWEV.SWEV3"]
    Swev3 = 103,
    #[doc = "102: Software event 2, triggered by SWEV.SWEV2"]
    Swev2 = 102,
    #[doc = "101: Software event 1, triggered by SWEV.SWEV1"]
    Swev1 = 101,
    #[doc = "100: Software event 0, triggered by SWEV.SWEV0"]
    Swev0 = 100,
    #[doc = "99: Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    WdtNmi = 99,
    #[doc = "94: CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CryptoDmaDoneIrq = 94,
    #[doc = "93: CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CryptoResultAvailIrq = 93,
    #[doc = "92: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PortEvent7 = 92,
    #[doc = "91: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PortEvent6 = 91,
    #[doc = "90: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PortEvent5 = 90,
    #[doc = "89: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PortEvent4 = 89,
    #[doc = "88: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PortEvent3 = 88,
    #[doc = "87: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PortEvent2 = 87,
    #[doc = "86: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PortEvent1 = 86,
    #[doc = "85: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    PortEvent0 = 85,
    #[doc = "84: GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    Gpt3bDmabreq = 84,
    #[doc = "83: GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    Gpt3aDmabreq = 83,
    #[doc = "82: GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    Gpt2bDmabreq = 82,
    #[doc = "81: GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    Gpt2aDmabreq = 81,
    #[doc = "80: GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    Gpt1bDmabreq = 80,
    #[doc = "79: GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    Gpt1aDmabreq = 79,
    #[doc = "78: GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    Gpt0bDmabreq = 78,
    #[doc = "77: GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    Gpt0aDmabreq = 77,
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
    #[doc = "51: UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    Uart0TxDmasreq = 51,
    #[doc = "50: UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    Uart0TxDmabreq = 50,
    #[doc = "49: UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    Uart0RxDmasreq = 49,
    #[doc = "48: UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    Uart0RxDmabreq = 48,
    #[doc = "43: SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    Ssi0TxDmasreq = 43,
    #[doc = "42: SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    Ssi0TxDmabreq = 42,
    #[doc = "41: SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    Ssi0RxDmasreq = 41,
    #[doc = "40: SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    Ssi0RxDmabreq = 40,
    #[doc = "39: Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    DmaDoneComb = 39,
    #[doc = "38: DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    DmaErr = 38,
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    Uart0Comb = 36,
    #[doc = "34: SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    Ssi0Comb = 34,
    #[doc = "30: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RfcCpe1 = 30,
    #[doc = "29: AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    AuxSwev1 = 29,
    #[doc = "27: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RfcCpe0 = 27,
    #[doc = "26: Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RfcHwComb = 26,
    #[doc = "25: RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RfcCmdAck = 25,
    #[doc = "24: Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WdtIrq = 24,
    #[doc = "22: DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DmaCh18Done = 22,
    #[doc = "21: FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    Flash = 21,
    #[doc = "20: DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DmaCh0Done = 20,
    #[doc = "19: GPT1B interrupt event, controlled by GPT1:TBMR"]
    Gpt1b = 19,
    #[doc = "18: GPT1A interrupt event, controlled by GPT1:TAMR"]
    Gpt1a = 18,
    #[doc = "17: GPT0B interrupt event, controlled by GPT0:TBMR"]
    Gpt0b = 17,
    #[doc = "16: GPT0A interrupt event, controlled by GPT0:TAMR"]
    Gpt0a = 16,
    #[doc = "15: GPT3B interrupt event, controlled by GPT3:TBMR"]
    Gpt3b = 15,
    #[doc = "14: GPT3A interrupt event, controlled by GPT3:TAMR"]
    Gpt3a = 14,
    #[doc = "13: GPT2B interrupt event, controlled by GPT2:TBMR"]
    Gpt2b = 13,
    #[doc = "12: GPT2A interrupt event, controlled by GPT2:TAMR"]
    Gpt2a = 12,
    #[doc = "11: AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AuxComb = 11,
    #[doc = "10: AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AonAuxSwev0 = 10,
    #[doc = "9: Interrupt event from I2C"]
    I2cIrq = 9,
    #[doc = "8: Interrupt event from I2S"]
    I2sIrq = 8,
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AonRtcComb = 7,
    #[doc = "6: Combined event from Oscillator control"]
    OscComb = 6,
    #[doc = "5: Combined event from battery monitor"]
    BatmonComb = 5,
    #[doc = "4: Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    AonGpioEdge = 4,
    #[doc = "3: AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AonProg2 = 3,
    #[doc = "2: AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AonProg1 = 2,
    #[doc = "1: AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    AonProg0 = 1,
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
            120 => Some(Ev::CpuHalted),
            119 => Some(Ev::AonRtcUpd),
            118 => Some(Ev::AuxDmabreq),
            117 => Some(Ev::AuxDmasreq),
            116 => Some(Ev::AuxSwDmabreq),
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
            104 => Some(Ev::TrngIrq),
            103 => Some(Ev::Swev3),
            102 => Some(Ev::Swev2),
            101 => Some(Ev::Swev1),
            100 => Some(Ev::Swev0),
            99 => Some(Ev::WdtNmi),
            94 => Some(Ev::CryptoDmaDoneIrq),
            93 => Some(Ev::CryptoResultAvailIrq),
            92 => Some(Ev::PortEvent7),
            91 => Some(Ev::PortEvent6),
            90 => Some(Ev::PortEvent5),
            89 => Some(Ev::PortEvent4),
            88 => Some(Ev::PortEvent3),
            87 => Some(Ev::PortEvent2),
            86 => Some(Ev::PortEvent1),
            85 => Some(Ev::PortEvent0),
            84 => Some(Ev::Gpt3bDmabreq),
            83 => Some(Ev::Gpt3aDmabreq),
            82 => Some(Ev::Gpt2bDmabreq),
            81 => Some(Ev::Gpt2aDmabreq),
            80 => Some(Ev::Gpt1bDmabreq),
            79 => Some(Ev::Gpt1aDmabreq),
            78 => Some(Ev::Gpt0bDmabreq),
            77 => Some(Ev::Gpt0aDmabreq),
            68 => Some(Ev::Gpt3bCmp),
            67 => Some(Ev::Gpt3aCmp),
            66 => Some(Ev::Gpt2bCmp),
            65 => Some(Ev::Gpt2aCmp),
            64 => Some(Ev::Gpt1bCmp),
            63 => Some(Ev::Gpt1aCmp),
            62 => Some(Ev::Gpt0bCmp),
            61 => Some(Ev::Gpt0aCmp),
            51 => Some(Ev::Uart0TxDmasreq),
            50 => Some(Ev::Uart0TxDmabreq),
            49 => Some(Ev::Uart0RxDmasreq),
            48 => Some(Ev::Uart0RxDmabreq),
            43 => Some(Ev::Ssi0TxDmasreq),
            42 => Some(Ev::Ssi0TxDmabreq),
            41 => Some(Ev::Ssi0RxDmasreq),
            40 => Some(Ev::Ssi0RxDmabreq),
            39 => Some(Ev::DmaDoneComb),
            38 => Some(Ev::DmaErr),
            36 => Some(Ev::Uart0Comb),
            34 => Some(Ev::Ssi0Comb),
            30 => Some(Ev::RfcCpe1),
            29 => Some(Ev::AuxSwev1),
            27 => Some(Ev::RfcCpe0),
            26 => Some(Ev::RfcHwComb),
            25 => Some(Ev::RfcCmdAck),
            24 => Some(Ev::WdtIrq),
            22 => Some(Ev::DmaCh18Done),
            21 => Some(Ev::Flash),
            20 => Some(Ev::DmaCh0Done),
            19 => Some(Ev::Gpt1b),
            18 => Some(Ev::Gpt1a),
            17 => Some(Ev::Gpt0b),
            16 => Some(Ev::Gpt0a),
            15 => Some(Ev::Gpt3b),
            14 => Some(Ev::Gpt3a),
            13 => Some(Ev::Gpt2b),
            12 => Some(Ev::Gpt2a),
            11 => Some(Ev::AuxComb),
            10 => Some(Ev::AonAuxSwev0),
            9 => Some(Ev::I2cIrq),
            8 => Some(Ev::I2sIrq),
            7 => Some(Ev::AonRtcComb),
            6 => Some(Ev::OscComb),
            5 => Some(Ev::BatmonComb),
            4 => Some(Ev::AonGpioEdge),
            3 => Some(Ev::AonProg2),
            2 => Some(Ev::AonProg1),
            1 => Some(Ev::AonProg0),
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == Ev::AlwaysActive
    }
    #[doc = "CPU halted"]
    #[inline(always)]
    pub fn is_cpu_halted(&self) -> bool {
        *self == Ev::CpuHalted
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == Ev::AonRtcUpd
    }
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn is_aux_dmabreq(&self) -> bool {
        *self == Ev::AuxDmabreq
    }
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn is_aux_dmasreq(&self) -> bool {
        *self == Ev::AuxDmasreq
    }
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline(always)]
    pub fn is_aux_sw_dmabreq(&self) -> bool {
        *self == Ev::AuxSwDmabreq
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
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    #[inline(always)]
    pub fn is_trng_irq(&self) -> bool {
        *self == Ev::TrngIrq
    }
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline(always)]
    pub fn is_swev3(&self) -> bool {
        *self == Ev::Swev3
    }
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    #[inline(always)]
    pub fn is_swev2(&self) -> bool {
        *self == Ev::Swev2
    }
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    #[inline(always)]
    pub fn is_swev1(&self) -> bool {
        *self == Ev::Swev1
    }
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    #[inline(always)]
    pub fn is_swev0(&self) -> bool {
        *self == Ev::Swev0
    }
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    #[inline(always)]
    pub fn is_wdt_nmi(&self) -> bool {
        *self == Ev::WdtNmi
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline(always)]
    pub fn is_crypto_dma_done_irq(&self) -> bool {
        *self == Ev::CryptoDmaDoneIrq
    }
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline(always)]
    pub fn is_crypto_result_avail_irq(&self) -> bool {
        *self == Ev::CryptoResultAvailIrq
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
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event5(&self) -> bool {
        *self == Ev::PortEvent5
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event4(&self) -> bool {
        *self == Ev::PortEvent4
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
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event1(&self) -> bool {
        *self == Ev::PortEvent1
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    #[inline(always)]
    pub fn is_port_event0(&self) -> bool {
        *self == Ev::PortEvent0
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn is_gpt3b_dmabreq(&self) -> bool {
        *self == Ev::Gpt3bDmabreq
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn is_gpt3a_dmabreq(&self) -> bool {
        *self == Ev::Gpt3aDmabreq
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn is_gpt2b_dmabreq(&self) -> bool {
        *self == Ev::Gpt2bDmabreq
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn is_gpt2a_dmabreq(&self) -> bool {
        *self == Ev::Gpt2aDmabreq
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn is_gpt1b_dmabreq(&self) -> bool {
        *self == Ev::Gpt1bDmabreq
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn is_gpt1a_dmabreq(&self) -> bool {
        *self == Ev::Gpt1aDmabreq
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn is_gpt0b_dmabreq(&self) -> bool {
        *self == Ev::Gpt0bDmabreq
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn is_gpt0a_dmabreq(&self) -> bool {
        *self == Ev::Gpt0aDmabreq
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
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn is_uart0_tx_dmasreq(&self) -> bool {
        *self == Ev::Uart0TxDmasreq
    }
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn is_uart0_tx_dmabreq(&self) -> bool {
        *self == Ev::Uart0TxDmabreq
    }
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart0_rx_dmasreq(&self) -> bool {
        *self == Ev::Uart0RxDmasreq
    }
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn is_uart0_rx_dmabreq(&self) -> bool {
        *self == Ev::Uart0RxDmabreq
    }
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_ssi0_tx_dmasreq(&self) -> bool {
        *self == Ev::Ssi0TxDmasreq
    }
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn is_ssi0_tx_dmabreq(&self) -> bool {
        *self == Ev::Ssi0TxDmabreq
    }
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_ssi0_rx_dmasreq(&self) -> bool {
        *self == Ev::Ssi0RxDmasreq
    }
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_ssi0_rx_dmabreq(&self) -> bool {
        *self == Ev::Ssi0RxDmabreq
    }
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline(always)]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == Ev::DmaDoneComb
    }
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    #[inline(always)]
    pub fn is_dma_err(&self) -> bool {
        *self == Ev::DmaErr
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == Ev::Uart0Comb
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
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Ev::AuxSwev1
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
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    #[inline(always)]
    pub fn is_wdt_irq(&self) -> bool {
        *self == Ev::WdtIrq
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn is_dma_ch18_done(&self) -> bool {
        *self == Ev::DmaCh18Done
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == Ev::Flash
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn is_dma_ch0_done(&self) -> bool {
        *self == Ev::DmaCh0Done
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn is_gpt1b(&self) -> bool {
        *self == Ev::Gpt1b
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn is_gpt1a(&self) -> bool {
        *self == Ev::Gpt1a
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn is_gpt0b(&self) -> bool {
        *self == Ev::Gpt0b
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn is_gpt0a(&self) -> bool {
        *self == Ev::Gpt0a
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn is_gpt3b(&self) -> bool {
        *self == Ev::Gpt3b
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn is_gpt3a(&self) -> bool {
        *self == Ev::Gpt3a
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn is_gpt2b(&self) -> bool {
        *self == Ev::Gpt2b
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn is_gpt2a(&self) -> bool {
        *self == Ev::Gpt2a
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn is_aux_comb(&self) -> bool {
        *self == Ev::AuxComb
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn is_aon_aux_swev0(&self) -> bool {
        *self == Ev::AonAuxSwev0
    }
    #[doc = "Interrupt event from I2C"]
    #[inline(always)]
    pub fn is_i2c_irq(&self) -> bool {
        *self == Ev::I2cIrq
    }
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn is_i2s_irq(&self) -> bool {
        *self == Ev::I2sIrq
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
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline(always)]
    pub fn is_aon_prog2(&self) -> bool {
        *self == Ev::AonProg2
    }
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    #[inline(always)]
    pub fn is_aon_prog1(&self) -> bool {
        *self == Ev::AonProg1
    }
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    #[inline(always)]
    pub fn is_aon_prog0(&self) -> bool {
        *self == Ev::AonProg0
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
    #[doc = "CPU halted"]
    #[inline(always)]
    pub fn cpu_halted(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::CpuHalted)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonRtcUpd)
    }
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn aux_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxDmabreq)
    }
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn aux_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxDmasreq)
    }
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline(always)]
    pub fn aux_sw_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxSwDmabreq)
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
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    #[inline(always)]
    pub fn trng_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::TrngIrq)
    }
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline(always)]
    pub fn swev3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Swev3)
    }
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    #[inline(always)]
    pub fn swev2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Swev2)
    }
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    #[inline(always)]
    pub fn swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Swev1)
    }
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    #[inline(always)]
    pub fn swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Swev0)
    }
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    #[inline(always)]
    pub fn wdt_nmi(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::WdtNmi)
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline(always)]
    pub fn crypto_dma_done_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::CryptoDmaDoneIrq)
    }
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline(always)]
    pub fn crypto_result_avail_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::CryptoResultAvailIrq)
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
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn port_event5(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent5)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn port_event4(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent4)
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
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    #[inline(always)]
    pub fn port_event1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent1)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    #[inline(always)]
    pub fn port_event0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::PortEvent0)
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3bDmabreq)
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3aDmabreq)
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2bDmabreq)
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2aDmabreq)
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1bDmabreq)
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1aDmabreq)
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0bDmabreq)
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0aDmabreq)
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
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart0_tx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0TxDmasreq)
    }
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline(always)]
    pub fn uart0_tx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0TxDmabreq)
    }
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart0_rx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0RxDmasreq)
    }
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline(always)]
    pub fn uart0_rx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0RxDmabreq)
    }
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi0_tx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi0TxDmasreq)
    }
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline(always)]
    pub fn ssi0_tx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi0TxDmabreq)
    }
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn ssi0_rx_dmasreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi0RxDmasreq)
    }
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn ssi0_rx_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi0RxDmabreq)
    }
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline(always)]
    pub fn dma_done_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::DmaDoneComb)
    }
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    #[inline(always)]
    pub fn dma_err(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::DmaErr)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart0Comb)
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
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn aux_swev1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxSwev1)
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
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    #[inline(always)]
    pub fn wdt_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::WdtIrq)
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch18_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::DmaCh18Done)
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Flash)
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline(always)]
    pub fn dma_ch0_done(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::DmaCh0Done)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn gpt1b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1b)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn gpt1a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1a)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn gpt0b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0b)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn gpt0a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0a)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn gpt3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3b)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn gpt3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3a)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn gpt2b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2b)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn gpt2a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2a)
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxComb)
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline(always)]
    pub fn aon_aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonAuxSwev0)
    }
    #[doc = "Interrupt event from I2C"]
    #[inline(always)]
    pub fn i2c_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::I2cIrq)
    }
    #[doc = "Interrupt event from I2S"]
    #[inline(always)]
    pub fn i2s_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::I2sIrq)
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
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline(always)]
    pub fn aon_prog2(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonProg2)
    }
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    #[inline(always)]
    pub fn aon_prog1(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonProg1)
    }
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    #[inline(always)]
    pub fn aon_prog0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AonProg0)
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
    pub fn ev(&mut self) -> EvW<Udmach14bselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 14 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach14bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach14bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach14bselSpec;
impl crate::RegisterSpec for Udmach14bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach14bsel::R`](R) reader structure"]
impl crate::Readable for Udmach14bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach14bsel::W`](W) writer structure"]
impl crate::Writable for Udmach14bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH14BSEL to value 0x01"]
impl crate::Resettable for Udmach14bselSpec {
    const RESET_VALUE: u32 = 0x01;
}
