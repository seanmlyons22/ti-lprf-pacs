#[doc = "Register `GPT2ACAPTSEL` reader"]
pub struct R(crate::R<GPT2ACAPTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPT2ACAPTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPT2ACAPTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPT2ACAPTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPT2ACAPTSEL` writer"]
pub struct W(crate::W<GPT2ACAPTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPT2ACAPTSEL_SPEC>;
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
impl From<crate::W<GPT2ACAPTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPT2ACAPTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 89"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "119: RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD = 119,
    #[doc = "115: AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_ADC_IRQ = 115,
    #[doc = "114: Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    AUX_OBSMUX0 = 114,
    #[doc = "113: AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL = 113,
    #[doc = "112: AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    AUX_ADC_DONE = 112,
    #[doc = "111: Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AUX_SMPH_AUTOTAKE_DONE = 111,
    #[doc = "110: AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    AUX_TIMER1_EV = 110,
    #[doc = "109: AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    AUX_TIMER0_EV = 109,
    #[doc = "108: AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AUX_TDC_DONE = 108,
    #[doc = "107: AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AUX_COMPB = 107,
    #[doc = "106: AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AUX_COMPA = 106,
    #[doc = "105: AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    AUX_AON_WU_EV = 105,
    #[doc = "90: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5 = 90,
    #[doc = "89: Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4 = 89,
    #[doc = "68: GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    GPT3B_CMP = 68,
    #[doc = "67: GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    GPT3A_CMP = 67,
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    GPT2B_CMP = 66,
    #[doc = "65: GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    GPT2A_CMP = 65,
    #[doc = "64: GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    GPT1B_CMP = 64,
    #[doc = "63: GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    GPT1A_CMP = 63,
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP = 62,
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    GPT0A_CMP = 61,
    #[doc = "60: AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    AUX_TIMER2_PULSE = 60,
    #[doc = "59: AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    AUX_TIMER2_EV3 = 59,
    #[doc = "58: AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    AUX_TIMER2_EV2 = 58,
    #[doc = "57: AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    AUX_TIMER2_EV1 = 57,
    #[doc = "56: AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 56,
    #[doc = "37: UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    UART1_COMB = 37,
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB = 36,
    #[doc = "35: SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB = 35,
    #[doc = "34: SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB = 34,
    #[doc = "30: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RFC_CPE_1 = 30,
    #[doc = "27: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0 = 27,
    #[doc = "26: Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB = 26,
    #[doc = "25: RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RFC_CMD_ACK = 25,
    #[doc = "21: FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    FLASH = 21,
    #[doc = "11: AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB = 11,
    #[doc = "9: Interrupt event from I2C"]
    I2C_IRQ = 9,
    #[doc = "7: Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB = 7,
    #[doc = "6: Combined event from Oscillator control"]
    OSC_COMB = 6,
    #[doc = "5: Combined event from battery monitor"]
    BATMON_COMB = 5,
    #[doc = "4: Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE = 4,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            121 => Some(EV_A::ALWAYS_ACTIVE),
            119 => Some(EV_A::AON_RTC_UPD),
            115 => Some(EV_A::AUX_ADC_IRQ),
            114 => Some(EV_A::AUX_OBSMUX0),
            113 => Some(EV_A::AUX_ADC_FIFO_ALMOST_FULL),
            112 => Some(EV_A::AUX_ADC_DONE),
            111 => Some(EV_A::AUX_SMPH_AUTOTAKE_DONE),
            110 => Some(EV_A::AUX_TIMER1_EV),
            109 => Some(EV_A::AUX_TIMER0_EV),
            108 => Some(EV_A::AUX_TDC_DONE),
            107 => Some(EV_A::AUX_COMPB),
            106 => Some(EV_A::AUX_COMPA),
            105 => Some(EV_A::AUX_AON_WU_EV),
            90 => Some(EV_A::PORT_EVENT5),
            89 => Some(EV_A::PORT_EVENT4),
            68 => Some(EV_A::GPT3B_CMP),
            67 => Some(EV_A::GPT3A_CMP),
            66 => Some(EV_A::GPT2B_CMP),
            65 => Some(EV_A::GPT2A_CMP),
            64 => Some(EV_A::GPT1B_CMP),
            63 => Some(EV_A::GPT1A_CMP),
            62 => Some(EV_A::GPT0B_CMP),
            61 => Some(EV_A::GPT0A_CMP),
            60 => Some(EV_A::AUX_TIMER2_PULSE),
            59 => Some(EV_A::AUX_TIMER2_EV3),
            58 => Some(EV_A::AUX_TIMER2_EV2),
            57 => Some(EV_A::AUX_TIMER2_EV1),
            56 => Some(EV_A::AUX_TIMER2_EV0),
            37 => Some(EV_A::UART1_COMB),
            36 => Some(EV_A::UART0_COMB),
            35 => Some(EV_A::SSI1_COMB),
            34 => Some(EV_A::SSI0_COMB),
            30 => Some(EV_A::RFC_CPE_1),
            27 => Some(EV_A::RFC_CPE_0),
            26 => Some(EV_A::RFC_HW_COMB),
            25 => Some(EV_A::RFC_CMD_ACK),
            21 => Some(EV_A::FLASH),
            11 => Some(EV_A::AUX_COMB),
            9 => Some(EV_A::I2C_IRQ),
            7 => Some(EV_A::AON_RTC_COMB),
            6 => Some(EV_A::OSC_COMB),
            5 => Some(EV_A::BATMON_COMB),
            4 => Some(EV_A::AON_GPIO_EDGE),
            0 => Some(EV_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AON_RTC_UPD`"]
    #[inline(always)]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == EV_A::AON_RTC_UPD
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline(always)]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == EV_A::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_OBSMUX0`"]
    #[inline(always)]
    pub fn is_aux_obsmux0(&self) -> bool {
        *self == EV_A::AUX_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EV_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline(always)]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EV_A::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline(always)]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EV_A::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EV_A::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EV_A::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_AON_WU_EV`"]
    #[inline(always)]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == EV_A::AUX_AON_WU_EV
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT5`"]
    #[inline(always)]
    pub fn is_port_event5(&self) -> bool {
        *self == EV_A::PORT_EVENT5
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT4`"]
    #[inline(always)]
    pub fn is_port_event4(&self) -> bool {
        *self == EV_A::PORT_EVENT4
    }
    #[doc = "Checks if the value of the field is `GPT3B_CMP`"]
    #[inline(always)]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == EV_A::GPT3B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT3A_CMP`"]
    #[inline(always)]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == EV_A::GPT3A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2B_CMP`"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == EV_A::GPT2B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2A_CMP`"]
    #[inline(always)]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == EV_A::GPT2A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1B_CMP`"]
    #[inline(always)]
    pub fn is_gpt1b_cmp(&self) -> bool {
        *self == EV_A::GPT1B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1A_CMP`"]
    #[inline(always)]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == EV_A::GPT1A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0B_CMP`"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == EV_A::GPT0B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0A_CMP`"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == EV_A::GPT0A_CMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == EV_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == EV_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `UART1_COMB`"]
    #[inline(always)]
    pub fn is_uart1_comb(&self) -> bool {
        *self == EV_A::UART1_COMB
    }
    #[doc = "Checks if the value of the field is `UART0_COMB`"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == EV_A::UART0_COMB
    }
    #[doc = "Checks if the value of the field is `SSI1_COMB`"]
    #[inline(always)]
    pub fn is_ssi1_comb(&self) -> bool {
        *self == EV_A::SSI1_COMB
    }
    #[doc = "Checks if the value of the field is `SSI0_COMB`"]
    #[inline(always)]
    pub fn is_ssi0_comb(&self) -> bool {
        *self == EV_A::SSI0_COMB
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_1`"]
    #[inline(always)]
    pub fn is_rfc_cpe_1(&self) -> bool {
        *self == EV_A::RFC_CPE_1
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_0`"]
    #[inline(always)]
    pub fn is_rfc_cpe_0(&self) -> bool {
        *self == EV_A::RFC_CPE_0
    }
    #[doc = "Checks if the value of the field is `RFC_HW_COMB`"]
    #[inline(always)]
    pub fn is_rfc_hw_comb(&self) -> bool {
        *self == EV_A::RFC_HW_COMB
    }
    #[doc = "Checks if the value of the field is `RFC_CMD_ACK`"]
    #[inline(always)]
    pub fn is_rfc_cmd_ack(&self) -> bool {
        *self == EV_A::RFC_CMD_ACK
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == EV_A::FLASH
    }
    #[doc = "Checks if the value of the field is `AUX_COMB`"]
    #[inline(always)]
    pub fn is_aux_comb(&self) -> bool {
        *self == EV_A::AUX_COMB
    }
    #[doc = "Checks if the value of the field is `I2C_IRQ`"]
    #[inline(always)]
    pub fn is_i2c_irq(&self) -> bool {
        *self == EV_A::I2C_IRQ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_COMB`"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == EV_A::AON_RTC_COMB
    }
    #[doc = "Checks if the value of the field is `OSC_COMB`"]
    #[inline(always)]
    pub fn is_osc_comb(&self) -> bool {
        *self == EV_A::OSC_COMB
    }
    #[doc = "Checks if the value of the field is `BATMON_COMB`"]
    #[inline(always)]
    pub fn is_batmon_comb(&self) -> bool {
        *self == EV_A::BATMON_COMB
    }
    #[doc = "Checks if the value of the field is `AON_GPIO_EDGE`"]
    #[inline(always)]
    pub fn is_aon_gpio_edge(&self) -> bool {
        *self == EV_A::AON_GPIO_EDGE
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EV_A::NONE
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPT2ACAPTSEL_SPEC, u8, EV_A, 7, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline(always)]
    pub fn aon_rtc_upd(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_UPD)
    }
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_IRQ)
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.MCU_OBSMUX0"]
    #[inline(always)]
    pub fn aux_obsmux0(self) -> &'a mut W {
        self.variant(EV_A::AUX_OBSMUX0)
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_ADC_DONE)
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER1_EV)
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER0_EV)
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV_A::AUX_TDC_DONE)
    }
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMPB)
    }
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMPA)
    }
    #[doc = "AON wakeup event, the corresponding flag is here AUX_EVCTL:EVTOMCUFLAGS.AUX_WU_EV"]
    #[inline(always)]
    pub fn aux_aon_wu_ev(self) -> &'a mut W {
        self.variant(EV_A::AUX_AON_WU_EV)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn port_event5(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT5)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline(always)]
    pub fn port_event4(self) -> &'a mut W {
        self.variant(EV_A::PORT_EVENT4)
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt3b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT3B_CMP)
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt3a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT3A_CMP)
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt2b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2B_CMP)
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt2a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2A_CMP)
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt1b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT1B_CMP)
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt1a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT1A_CMP)
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt0b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0B_CMP)
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT0A_CMP)
    }
    #[doc = "AUX Timer2 pulse, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_PULSE)
    }
    #[doc = "AUX Timer2 event 3, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV3)
    }
    #[doc = "AUX Timer2 event 2, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV2)
    }
    #[doc = "AUX Timer2 event 1, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV1)
    }
    #[doc = "AUX Timer2 event 0, corresponding to flag AUX_EVCTL:EVTOMCUFLAGS.AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(EV_A::AUX_TIMER2_EV0)
    }
    #[doc = "UART1 combined interrupt, interrupt flags are found here UART1:MIS"]
    #[inline(always)]
    pub fn uart1_comb(self) -> &'a mut W {
        self.variant(EV_A::UART1_COMB)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut W {
        self.variant(EV_A::UART0_COMB)
    }
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn ssi1_comb(self) -> &'a mut W {
        self.variant(EV_A::SSI1_COMB)
    }
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    #[inline(always)]
    pub fn ssi0_comb(self) -> &'a mut W {
        self.variant(EV_A::SSI0_COMB)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    #[inline(always)]
    pub fn rfc_cpe_1(self) -> &'a mut W {
        self.variant(EV_A::RFC_CPE_1)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    #[inline(always)]
    pub fn rfc_cpe_0(self) -> &'a mut W {
        self.variant(EV_A::RFC_CPE_0)
    }
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline(always)]
    pub fn rfc_hw_comb(self) -> &'a mut W {
        self.variant(EV_A::RFC_HW_COMB)
    }
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    #[inline(always)]
    pub fn rfc_cmd_ack(self) -> &'a mut W {
        self.variant(EV_A::RFC_CMD_ACK)
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(EV_A::FLASH)
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_comb(self) -> &'a mut W {
        self.variant(EV_A::AUX_COMB)
    }
    #[doc = "Interrupt event from I2C"]
    #[inline(always)]
    pub fn i2c_irq(self) -> &'a mut W {
        self.variant(EV_A::I2C_IRQ)
    }
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline(always)]
    pub fn aon_rtc_comb(self) -> &'a mut W {
        self.variant(EV_A::AON_RTC_COMB)
    }
    #[doc = "Combined event from Oscillator control"]
    #[inline(always)]
    pub fn osc_comb(self) -> &'a mut W {
        self.variant(EV_A::OSC_COMB)
    }
    #[doc = "Combined event from battery monitor"]
    #[inline(always)]
    pub fn batmon_comb(self) -> &'a mut W {
        self.variant(EV_A::BATMON_COMB)
    }
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    #[inline(always)]
    pub fn aon_gpio_edge(self) -> &'a mut W {
        self.variant(EV_A::AON_GPIO_EDGE)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<0> {
        EV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Selection for GPT2 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt2acaptsel](index.html) module"]
pub struct GPT2ACAPTSEL_SPEC;
impl crate::RegisterSpec for GPT2ACAPTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpt2acaptsel::R](R) reader structure"]
impl crate::Readable for GPT2ACAPTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpt2acaptsel::W](W) writer structure"]
impl crate::Writable for GPT2ACAPTSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPT2ACAPTSEL to value 0x59"]
impl crate::Resettable for GPT2ACAPTSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x59;
}
