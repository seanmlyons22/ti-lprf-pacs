#[doc = "Register `IOCFG19` reader"]
pub struct R(crate::R<IOCFG19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCFG19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCFG19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCFG19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCFG19` writer"]
pub struct W(crate::W<IOCFG19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCFG19_SPEC>;
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
impl From<crate::W<IOCFG19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCFG19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT_ID` reader - 5:0\\]
Selects usage for DIO19 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
pub type PORT_ID_R = crate::FieldReader<u8, PORT_ID_A>;
#[doc = "5:0\\]
Selects usage for DIO19 Note: This field should not be written other than the times when PORT_ID value is specifically required to change.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT_ID_A {
    #[doc = "56: RF Core SMI Command Link In"]
    RFC_SMI_CL_IN = 56,
    #[doc = "55: RF Core SMI Command Link Out"]
    RFC_SMI_CL_OUT = 55,
    #[doc = "54: RF Core SMI Data Link In"]
    RFC_SMI_DL_IN = 54,
    #[doc = "53: RF Core SMI Data Link Out"]
    RFC_SMI_DL_OUT = 53,
    #[doc = "52: RF Core Data In 1"]
    RFC_GPI1 = 52,
    #[doc = "51: RF Core Data In 0"]
    RFC_GPI0 = 51,
    #[doc = "50: RF Core Data Out 3"]
    RFC_GPO3 = 50,
    #[doc = "49: RF Core Data Out 2"]
    RFC_GPO2 = 49,
    #[doc = "48: RF Core Data Out 1"]
    RFC_GPO1 = 48,
    #[doc = "47: RF Core Data Out 0"]
    RFC_GPO0 = 47,
    #[doc = "46: RF Core Trace"]
    RFC_TRC = 46,
    #[doc = "41: I2S MCLK"]
    I2S_MCLK = 41,
    #[doc = "40: I2S BCLK"]
    I2S_BCLK = 40,
    #[doc = "39: I2S WCLK"]
    I2S_WCLK = 39,
    #[doc = "38: I2S Data 1"]
    I2S_AD1 = 38,
    #[doc = "37: I2S Data 0"]
    I2S_AD0 = 37,
    #[doc = "36: SSI1 CLK"]
    SSI1_CLK = 36,
    #[doc = "35: SSI1 FSS"]
    SSI1_FSS = 35,
    #[doc = "34: SSI1 TX"]
    SSI1_TX = 34,
    #[doc = "33: SSI1 RX"]
    SSI1_RX = 33,
    #[doc = "32: CPU SWV"]
    CPU_SWV = 32,
    #[doc = "30: PORT EVENT 7 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT7 = 30,
    #[doc = "29: PORT EVENT 6 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT6 = 29,
    #[doc = "28: PORT EVENT 5 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT5 = 28,
    #[doc = "27: PORT EVENT 4 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT4 = 27,
    #[doc = "26: PORT EVENT 3 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT3 = 26,
    #[doc = "25: PORT EVENT 2 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT2 = 25,
    #[doc = "24: PORT EVENT 1 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT1 = 24,
    #[doc = "23: PORT EVENT 0 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PORT_EVENT0 = 23,
    #[doc = "22: UART1 RTS"]
    UART1_RTS = 22,
    #[doc = "21: UART1 CTS"]
    UART1_CTS = 21,
    #[doc = "20: UART1 TX"]
    UART1_TX = 20,
    #[doc = "19: UART1 RX"]
    UART1_RX = 19,
    #[doc = "18: UART0 RTS"]
    UART0_RTS = 18,
    #[doc = "17: UART0 CTS"]
    UART0_CTS = 17,
    #[doc = "16: UART0 TX"]
    UART0_TX = 16,
    #[doc = "15: UART0 RX"]
    UART0_RX = 15,
    #[doc = "14: I2C Clock"]
    I2C_MSSCL = 14,
    #[doc = "13: I2C Data"]
    I2C_MSSDA = 13,
    #[doc = "12: SSI0 CLK"]
    SSI0_CLK = 12,
    #[doc = "11: SSI0 FSS"]
    SSI0_FSS = 11,
    #[doc = "10: SSI0 TX"]
    SSI0_TX = 10,
    #[doc = "9: SSI0 RX"]
    SSI0_RX = 9,
    #[doc = "8: AUX IO"]
    AUX_IO = 8,
    #[doc = "7: AON 32 KHz clock (SCLK_LF)"]
    AON_CLK32K = 7,
    #[doc = "0: General Purpose IO"]
    GPIO = 0,
}
impl From<PORT_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_ID_A) -> Self {
        variant as _
    }
}
impl PORT_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PORT_ID_A> {
        match self.bits {
            56 => Some(PORT_ID_A::RFC_SMI_CL_IN),
            55 => Some(PORT_ID_A::RFC_SMI_CL_OUT),
            54 => Some(PORT_ID_A::RFC_SMI_DL_IN),
            53 => Some(PORT_ID_A::RFC_SMI_DL_OUT),
            52 => Some(PORT_ID_A::RFC_GPI1),
            51 => Some(PORT_ID_A::RFC_GPI0),
            50 => Some(PORT_ID_A::RFC_GPO3),
            49 => Some(PORT_ID_A::RFC_GPO2),
            48 => Some(PORT_ID_A::RFC_GPO1),
            47 => Some(PORT_ID_A::RFC_GPO0),
            46 => Some(PORT_ID_A::RFC_TRC),
            41 => Some(PORT_ID_A::I2S_MCLK),
            40 => Some(PORT_ID_A::I2S_BCLK),
            39 => Some(PORT_ID_A::I2S_WCLK),
            38 => Some(PORT_ID_A::I2S_AD1),
            37 => Some(PORT_ID_A::I2S_AD0),
            36 => Some(PORT_ID_A::SSI1_CLK),
            35 => Some(PORT_ID_A::SSI1_FSS),
            34 => Some(PORT_ID_A::SSI1_TX),
            33 => Some(PORT_ID_A::SSI1_RX),
            32 => Some(PORT_ID_A::CPU_SWV),
            30 => Some(PORT_ID_A::PORT_EVENT7),
            29 => Some(PORT_ID_A::PORT_EVENT6),
            28 => Some(PORT_ID_A::PORT_EVENT5),
            27 => Some(PORT_ID_A::PORT_EVENT4),
            26 => Some(PORT_ID_A::PORT_EVENT3),
            25 => Some(PORT_ID_A::PORT_EVENT2),
            24 => Some(PORT_ID_A::PORT_EVENT1),
            23 => Some(PORT_ID_A::PORT_EVENT0),
            22 => Some(PORT_ID_A::UART1_RTS),
            21 => Some(PORT_ID_A::UART1_CTS),
            20 => Some(PORT_ID_A::UART1_TX),
            19 => Some(PORT_ID_A::UART1_RX),
            18 => Some(PORT_ID_A::UART0_RTS),
            17 => Some(PORT_ID_A::UART0_CTS),
            16 => Some(PORT_ID_A::UART0_TX),
            15 => Some(PORT_ID_A::UART0_RX),
            14 => Some(PORT_ID_A::I2C_MSSCL),
            13 => Some(PORT_ID_A::I2C_MSSDA),
            12 => Some(PORT_ID_A::SSI0_CLK),
            11 => Some(PORT_ID_A::SSI0_FSS),
            10 => Some(PORT_ID_A::SSI0_TX),
            9 => Some(PORT_ID_A::SSI0_RX),
            8 => Some(PORT_ID_A::AUX_IO),
            7 => Some(PORT_ID_A::AON_CLK32K),
            0 => Some(PORT_ID_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_CL_IN`"]
    #[inline(always)]
    pub fn is_rfc_smi_cl_in(&self) -> bool {
        *self == PORT_ID_A::RFC_SMI_CL_IN
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_CL_OUT`"]
    #[inline(always)]
    pub fn is_rfc_smi_cl_out(&self) -> bool {
        *self == PORT_ID_A::RFC_SMI_CL_OUT
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_DL_IN`"]
    #[inline(always)]
    pub fn is_rfc_smi_dl_in(&self) -> bool {
        *self == PORT_ID_A::RFC_SMI_DL_IN
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_DL_OUT`"]
    #[inline(always)]
    pub fn is_rfc_smi_dl_out(&self) -> bool {
        *self == PORT_ID_A::RFC_SMI_DL_OUT
    }
    #[doc = "Checks if the value of the field is `RFC_GPI1`"]
    #[inline(always)]
    pub fn is_rfc_gpi1(&self) -> bool {
        *self == PORT_ID_A::RFC_GPI1
    }
    #[doc = "Checks if the value of the field is `RFC_GPI0`"]
    #[inline(always)]
    pub fn is_rfc_gpi0(&self) -> bool {
        *self == PORT_ID_A::RFC_GPI0
    }
    #[doc = "Checks if the value of the field is `RFC_GPO3`"]
    #[inline(always)]
    pub fn is_rfc_gpo3(&self) -> bool {
        *self == PORT_ID_A::RFC_GPO3
    }
    #[doc = "Checks if the value of the field is `RFC_GPO2`"]
    #[inline(always)]
    pub fn is_rfc_gpo2(&self) -> bool {
        *self == PORT_ID_A::RFC_GPO2
    }
    #[doc = "Checks if the value of the field is `RFC_GPO1`"]
    #[inline(always)]
    pub fn is_rfc_gpo1(&self) -> bool {
        *self == PORT_ID_A::RFC_GPO1
    }
    #[doc = "Checks if the value of the field is `RFC_GPO0`"]
    #[inline(always)]
    pub fn is_rfc_gpo0(&self) -> bool {
        *self == PORT_ID_A::RFC_GPO0
    }
    #[doc = "Checks if the value of the field is `RFC_TRC`"]
    #[inline(always)]
    pub fn is_rfc_trc(&self) -> bool {
        *self == PORT_ID_A::RFC_TRC
    }
    #[doc = "Checks if the value of the field is `I2S_MCLK`"]
    #[inline(always)]
    pub fn is_i2s_mclk(&self) -> bool {
        *self == PORT_ID_A::I2S_MCLK
    }
    #[doc = "Checks if the value of the field is `I2S_BCLK`"]
    #[inline(always)]
    pub fn is_i2s_bclk(&self) -> bool {
        *self == PORT_ID_A::I2S_BCLK
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline(always)]
    pub fn is_i2s_wclk(&self) -> bool {
        *self == PORT_ID_A::I2S_WCLK
    }
    #[doc = "Checks if the value of the field is `I2S_AD1`"]
    #[inline(always)]
    pub fn is_i2s_ad1(&self) -> bool {
        *self == PORT_ID_A::I2S_AD1
    }
    #[doc = "Checks if the value of the field is `I2S_AD0`"]
    #[inline(always)]
    pub fn is_i2s_ad0(&self) -> bool {
        *self == PORT_ID_A::I2S_AD0
    }
    #[doc = "Checks if the value of the field is `SSI1_CLK`"]
    #[inline(always)]
    pub fn is_ssi1_clk(&self) -> bool {
        *self == PORT_ID_A::SSI1_CLK
    }
    #[doc = "Checks if the value of the field is `SSI1_FSS`"]
    #[inline(always)]
    pub fn is_ssi1_fss(&self) -> bool {
        *self == PORT_ID_A::SSI1_FSS
    }
    #[doc = "Checks if the value of the field is `SSI1_TX`"]
    #[inline(always)]
    pub fn is_ssi1_tx(&self) -> bool {
        *self == PORT_ID_A::SSI1_TX
    }
    #[doc = "Checks if the value of the field is `SSI1_RX`"]
    #[inline(always)]
    pub fn is_ssi1_rx(&self) -> bool {
        *self == PORT_ID_A::SSI1_RX
    }
    #[doc = "Checks if the value of the field is `CPU_SWV`"]
    #[inline(always)]
    pub fn is_cpu_swv(&self) -> bool {
        *self == PORT_ID_A::CPU_SWV
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT7`"]
    #[inline(always)]
    pub fn is_port_event7(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT7
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT6`"]
    #[inline(always)]
    pub fn is_port_event6(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT6
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT5`"]
    #[inline(always)]
    pub fn is_port_event5(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT5
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT4`"]
    #[inline(always)]
    pub fn is_port_event4(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT4
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT3`"]
    #[inline(always)]
    pub fn is_port_event3(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT3
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT2`"]
    #[inline(always)]
    pub fn is_port_event2(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT2
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT1`"]
    #[inline(always)]
    pub fn is_port_event1(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT1
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT0`"]
    #[inline(always)]
    pub fn is_port_event0(&self) -> bool {
        *self == PORT_ID_A::PORT_EVENT0
    }
    #[doc = "Checks if the value of the field is `UART1_RTS`"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        *self == PORT_ID_A::UART1_RTS
    }
    #[doc = "Checks if the value of the field is `UART1_CTS`"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        *self == PORT_ID_A::UART1_CTS
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PORT_ID_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PORT_ID_A::UART1_RX
    }
    #[doc = "Checks if the value of the field is `UART0_RTS`"]
    #[inline(always)]
    pub fn is_uart0_rts(&self) -> bool {
        *self == PORT_ID_A::UART0_RTS
    }
    #[doc = "Checks if the value of the field is `UART0_CTS`"]
    #[inline(always)]
    pub fn is_uart0_cts(&self) -> bool {
        *self == PORT_ID_A::UART0_CTS
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PORT_ID_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PORT_ID_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `I2C_MSSCL`"]
    #[inline(always)]
    pub fn is_i2c_msscl(&self) -> bool {
        *self == PORT_ID_A::I2C_MSSCL
    }
    #[doc = "Checks if the value of the field is `I2C_MSSDA`"]
    #[inline(always)]
    pub fn is_i2c_mssda(&self) -> bool {
        *self == PORT_ID_A::I2C_MSSDA
    }
    #[doc = "Checks if the value of the field is `SSI0_CLK`"]
    #[inline(always)]
    pub fn is_ssi0_clk(&self) -> bool {
        *self == PORT_ID_A::SSI0_CLK
    }
    #[doc = "Checks if the value of the field is `SSI0_FSS`"]
    #[inline(always)]
    pub fn is_ssi0_fss(&self) -> bool {
        *self == PORT_ID_A::SSI0_FSS
    }
    #[doc = "Checks if the value of the field is `SSI0_TX`"]
    #[inline(always)]
    pub fn is_ssi0_tx(&self) -> bool {
        *self == PORT_ID_A::SSI0_TX
    }
    #[doc = "Checks if the value of the field is `SSI0_RX`"]
    #[inline(always)]
    pub fn is_ssi0_rx(&self) -> bool {
        *self == PORT_ID_A::SSI0_RX
    }
    #[doc = "Checks if the value of the field is `AUX_IO`"]
    #[inline(always)]
    pub fn is_aux_io(&self) -> bool {
        *self == PORT_ID_A::AUX_IO
    }
    #[doc = "Checks if the value of the field is `AON_CLK32K`"]
    #[inline(always)]
    pub fn is_aon_clk32k(&self) -> bool {
        *self == PORT_ID_A::AON_CLK32K
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PORT_ID_A::GPIO
    }
}
#[doc = "Field `PORT_ID` writer - 5:0\\]
Selects usage for DIO19 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
pub type PORT_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IOCFG19_SPEC, u8, PORT_ID_A, 6, O>;
impl<'a, const O: u8> PORT_ID_W<'a, O> {
    #[doc = "RF Core SMI Command Link In"]
    #[inline(always)]
    pub fn rfc_smi_cl_in(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_SMI_CL_IN)
    }
    #[doc = "RF Core SMI Command Link Out"]
    #[inline(always)]
    pub fn rfc_smi_cl_out(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_SMI_CL_OUT)
    }
    #[doc = "RF Core SMI Data Link In"]
    #[inline(always)]
    pub fn rfc_smi_dl_in(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_SMI_DL_IN)
    }
    #[doc = "RF Core SMI Data Link Out"]
    #[inline(always)]
    pub fn rfc_smi_dl_out(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_SMI_DL_OUT)
    }
    #[doc = "RF Core Data In 1"]
    #[inline(always)]
    pub fn rfc_gpi1(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_GPI1)
    }
    #[doc = "RF Core Data In 0"]
    #[inline(always)]
    pub fn rfc_gpi0(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_GPI0)
    }
    #[doc = "RF Core Data Out 3"]
    #[inline(always)]
    pub fn rfc_gpo3(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_GPO3)
    }
    #[doc = "RF Core Data Out 2"]
    #[inline(always)]
    pub fn rfc_gpo2(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_GPO2)
    }
    #[doc = "RF Core Data Out 1"]
    #[inline(always)]
    pub fn rfc_gpo1(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_GPO1)
    }
    #[doc = "RF Core Data Out 0"]
    #[inline(always)]
    pub fn rfc_gpo0(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_GPO0)
    }
    #[doc = "RF Core Trace"]
    #[inline(always)]
    pub fn rfc_trc(self) -> &'a mut W {
        self.variant(PORT_ID_A::RFC_TRC)
    }
    #[doc = "I2S MCLK"]
    #[inline(always)]
    pub fn i2s_mclk(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2S_MCLK)
    }
    #[doc = "I2S BCLK"]
    #[inline(always)]
    pub fn i2s_bclk(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2S_BCLK)
    }
    #[doc = "I2S WCLK"]
    #[inline(always)]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2S_WCLK)
    }
    #[doc = "I2S Data 1"]
    #[inline(always)]
    pub fn i2s_ad1(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2S_AD1)
    }
    #[doc = "I2S Data 0"]
    #[inline(always)]
    pub fn i2s_ad0(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2S_AD0)
    }
    #[doc = "SSI1 CLK"]
    #[inline(always)]
    pub fn ssi1_clk(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI1_CLK)
    }
    #[doc = "SSI1 FSS"]
    #[inline(always)]
    pub fn ssi1_fss(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI1_FSS)
    }
    #[doc = "SSI1 TX"]
    #[inline(always)]
    pub fn ssi1_tx(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI1_TX)
    }
    #[doc = "SSI1 RX"]
    #[inline(always)]
    pub fn ssi1_rx(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI1_RX)
    }
    #[doc = "CPU SWV"]
    #[inline(always)]
    pub fn cpu_swv(self) -> &'a mut W {
        self.variant(PORT_ID_A::CPU_SWV)
    }
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event7(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT7)
    }
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event6(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT6)
    }
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event5(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT5)
    }
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event4(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT4)
    }
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event3(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT3)
    }
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event2(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT2)
    }
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event1(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT1)
    }
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event0(self) -> &'a mut W {
        self.variant(PORT_ID_A::PORT_EVENT0)
    }
    #[doc = "UART1 RTS"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART1_RTS)
    }
    #[doc = "UART1 CTS"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART1_CTS)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART1_TX)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART1_RX)
    }
    #[doc = "UART0 RTS"]
    #[inline(always)]
    pub fn uart0_rts(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART0_RTS)
    }
    #[doc = "UART0 CTS"]
    #[inline(always)]
    pub fn uart0_cts(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART0_CTS)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART0_TX)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PORT_ID_A::UART0_RX)
    }
    #[doc = "I2C Clock"]
    #[inline(always)]
    pub fn i2c_msscl(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2C_MSSCL)
    }
    #[doc = "I2C Data"]
    #[inline(always)]
    pub fn i2c_mssda(self) -> &'a mut W {
        self.variant(PORT_ID_A::I2C_MSSDA)
    }
    #[doc = "SSI0 CLK"]
    #[inline(always)]
    pub fn ssi0_clk(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI0_CLK)
    }
    #[doc = "SSI0 FSS"]
    #[inline(always)]
    pub fn ssi0_fss(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI0_FSS)
    }
    #[doc = "SSI0 TX"]
    #[inline(always)]
    pub fn ssi0_tx(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI0_TX)
    }
    #[doc = "SSI0 RX"]
    #[inline(always)]
    pub fn ssi0_rx(self) -> &'a mut W {
        self.variant(PORT_ID_A::SSI0_RX)
    }
    #[doc = "AUX IO"]
    #[inline(always)]
    pub fn aux_io(self) -> &'a mut W {
        self.variant(PORT_ID_A::AUX_IO)
    }
    #[doc = "AON 32 KHz clock (SCLK_LF)"]
    #[inline(always)]
    pub fn aon_clk32k(self) -> &'a mut W {
        self.variant(PORT_ID_A::AON_CLK32K)
    }
    #[doc = "General Purpose IO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PORT_ID_A::GPIO)
    }
}
#[doc = "Field `IOEV_MCU_WU_EN` reader - 6:6\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
pub type IOEV_MCU_WU_EN_R = crate::BitReader<bool>;
#[doc = "Field `IOEV_MCU_WU_EN` writer - 6:6\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
pub type IOEV_MCU_WU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `IOEV_RTC_EN` reader - 7:7\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
pub type IOEV_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `IOEV_RTC_EN` writer - 7:7\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
pub type IOEV_RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `IOSTR` reader - 9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
pub type IOSTR_R = crate::FieldReader<u8, IOSTR_A>;
#[doc = "9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOSTR_A {
    #[doc = "3: Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    MAX = 3,
    #[doc = "2: Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    MED = 2,
    #[doc = "1: Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    MIN = 1,
    #[doc = "0: Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    AUTO = 0,
}
impl From<IOSTR_A> for u8 {
    #[inline(always)]
    fn from(variant: IOSTR_A) -> Self {
        variant as _
    }
}
impl IOSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOSTR_A {
        match self.bits {
            3 => IOSTR_A::MAX,
            2 => IOSTR_A::MED,
            1 => IOSTR_A::MIN,
            0 => IOSTR_A::AUTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == IOSTR_A::MAX
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == IOSTR_A::MED
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == IOSTR_A::MIN
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == IOSTR_A::AUTO
    }
}
#[doc = "Field `IOSTR` writer - 9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
pub type IOSTR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IOCFG19_SPEC, u8, IOSTR_A, 2, O>;
impl<'a, const O: u8> IOSTR_W<'a, O> {
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(IOSTR_A::MAX)
    }
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(IOSTR_A::MED)
    }
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(IOSTR_A::MIN)
    }
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(IOSTR_A::AUTO)
    }
}
#[doc = "Field `IOCURR` reader - 11:10\\]
Selects IO current mode of this IO."]
pub type IOCURR_R = crate::FieldReader<u8, IOCURR_A>;
#[doc = "11:10\\]
Selects IO current mode of this IO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOCURR_A {
    #[doc = "2: Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    _4_8MA = 2,
    #[doc = "1: High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    _4MA = 1,
    #[doc = "0: Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    _2MA = 0,
}
impl From<IOCURR_A> for u8 {
    #[inline(always)]
    fn from(variant: IOCURR_A) -> Self {
        variant as _
    }
}
impl IOCURR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IOCURR_A> {
        match self.bits {
            2 => Some(IOCURR_A::_4_8MA),
            1 => Some(IOCURR_A::_4MA),
            0 => Some(IOCURR_A::_2MA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4_8MA`"]
    #[inline(always)]
    pub fn is_4_8ma(&self) -> bool {
        *self == IOCURR_A::_4_8MA
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == IOCURR_A::_4MA
    }
    #[doc = "Checks if the value of the field is `_2MA`"]
    #[inline(always)]
    pub fn is_2ma(&self) -> bool {
        *self == IOCURR_A::_2MA
    }
}
#[doc = "Field `IOCURR` writer - 11:10\\]
Selects IO current mode of this IO."]
pub type IOCURR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCFG19_SPEC, u8, IOCURR_A, 2, O>;
impl<'a, const O: u8> IOCURR_W<'a, O> {
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn _4_8ma(self) -> &'a mut W {
        self.variant(IOCURR_A::_4_8MA)
    }
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn _4ma(self) -> &'a mut W {
        self.variant(IOCURR_A::_4MA)
    }
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn _2ma(self) -> &'a mut W {
        self.variant(IOCURR_A::_2MA)
    }
}
#[doc = "Field `SLEW_RED` reader - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
pub type SLEW_RED_R = crate::BitReader<bool>;
#[doc = "Field `SLEW_RED` writer - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
pub type SLEW_RED_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `PULL_CTL` reader - 14:13\\]
Pull control"]
pub type PULL_CTL_R = crate::FieldReader<u8, PULL_CTL_A>;
#[doc = "14:13\\]
Pull control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PULL_CTL_A {
    #[doc = "3: No pull"]
    DIS = 3,
    #[doc = "2: Pull up"]
    UP = 2,
    #[doc = "1: Pull down"]
    DWN = 1,
}
impl From<PULL_CTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PULL_CTL_A) -> Self {
        variant as _
    }
}
impl PULL_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PULL_CTL_A> {
        match self.bits {
            3 => Some(PULL_CTL_A::DIS),
            2 => Some(PULL_CTL_A::UP),
            1 => Some(PULL_CTL_A::DWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PULL_CTL_A::DIS
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == PULL_CTL_A::UP
    }
    #[doc = "Checks if the value of the field is `DWN`"]
    #[inline(always)]
    pub fn is_dwn(&self) -> bool {
        *self == PULL_CTL_A::DWN
    }
}
#[doc = "Field `PULL_CTL` writer - 14:13\\]
Pull control"]
pub type PULL_CTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IOCFG19_SPEC, u8, PULL_CTL_A, 2, O>;
impl<'a, const O: u8> PULL_CTL_W<'a, O> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PULL_CTL_A::DIS)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(PULL_CTL_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn dwn(self) -> &'a mut W {
        self.variant(PULL_CTL_A::DWN)
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `EDGE_DET` reader - 17:16\\]
Enable generation of edge detection events on this IO"]
pub type EDGE_DET_R = crate::FieldReader<u8, EDGE_DET_A>;
#[doc = "17:16\\]
Enable generation of edge detection events on this IO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE_DET_A {
    #[doc = "3: Positive and negative edge detection"]
    BOTH = 3,
    #[doc = "2: Positive edge detection"]
    POS = 2,
    #[doc = "1: Negative edge detection"]
    NEG = 1,
    #[doc = "0: No edge detection"]
    NONE = 0,
}
impl From<EDGE_DET_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE_DET_A) -> Self {
        variant as _
    }
}
impl EDGE_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_DET_A {
        match self.bits {
            3 => EDGE_DET_A::BOTH,
            2 => EDGE_DET_A::POS,
            1 => EDGE_DET_A::NEG,
            0 => EDGE_DET_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EDGE_DET_A::BOTH
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == EDGE_DET_A::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == EDGE_DET_A::NEG
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EDGE_DET_A::NONE
    }
}
#[doc = "Field `EDGE_DET` writer - 17:16\\]
Enable generation of edge detection events on this IO"]
pub type EDGE_DET_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IOCFG19_SPEC, u8, EDGE_DET_A, 2, O>;
impl<'a, const O: u8> EDGE_DET_W<'a, O> {
    #[doc = "Positive and negative edge detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EDGE_DET_A::BOTH)
    }
    #[doc = "Positive edge detection"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(EDGE_DET_A::POS)
    }
    #[doc = "Negative edge detection"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(EDGE_DET_A::NEG)
    }
    #[doc = "No edge detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EDGE_DET_A::NONE)
    }
}
#[doc = "Field `EDGE_IRQ_EN` reader - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
pub type EDGE_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `EDGE_IRQ_EN` writer - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
pub type EDGE_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `RESERVED19` reader - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED19` writer - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCFG19_SPEC, u8, u8, 2, O>;
#[doc = "Field `IOEV_AON_PROG0_EN` reader - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
pub type IOEV_AON_PROG0_EN_R = crate::BitReader<bool>;
#[doc = "Field `IOEV_AON_PROG0_EN` writer - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
pub type IOEV_AON_PROG0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `IOEV_AON_PROG1_EN` reader - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
pub type IOEV_AON_PROG1_EN_R = crate::BitReader<bool>;
#[doc = "Field `IOEV_AON_PROG1_EN` writer - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
pub type IOEV_AON_PROG1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `IOEV_AON_PROG2_EN` reader - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
pub type IOEV_AON_PROG2_EN_R = crate::BitReader<bool>;
#[doc = "Field `IOEV_AON_PROG2_EN` writer - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
pub type IOEV_AON_PROG2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `IOMODE` reader - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
pub type IOMODE_R = crate::FieldReader<u8, IOMODE_A>;
#[doc = "26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOMODE_A {
    #[doc = "7: Open Source Inverted input / output"]
    OPENSRC_INV = 7,
    #[doc = "6: Open Source Normal input / output"]
    OPENSRC = 6,
    #[doc = "5: Open Drain Inverted input / output"]
    OPENDR_INV = 5,
    #[doc = "4: Open Drain, Normal input / output"]
    OPENDR = 4,
    #[doc = "1: Inverted input / ouput"]
    INV = 1,
    #[doc = "0: Normal input / output"]
    NORMAL = 0,
}
impl From<IOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IOMODE_A) -> Self {
        variant as _
    }
}
impl IOMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IOMODE_A> {
        match self.bits {
            7 => Some(IOMODE_A::OPENSRC_INV),
            6 => Some(IOMODE_A::OPENSRC),
            5 => Some(IOMODE_A::OPENDR_INV),
            4 => Some(IOMODE_A::OPENDR),
            1 => Some(IOMODE_A::INV),
            0 => Some(IOMODE_A::NORMAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OPENSRC_INV`"]
    #[inline(always)]
    pub fn is_opensrc_inv(&self) -> bool {
        *self == IOMODE_A::OPENSRC_INV
    }
    #[doc = "Checks if the value of the field is `OPENSRC`"]
    #[inline(always)]
    pub fn is_opensrc(&self) -> bool {
        *self == IOMODE_A::OPENSRC
    }
    #[doc = "Checks if the value of the field is `OPENDR_INV`"]
    #[inline(always)]
    pub fn is_opendr_inv(&self) -> bool {
        *self == IOMODE_A::OPENDR_INV
    }
    #[doc = "Checks if the value of the field is `OPENDR`"]
    #[inline(always)]
    pub fn is_opendr(&self) -> bool {
        *self == IOMODE_A::OPENDR
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == IOMODE_A::INV
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IOMODE_A::NORMAL
    }
}
#[doc = "Field `IOMODE` writer - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
pub type IOMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCFG19_SPEC, u8, IOMODE_A, 3, O>;
impl<'a, const O: u8> IOMODE_W<'a, O> {
    #[doc = "Open Source Inverted input / output"]
    #[inline(always)]
    pub fn opensrc_inv(self) -> &'a mut W {
        self.variant(IOMODE_A::OPENSRC_INV)
    }
    #[doc = "Open Source Normal input / output"]
    #[inline(always)]
    pub fn opensrc(self) -> &'a mut W {
        self.variant(IOMODE_A::OPENSRC)
    }
    #[doc = "Open Drain Inverted input / output"]
    #[inline(always)]
    pub fn opendr_inv(self) -> &'a mut W {
        self.variant(IOMODE_A::OPENDR_INV)
    }
    #[doc = "Open Drain, Normal input / output"]
    #[inline(always)]
    pub fn opendr(self) -> &'a mut W {
        self.variant(IOMODE_A::OPENDR)
    }
    #[doc = "Inverted input / ouput"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(IOMODE_A::INV)
    }
    #[doc = "Normal input / output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IOMODE_A::NORMAL)
    }
}
#[doc = "Field `WU_CFG` reader - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
pub type WU_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WU_CFG` writer - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
pub type WU_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCFG19_SPEC, u8, u8, 2, O>;
#[doc = "Field `IE` reader - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `HYST_EN` reader - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
pub type HYST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HYST_EN` writer - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
pub type HYST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED31_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED31_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG19_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects usage for DIO19 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
    #[inline(always)]
    pub fn port_id(&self) -> PORT_ID_R {
        PORT_ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
    #[inline(always)]
    pub fn ioev_mcu_wu_en(&self) -> IOEV_MCU_WU_EN_R {
        IOEV_MCU_WU_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
    #[inline(always)]
    pub fn ioev_rtc_en(&self) -> IOEV_RTC_EN_R {
        IOEV_RTC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline(always)]
    pub fn iostr(&self) -> IOSTR_R {
        IOSTR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects IO current mode of this IO."]
    #[inline(always)]
    pub fn iocurr(&self) -> IOCURR_R {
        IOCURR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub fn slew_red(&self) -> SLEW_RED_R {
        SLEW_RED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Pull control"]
    #[inline(always)]
    pub fn pull_ctl(&self) -> PULL_CTL_R {
        PULL_CTL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable generation of edge detection events on this IO"]
    #[inline(always)]
    pub fn edge_det(&self) -> EDGE_DET_R {
        EDGE_DET_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline(always)]
    pub fn edge_irq_en(&self) -> EDGE_IRQ_EN_R {
        EDGE_IRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
    #[inline(always)]
    pub fn ioev_aon_prog0_en(&self) -> IOEV_AON_PROG0_EN_R {
        IOEV_AON_PROG0_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
    #[inline(always)]
    pub fn ioev_aon_prog1_en(&self) -> IOEV_AON_PROG1_EN_R {
        IOEV_AON_PROG1_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
    #[inline(always)]
    pub fn ioev_aon_prog2_en(&self) -> IOEV_AON_PROG2_EN_R {
        IOEV_AON_PROG2_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub fn iomode(&self) -> IOMODE_R {
        IOMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub fn wu_cfg(&self) -> WU_CFG_R {
        WU_CFG_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects usage for DIO19 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
    #[inline(always)]
    #[must_use]
    pub fn port_id(&mut self) -> PORT_ID_W<0> {
        PORT_ID_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_mcu_wu_en(&mut self) -> IOEV_MCU_WU_EN_W<6> {
        IOEV_MCU_WU_EN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_rtc_en(&mut self) -> IOEV_RTC_EN_W<7> {
        IOEV_RTC_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline(always)]
    #[must_use]
    pub fn iostr(&mut self) -> IOSTR_W<8> {
        IOSTR_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects IO current mode of this IO."]
    #[inline(always)]
    #[must_use]
    pub fn iocurr(&mut self) -> IOCURR_W<10> {
        IOCURR_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    #[must_use]
    pub fn slew_red(&mut self) -> SLEW_RED_W<12> {
        SLEW_RED_W::new(self)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Pull control"]
    #[inline(always)]
    #[must_use]
    pub fn pull_ctl(&mut self) -> PULL_CTL_W<13> {
        PULL_CTL_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable generation of edge detection events on this IO"]
    #[inline(always)]
    #[must_use]
    pub fn edge_det(&mut self) -> EDGE_DET_W<16> {
        EDGE_DET_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn edge_irq_en(&mut self) -> EDGE_IRQ_EN_W<18> {
        EDGE_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<19> {
        RESERVED19_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_aon_prog0_en(&mut self) -> IOEV_AON_PROG0_EN_W<21> {
        IOEV_AON_PROG0_EN_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_aon_prog1_en(&mut self) -> IOEV_AON_PROG1_EN_W<22> {
        IOEV_AON_PROG1_EN_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_aon_prog2_en(&mut self) -> IOEV_AON_PROG2_EN_W<23> {
        IOEV_AON_PROG2_EN_W::new(self)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn iomode(&mut self) -> IOMODE_W<24> {
        IOMODE_W::new(self)
    }
    #[doc = "Bits 27:28 - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    #[must_use]
    pub fn wu_cfg(&mut self) -> WU_CFG_W<27> {
        WU_CFG_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<29> {
        IE_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HYST_EN_W<30> {
        HYST_EN_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> RESERVED31_W<31> {
        RESERVED31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of DIO19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg19](index.html) module"]
pub struct IOCFG19_SPEC;
impl crate::RegisterSpec for IOCFG19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocfg19::R](R) reader structure"]
impl crate::Readable for IOCFG19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocfg19::W](W) writer structure"]
impl crate::Writable for IOCFG19_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCFG19 to value 0x6000"]
impl crate::Resettable for IOCFG19_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000;
}
