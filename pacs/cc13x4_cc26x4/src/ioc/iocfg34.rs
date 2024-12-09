#[doc = "Register `IOCFG34` reader"]
pub type R = crate::R<Iocfg34Spec>;
#[doc = "Register `IOCFG34` writer"]
pub type W = crate::W<Iocfg34Spec>;
#[doc = "6:0\\]
Selects usage for DIO34 Note: This field should not be written other than the times when PORT_ID value is specifically required to change.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PortId {
    #[doc = "74: I2C1 Clock"]
    I2c1Msscl = 74,
    #[doc = "73: I2C1 Data"]
    I2c1Mssda = 73,
    #[doc = "72: UART3 RTS"]
    Uart3Rts = 72,
    #[doc = "71: UART3 CTS"]
    Uart3Cts = 71,
    #[doc = "70: UART3 TX"]
    Uart3Tx = 70,
    #[doc = "69: UART3 RX"]
    Uart3Rx = 69,
    #[doc = "68: UART2 RTS"]
    Uart2Rts = 68,
    #[doc = "67: UART2 CTS"]
    Uart2Cts = 67,
    #[doc = "66: UART2 TX"]
    Uart2Tx = 66,
    #[doc = "65: UART2 RX"]
    Uart2Rx = 65,
    #[doc = "64: SPI3 CLK"]
    Spi3Clk = 64,
    #[doc = "63: SPI3 CS"]
    Spi3Cs = 63,
    #[doc = "62: SPI3 TX"]
    Spi3Tx = 62,
    #[doc = "61: SPI3 RX"]
    Spi3Rx = 61,
    #[doc = "60: SPI2 CLK"]
    Spi2Clk = 60,
    #[doc = "59: SPI2 CS"]
    Spi2Cs = 59,
    #[doc = "58: SPI2 TX"]
    Spi2Tx = 58,
    #[doc = "57: SPI2 RX"]
    Spi2Rx = 57,
    #[doc = "56: RF Core SMI Command Link In"]
    RfcSmiClIn = 56,
    #[doc = "55: RF Core SMI Command Link Out"]
    RfcSmiClOut = 55,
    #[doc = "54: RF Core SMI Data Link In"]
    RfcSmiDlIn = 54,
    #[doc = "53: RF Core SMI Data Link Out"]
    RfcSmiDlOut = 53,
    #[doc = "52: RF Core Data In 1"]
    RfcGpi1 = 52,
    #[doc = "51: RF Core Data In 0"]
    RfcGpi0 = 51,
    #[doc = "50: RF Core Data Out 3"]
    RfcGpo3 = 50,
    #[doc = "49: RF Core Data Out 2"]
    RfcGpo2 = 49,
    #[doc = "48: RF Core Data Out 1"]
    RfcGpo1 = 48,
    #[doc = "47: RF Core Data Out 0"]
    RfcGpo0 = 47,
    #[doc = "46: RF Core Trace"]
    RfcTrc = 46,
    #[doc = "41: I2S MCLK"]
    I2sMclk = 41,
    #[doc = "40: I2S BCLK"]
    I2sBclk = 40,
    #[doc = "39: I2S WCLK"]
    I2sWclk = 39,
    #[doc = "38: I2S Data 1"]
    I2sAd1 = 38,
    #[doc = "37: I2S Data 0"]
    I2sAd0 = 37,
    #[doc = "36: SPI1 CLK"]
    Spi1Clk = 36,
    #[doc = "35: SPI1 CS"]
    Spi1Cs = 35,
    #[doc = "34: SPI1 TX"]
    Spi1Tx = 34,
    #[doc = "33: SPI1 RX"]
    Spi1Rx = 33,
    #[doc = "32: CPU SWV"]
    CpuSwv = 32,
    #[doc = "30: PORT EVENT 7 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent7 = 30,
    #[doc = "29: PORT EVENT 6 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent6 = 29,
    #[doc = "28: PORT EVENT 5 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent5 = 28,
    #[doc = "27: PORT EVENT 4 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent4 = 27,
    #[doc = "26: PORT EVENT 3 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent3 = 26,
    #[doc = "25: PORT EVENT 2 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent2 = 25,
    #[doc = "24: PORT EVENT 1 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent1 = 24,
    #[doc = "23: PORT EVENT 0 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    PortEvent0 = 23,
    #[doc = "22: UART1 RTS"]
    Uart1Rts = 22,
    #[doc = "21: UART1 CTS"]
    Uart1Cts = 21,
    #[doc = "20: UART1 TX"]
    Uart1Tx = 20,
    #[doc = "19: UART1 RX"]
    Uart1Rx = 19,
    #[doc = "18: UART0 RTS"]
    Uart0Rts = 18,
    #[doc = "17: UART0 CTS"]
    Uart0Cts = 17,
    #[doc = "16: UART0 TX"]
    Uart0Tx = 16,
    #[doc = "15: UART0 RX"]
    Uart0Rx = 15,
    #[doc = "14: I2C0 Clock"]
    I2c0Msscl = 14,
    #[doc = "13: I2C0 Data"]
    I2c0Mssda = 13,
    #[doc = "12: SPI0 CLK"]
    Spi0Clk = 12,
    #[doc = "11: SPI0 CS"]
    Spi0Cs = 11,
    #[doc = "10: SPI0 TX"]
    Spi0Tx = 10,
    #[doc = "9: SPI0 RX"]
    Spi0Rx = 9,
    #[doc = "7: AON 32 KHz clock (SCLK_LF)"]
    AonClk32k = 7,
    #[doc = "0: General Purpose IO"]
    Gpio = 0,
}
impl From<PortId> for u8 {
    #[inline(always)]
    fn from(variant: PortId) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PortId {
    type Ux = u8;
}
impl crate::IsEnum for PortId {}
#[doc = "Field `PORT_ID` reader - 6:0\\]
Selects usage for DIO34 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
pub type PortIdR = crate::FieldReader<PortId>;
impl PortIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PortId> {
        match self.bits {
            74 => Some(PortId::I2c1Msscl),
            73 => Some(PortId::I2c1Mssda),
            72 => Some(PortId::Uart3Rts),
            71 => Some(PortId::Uart3Cts),
            70 => Some(PortId::Uart3Tx),
            69 => Some(PortId::Uart3Rx),
            68 => Some(PortId::Uart2Rts),
            67 => Some(PortId::Uart2Cts),
            66 => Some(PortId::Uart2Tx),
            65 => Some(PortId::Uart2Rx),
            64 => Some(PortId::Spi3Clk),
            63 => Some(PortId::Spi3Cs),
            62 => Some(PortId::Spi3Tx),
            61 => Some(PortId::Spi3Rx),
            60 => Some(PortId::Spi2Clk),
            59 => Some(PortId::Spi2Cs),
            58 => Some(PortId::Spi2Tx),
            57 => Some(PortId::Spi2Rx),
            56 => Some(PortId::RfcSmiClIn),
            55 => Some(PortId::RfcSmiClOut),
            54 => Some(PortId::RfcSmiDlIn),
            53 => Some(PortId::RfcSmiDlOut),
            52 => Some(PortId::RfcGpi1),
            51 => Some(PortId::RfcGpi0),
            50 => Some(PortId::RfcGpo3),
            49 => Some(PortId::RfcGpo2),
            48 => Some(PortId::RfcGpo1),
            47 => Some(PortId::RfcGpo0),
            46 => Some(PortId::RfcTrc),
            41 => Some(PortId::I2sMclk),
            40 => Some(PortId::I2sBclk),
            39 => Some(PortId::I2sWclk),
            38 => Some(PortId::I2sAd1),
            37 => Some(PortId::I2sAd0),
            36 => Some(PortId::Spi1Clk),
            35 => Some(PortId::Spi1Cs),
            34 => Some(PortId::Spi1Tx),
            33 => Some(PortId::Spi1Rx),
            32 => Some(PortId::CpuSwv),
            30 => Some(PortId::PortEvent7),
            29 => Some(PortId::PortEvent6),
            28 => Some(PortId::PortEvent5),
            27 => Some(PortId::PortEvent4),
            26 => Some(PortId::PortEvent3),
            25 => Some(PortId::PortEvent2),
            24 => Some(PortId::PortEvent1),
            23 => Some(PortId::PortEvent0),
            22 => Some(PortId::Uart1Rts),
            21 => Some(PortId::Uart1Cts),
            20 => Some(PortId::Uart1Tx),
            19 => Some(PortId::Uart1Rx),
            18 => Some(PortId::Uart0Rts),
            17 => Some(PortId::Uart0Cts),
            16 => Some(PortId::Uart0Tx),
            15 => Some(PortId::Uart0Rx),
            14 => Some(PortId::I2c0Msscl),
            13 => Some(PortId::I2c0Mssda),
            12 => Some(PortId::Spi0Clk),
            11 => Some(PortId::Spi0Cs),
            10 => Some(PortId::Spi0Tx),
            9 => Some(PortId::Spi0Rx),
            7 => Some(PortId::AonClk32k),
            0 => Some(PortId::Gpio),
            _ => None,
        }
    }
    #[doc = "I2C1 Clock"]
    #[inline(always)]
    pub fn is_i2c1_msscl(&self) -> bool {
        *self == PortId::I2c1Msscl
    }
    #[doc = "I2C1 Data"]
    #[inline(always)]
    pub fn is_i2c1_mssda(&self) -> bool {
        *self == PortId::I2c1Mssda
    }
    #[doc = "UART3 RTS"]
    #[inline(always)]
    pub fn is_uart3_rts(&self) -> bool {
        *self == PortId::Uart3Rts
    }
    #[doc = "UART3 CTS"]
    #[inline(always)]
    pub fn is_uart3_cts(&self) -> bool {
        *self == PortId::Uart3Cts
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PortId::Uart3Tx
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PortId::Uart3Rx
    }
    #[doc = "UART2 RTS"]
    #[inline(always)]
    pub fn is_uart2_rts(&self) -> bool {
        *self == PortId::Uart2Rts
    }
    #[doc = "UART2 CTS"]
    #[inline(always)]
    pub fn is_uart2_cts(&self) -> bool {
        *self == PortId::Uart2Cts
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PortId::Uart2Tx
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PortId::Uart2Rx
    }
    #[doc = "SPI3 CLK"]
    #[inline(always)]
    pub fn is_spi3_clk(&self) -> bool {
        *self == PortId::Spi3Clk
    }
    #[doc = "SPI3 CS"]
    #[inline(always)]
    pub fn is_spi3_cs(&self) -> bool {
        *self == PortId::Spi3Cs
    }
    #[doc = "SPI3 TX"]
    #[inline(always)]
    pub fn is_spi3_tx(&self) -> bool {
        *self == PortId::Spi3Tx
    }
    #[doc = "SPI3 RX"]
    #[inline(always)]
    pub fn is_spi3_rx(&self) -> bool {
        *self == PortId::Spi3Rx
    }
    #[doc = "SPI2 CLK"]
    #[inline(always)]
    pub fn is_spi2_clk(&self) -> bool {
        *self == PortId::Spi2Clk
    }
    #[doc = "SPI2 CS"]
    #[inline(always)]
    pub fn is_spi2_cs(&self) -> bool {
        *self == PortId::Spi2Cs
    }
    #[doc = "SPI2 TX"]
    #[inline(always)]
    pub fn is_spi2_tx(&self) -> bool {
        *self == PortId::Spi2Tx
    }
    #[doc = "SPI2 RX"]
    #[inline(always)]
    pub fn is_spi2_rx(&self) -> bool {
        *self == PortId::Spi2Rx
    }
    #[doc = "RF Core SMI Command Link In"]
    #[inline(always)]
    pub fn is_rfc_smi_cl_in(&self) -> bool {
        *self == PortId::RfcSmiClIn
    }
    #[doc = "RF Core SMI Command Link Out"]
    #[inline(always)]
    pub fn is_rfc_smi_cl_out(&self) -> bool {
        *self == PortId::RfcSmiClOut
    }
    #[doc = "RF Core SMI Data Link In"]
    #[inline(always)]
    pub fn is_rfc_smi_dl_in(&self) -> bool {
        *self == PortId::RfcSmiDlIn
    }
    #[doc = "RF Core SMI Data Link Out"]
    #[inline(always)]
    pub fn is_rfc_smi_dl_out(&self) -> bool {
        *self == PortId::RfcSmiDlOut
    }
    #[doc = "RF Core Data In 1"]
    #[inline(always)]
    pub fn is_rfc_gpi1(&self) -> bool {
        *self == PortId::RfcGpi1
    }
    #[doc = "RF Core Data In 0"]
    #[inline(always)]
    pub fn is_rfc_gpi0(&self) -> bool {
        *self == PortId::RfcGpi0
    }
    #[doc = "RF Core Data Out 3"]
    #[inline(always)]
    pub fn is_rfc_gpo3(&self) -> bool {
        *self == PortId::RfcGpo3
    }
    #[doc = "RF Core Data Out 2"]
    #[inline(always)]
    pub fn is_rfc_gpo2(&self) -> bool {
        *self == PortId::RfcGpo2
    }
    #[doc = "RF Core Data Out 1"]
    #[inline(always)]
    pub fn is_rfc_gpo1(&self) -> bool {
        *self == PortId::RfcGpo1
    }
    #[doc = "RF Core Data Out 0"]
    #[inline(always)]
    pub fn is_rfc_gpo0(&self) -> bool {
        *self == PortId::RfcGpo0
    }
    #[doc = "RF Core Trace"]
    #[inline(always)]
    pub fn is_rfc_trc(&self) -> bool {
        *self == PortId::RfcTrc
    }
    #[doc = "I2S MCLK"]
    #[inline(always)]
    pub fn is_i2s_mclk(&self) -> bool {
        *self == PortId::I2sMclk
    }
    #[doc = "I2S BCLK"]
    #[inline(always)]
    pub fn is_i2s_bclk(&self) -> bool {
        *self == PortId::I2sBclk
    }
    #[doc = "I2S WCLK"]
    #[inline(always)]
    pub fn is_i2s_wclk(&self) -> bool {
        *self == PortId::I2sWclk
    }
    #[doc = "I2S Data 1"]
    #[inline(always)]
    pub fn is_i2s_ad1(&self) -> bool {
        *self == PortId::I2sAd1
    }
    #[doc = "I2S Data 0"]
    #[inline(always)]
    pub fn is_i2s_ad0(&self) -> bool {
        *self == PortId::I2sAd0
    }
    #[doc = "SPI1 CLK"]
    #[inline(always)]
    pub fn is_spi1_clk(&self) -> bool {
        *self == PortId::Spi1Clk
    }
    #[doc = "SPI1 CS"]
    #[inline(always)]
    pub fn is_spi1_cs(&self) -> bool {
        *self == PortId::Spi1Cs
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == PortId::Spi1Tx
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == PortId::Spi1Rx
    }
    #[doc = "CPU SWV"]
    #[inline(always)]
    pub fn is_cpu_swv(&self) -> bool {
        *self == PortId::CpuSwv
    }
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event7(&self) -> bool {
        *self == PortId::PortEvent7
    }
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event6(&self) -> bool {
        *self == PortId::PortEvent6
    }
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event5(&self) -> bool {
        *self == PortId::PortEvent5
    }
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event4(&self) -> bool {
        *self == PortId::PortEvent4
    }
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event3(&self) -> bool {
        *self == PortId::PortEvent3
    }
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event2(&self) -> bool {
        *self == PortId::PortEvent2
    }
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event1(&self) -> bool {
        *self == PortId::PortEvent1
    }
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn is_port_event0(&self) -> bool {
        *self == PortId::PortEvent0
    }
    #[doc = "UART1 RTS"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        *self == PortId::Uart1Rts
    }
    #[doc = "UART1 CTS"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        *self == PortId::Uart1Cts
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PortId::Uart1Tx
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PortId::Uart1Rx
    }
    #[doc = "UART0 RTS"]
    #[inline(always)]
    pub fn is_uart0_rts(&self) -> bool {
        *self == PortId::Uart0Rts
    }
    #[doc = "UART0 CTS"]
    #[inline(always)]
    pub fn is_uart0_cts(&self) -> bool {
        *self == PortId::Uart0Cts
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PortId::Uart0Tx
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PortId::Uart0Rx
    }
    #[doc = "I2C0 Clock"]
    #[inline(always)]
    pub fn is_i2c0_msscl(&self) -> bool {
        *self == PortId::I2c0Msscl
    }
    #[doc = "I2C0 Data"]
    #[inline(always)]
    pub fn is_i2c0_mssda(&self) -> bool {
        *self == PortId::I2c0Mssda
    }
    #[doc = "SPI0 CLK"]
    #[inline(always)]
    pub fn is_spi0_clk(&self) -> bool {
        *self == PortId::Spi0Clk
    }
    #[doc = "SPI0 CS"]
    #[inline(always)]
    pub fn is_spi0_cs(&self) -> bool {
        *self == PortId::Spi0Cs
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        *self == PortId::Spi0Tx
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        *self == PortId::Spi0Rx
    }
    #[doc = "AON 32 KHz clock (SCLK_LF)"]
    #[inline(always)]
    pub fn is_aon_clk32k(&self) -> bool {
        *self == PortId::AonClk32k
    }
    #[doc = "General Purpose IO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PortId::Gpio
    }
}
#[doc = "Field `PORT_ID` writer - 6:0\\]
Selects usage for DIO34 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
pub type PortIdW<'a, REG> = crate::FieldWriter<'a, REG, 7, PortId>;
impl<'a, REG> PortIdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2C1 Clock"]
    #[inline(always)]
    pub fn i2c1_msscl(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2c1Msscl)
    }
    #[doc = "I2C1 Data"]
    #[inline(always)]
    pub fn i2c1_mssda(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2c1Mssda)
    }
    #[doc = "UART3 RTS"]
    #[inline(always)]
    pub fn uart3_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart3Rts)
    }
    #[doc = "UART3 CTS"]
    #[inline(always)]
    pub fn uart3_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart3Cts)
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart3Tx)
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart3Rx)
    }
    #[doc = "UART2 RTS"]
    #[inline(always)]
    pub fn uart2_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart2Rts)
    }
    #[doc = "UART2 CTS"]
    #[inline(always)]
    pub fn uart2_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart2Cts)
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart2Tx)
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart2Rx)
    }
    #[doc = "SPI3 CLK"]
    #[inline(always)]
    pub fn spi3_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi3Clk)
    }
    #[doc = "SPI3 CS"]
    #[inline(always)]
    pub fn spi3_cs(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi3Cs)
    }
    #[doc = "SPI3 TX"]
    #[inline(always)]
    pub fn spi3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi3Tx)
    }
    #[doc = "SPI3 RX"]
    #[inline(always)]
    pub fn spi3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi3Rx)
    }
    #[doc = "SPI2 CLK"]
    #[inline(always)]
    pub fn spi2_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi2Clk)
    }
    #[doc = "SPI2 CS"]
    #[inline(always)]
    pub fn spi2_cs(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi2Cs)
    }
    #[doc = "SPI2 TX"]
    #[inline(always)]
    pub fn spi2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi2Tx)
    }
    #[doc = "SPI2 RX"]
    #[inline(always)]
    pub fn spi2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi2Rx)
    }
    #[doc = "RF Core SMI Command Link In"]
    #[inline(always)]
    pub fn rfc_smi_cl_in(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcSmiClIn)
    }
    #[doc = "RF Core SMI Command Link Out"]
    #[inline(always)]
    pub fn rfc_smi_cl_out(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcSmiClOut)
    }
    #[doc = "RF Core SMI Data Link In"]
    #[inline(always)]
    pub fn rfc_smi_dl_in(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcSmiDlIn)
    }
    #[doc = "RF Core SMI Data Link Out"]
    #[inline(always)]
    pub fn rfc_smi_dl_out(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcSmiDlOut)
    }
    #[doc = "RF Core Data In 1"]
    #[inline(always)]
    pub fn rfc_gpi1(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcGpi1)
    }
    #[doc = "RF Core Data In 0"]
    #[inline(always)]
    pub fn rfc_gpi0(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcGpi0)
    }
    #[doc = "RF Core Data Out 3"]
    #[inline(always)]
    pub fn rfc_gpo3(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcGpo3)
    }
    #[doc = "RF Core Data Out 2"]
    #[inline(always)]
    pub fn rfc_gpo2(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcGpo2)
    }
    #[doc = "RF Core Data Out 1"]
    #[inline(always)]
    pub fn rfc_gpo1(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcGpo1)
    }
    #[doc = "RF Core Data Out 0"]
    #[inline(always)]
    pub fn rfc_gpo0(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcGpo0)
    }
    #[doc = "RF Core Trace"]
    #[inline(always)]
    pub fn rfc_trc(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::RfcTrc)
    }
    #[doc = "I2S MCLK"]
    #[inline(always)]
    pub fn i2s_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2sMclk)
    }
    #[doc = "I2S BCLK"]
    #[inline(always)]
    pub fn i2s_bclk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2sBclk)
    }
    #[doc = "I2S WCLK"]
    #[inline(always)]
    pub fn i2s_wclk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2sWclk)
    }
    #[doc = "I2S Data 1"]
    #[inline(always)]
    pub fn i2s_ad1(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2sAd1)
    }
    #[doc = "I2S Data 0"]
    #[inline(always)]
    pub fn i2s_ad0(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2sAd0)
    }
    #[doc = "SPI1 CLK"]
    #[inline(always)]
    pub fn spi1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi1Clk)
    }
    #[doc = "SPI1 CS"]
    #[inline(always)]
    pub fn spi1_cs(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi1Cs)
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi1Tx)
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi1Rx)
    }
    #[doc = "CPU SWV"]
    #[inline(always)]
    pub fn cpu_swv(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::CpuSwv)
    }
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event7(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent7)
    }
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event6(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent6)
    }
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event5(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent5)
    }
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event4(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent4)
    }
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event3(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent3)
    }
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event2(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent2)
    }
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event1(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent1)
    }
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it through registers in the EVENT module, for example EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, and so on"]
    #[inline(always)]
    pub fn port_event0(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent0)
    }
    #[doc = "UART1 RTS"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart1Rts)
    }
    #[doc = "UART1 CTS"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart1Cts)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart1Tx)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart1Rx)
    }
    #[doc = "UART0 RTS"]
    #[inline(always)]
    pub fn uart0_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart0Rts)
    }
    #[doc = "UART0 CTS"]
    #[inline(always)]
    pub fn uart0_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart0Cts)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart0Tx)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Uart0Rx)
    }
    #[doc = "I2C0 Clock"]
    #[inline(always)]
    pub fn i2c0_msscl(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2c0Msscl)
    }
    #[doc = "I2C0 Data"]
    #[inline(always)]
    pub fn i2c0_mssda(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2c0Mssda)
    }
    #[doc = "SPI0 CLK"]
    #[inline(always)]
    pub fn spi0_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi0Clk)
    }
    #[doc = "SPI0 CS"]
    #[inline(always)]
    pub fn spi0_cs(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi0Cs)
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi0Tx)
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Spi0Rx)
    }
    #[doc = "AON 32 KHz clock (SCLK_LF)"]
    #[inline(always)]
    pub fn aon_clk32k(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::AonClk32k)
    }
    #[doc = "General Purpose IO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Gpio)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `IOEV_RTC_EN` reader - 8:8\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
pub type IoevRtcEnR = crate::BitReader;
#[doc = "Field `IOEV_RTC_EN` writer - 8:8\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
pub type IoevRtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "10:9\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iostr {
    #[doc = "3: Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    Max = 3,
    #[doc = "2: Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    Med = 2,
    #[doc = "1: Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    Min = 1,
    #[doc = "0: Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    Auto = 0,
}
impl From<Iostr> for u8 {
    #[inline(always)]
    fn from(variant: Iostr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iostr {
    type Ux = u8;
}
impl crate::IsEnum for Iostr {}
#[doc = "Field `IOSTR` reader - 10:9\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
pub type IostrR = crate::FieldReader<Iostr>;
impl IostrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iostr {
        match self.bits {
            3 => Iostr::Max,
            2 => Iostr::Med,
            1 => Iostr::Min,
            0 => Iostr::Auto,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Iostr::Max
    }
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == Iostr::Med
    }
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Iostr::Min
    }
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Iostr::Auto
    }
}
#[doc = "Field `IOSTR` writer - 10:9\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
pub type IostrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iostr, crate::Safe>;
impl<'a, REG> IostrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Iostr::Max)
    }
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(Iostr::Med)
    }
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Iostr::Min)
    }
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Iostr::Auto)
    }
}
#[doc = "12:11\\]
Selects IO current mode of this IO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iocurr {
    #[doc = "2: Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    _4_8ma = 2,
    #[doc = "1: High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    _4ma = 1,
    #[doc = "0: Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    _2ma = 0,
}
impl From<Iocurr> for u8 {
    #[inline(always)]
    fn from(variant: Iocurr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iocurr {
    type Ux = u8;
}
impl crate::IsEnum for Iocurr {}
#[doc = "Field `IOCURR` reader - 12:11\\]
Selects IO current mode of this IO."]
pub type IocurrR = crate::FieldReader<Iocurr>;
impl IocurrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iocurr> {
        match self.bits {
            2 => Some(Iocurr::_4_8ma),
            1 => Some(Iocurr::_4ma),
            0 => Some(Iocurr::_2ma),
            _ => None,
        }
    }
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn is_4_8ma(&self) -> bool {
        *self == Iocurr::_4_8ma
    }
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn is_4ma(&self) -> bool {
        *self == Iocurr::_4ma
    }
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn is_2ma(&self) -> bool {
        *self == Iocurr::_2ma
    }
}
#[doc = "Field `IOCURR` writer - 12:11\\]
Selects IO current mode of this IO."]
pub type IocurrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iocurr>;
impl<'a, REG> IocurrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn _4_8ma(self) -> &'a mut crate::W<REG> {
        self.variant(Iocurr::_4_8ma)
    }
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn _4ma(self) -> &'a mut crate::W<REG> {
        self.variant(Iocurr::_4ma)
    }
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    #[inline(always)]
    pub fn _2ma(self) -> &'a mut crate::W<REG> {
        self.variant(Iocurr::_2ma)
    }
}
#[doc = "Field `SLEW_RED` reader - 13:13\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
pub type SlewRedR = crate::BitReader;
#[doc = "Field `SLEW_RED` writer - 13:13\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
pub type SlewRedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "15:14\\]
Pull control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PullCtl {
    #[doc = "3: No pull"]
    Dis = 3,
    #[doc = "2: Pull up"]
    Up = 2,
    #[doc = "1: Pull down"]
    Dwn = 1,
}
impl From<PullCtl> for u8 {
    #[inline(always)]
    fn from(variant: PullCtl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PullCtl {
    type Ux = u8;
}
impl crate::IsEnum for PullCtl {}
#[doc = "Field `PULL_CTL` reader - 15:14\\]
Pull control"]
pub type PullCtlR = crate::FieldReader<PullCtl>;
impl PullCtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PullCtl> {
        match self.bits {
            3 => Some(PullCtl::Dis),
            2 => Some(PullCtl::Up),
            1 => Some(PullCtl::Dwn),
            _ => None,
        }
    }
    #[doc = "No pull"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PullCtl::Dis
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == PullCtl::Up
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn is_dwn(&self) -> bool {
        *self == PullCtl::Dwn
    }
}
#[doc = "Field `PULL_CTL` writer - 15:14\\]
Pull control"]
pub type PullCtlW<'a, REG> = crate::FieldWriter<'a, REG, 2, PullCtl>;
impl<'a, REG> PullCtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(PullCtl::Dis)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(PullCtl::Up)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn dwn(self) -> &'a mut crate::W<REG> {
        self.variant(PullCtl::Dwn)
    }
}
#[doc = "17:16\\]
Enable generation of edge detection events on this IO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EdgeDet {
    #[doc = "3: Positive and negative edge detection"]
    Both = 3,
    #[doc = "2: Positive edge detection"]
    Pos = 2,
    #[doc = "1: Negative edge detection"]
    Neg = 1,
    #[doc = "0: No edge detection"]
    None = 0,
}
impl From<EdgeDet> for u8 {
    #[inline(always)]
    fn from(variant: EdgeDet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EdgeDet {
    type Ux = u8;
}
impl crate::IsEnum for EdgeDet {}
#[doc = "Field `EDGE_DET` reader - 17:16\\]
Enable generation of edge detection events on this IO"]
pub type EdgeDetR = crate::FieldReader<EdgeDet>;
impl EdgeDetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdgeDet {
        match self.bits {
            3 => EdgeDet::Both,
            2 => EdgeDet::Pos,
            1 => EdgeDet::Neg,
            0 => EdgeDet::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Positive and negative edge detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EdgeDet::Both
    }
    #[doc = "Positive edge detection"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == EdgeDet::Pos
    }
    #[doc = "Negative edge detection"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == EdgeDet::Neg
    }
    #[doc = "No edge detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EdgeDet::None
    }
}
#[doc = "Field `EDGE_DET` writer - 17:16\\]
Enable generation of edge detection events on this IO"]
pub type EdgeDetW<'a, REG> = crate::FieldWriter<'a, REG, 2, EdgeDet, crate::Safe>;
impl<'a, REG> EdgeDetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive and negative edge detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeDet::Both)
    }
    #[doc = "Positive edge detection"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeDet::Pos)
    }
    #[doc = "Negative edge detection"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeDet::Neg)
    }
    #[doc = "No edge detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeDet::None)
    }
}
#[doc = "Field `EDGE_IRQ_EN` reader - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
pub type EdgeIrqEnR = crate::BitReader;
#[doc = "Field `EDGE_IRQ_EN` writer - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
pub type EdgeIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED19` reader - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader;
#[doc = "Field `RESERVED19` writer - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOEV_AON_PROG0_EN` reader - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
pub type IoevAonProg0EnR = crate::BitReader;
#[doc = "Field `IOEV_AON_PROG0_EN` writer - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
pub type IoevAonProg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEV_AON_PROG1_EN` reader - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
pub type IoevAonProg1EnR = crate::BitReader;
#[doc = "Field `IOEV_AON_PROG1_EN` writer - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
pub type IoevAonProg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEV_AON_PROG2_EN` reader - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
pub type IoevAonProg2EnR = crate::BitReader;
#[doc = "Field `IOEV_AON_PROG2_EN` writer - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
pub type IoevAonProg2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iomode {
    #[doc = "7: Open Source Inverted input / output"]
    OpensrcInv = 7,
    #[doc = "6: Open Source Normal input / output"]
    Opensrc = 6,
    #[doc = "5: Open Drain Inverted input / output"]
    OpendrInv = 5,
    #[doc = "4: Open Drain, Normal input / output"]
    Opendr = 4,
    #[doc = "1: Inverted input / ouput"]
    Inv = 1,
    #[doc = "0: Normal input / output"]
    Normal = 0,
}
impl From<Iomode> for u8 {
    #[inline(always)]
    fn from(variant: Iomode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iomode {
    type Ux = u8;
}
impl crate::IsEnum for Iomode {}
#[doc = "Field `IOMODE` reader - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
pub type IomodeR = crate::FieldReader<Iomode>;
impl IomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iomode> {
        match self.bits {
            7 => Some(Iomode::OpensrcInv),
            6 => Some(Iomode::Opensrc),
            5 => Some(Iomode::OpendrInv),
            4 => Some(Iomode::Opendr),
            1 => Some(Iomode::Inv),
            0 => Some(Iomode::Normal),
            _ => None,
        }
    }
    #[doc = "Open Source Inverted input / output"]
    #[inline(always)]
    pub fn is_opensrc_inv(&self) -> bool {
        *self == Iomode::OpensrcInv
    }
    #[doc = "Open Source Normal input / output"]
    #[inline(always)]
    pub fn is_opensrc(&self) -> bool {
        *self == Iomode::Opensrc
    }
    #[doc = "Open Drain Inverted input / output"]
    #[inline(always)]
    pub fn is_opendr_inv(&self) -> bool {
        *self == Iomode::OpendrInv
    }
    #[doc = "Open Drain, Normal input / output"]
    #[inline(always)]
    pub fn is_opendr(&self) -> bool {
        *self == Iomode::Opendr
    }
    #[doc = "Inverted input / ouput"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Iomode::Inv
    }
    #[doc = "Normal input / output"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Iomode::Normal
    }
}
#[doc = "Field `IOMODE` writer - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
pub type IomodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Iomode>;
impl<'a, REG> IomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open Source Inverted input / output"]
    #[inline(always)]
    pub fn opensrc_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::OpensrcInv)
    }
    #[doc = "Open Source Normal input / output"]
    #[inline(always)]
    pub fn opensrc(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Opensrc)
    }
    #[doc = "Open Drain Inverted input / output"]
    #[inline(always)]
    pub fn opendr_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::OpendrInv)
    }
    #[doc = "Open Drain, Normal input / output"]
    #[inline(always)]
    pub fn opendr(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Opendr)
    }
    #[doc = "Inverted input / ouput"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Inv)
    }
    #[doc = "Normal input / output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Normal)
    }
}
#[doc = "Field `WU_CFG` reader - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
pub type WuCfgR = crate::FieldReader;
#[doc = "Field `WU_CFG` writer - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
pub type WuCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IE` reader - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_EN` reader - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
pub type HystEnR = crate::BitReader;
#[doc = "Field `HYST_EN` writer - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
pub type HystEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEV_MCU_WU_EN` reader - 31:31\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
pub type IoevMcuWuEnR = crate::BitReader;
#[doc = "Field `IOEV_MCU_WU_EN` writer - 31:31\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
pub type IoevMcuWuEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Selects usage for DIO34 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
    #[inline(always)]
    pub fn port_id(&self) -> PortIdR {
        PortIdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
    #[inline(always)]
    pub fn ioev_rtc_en(&self) -> IoevRtcEnR {
        IoevRtcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline(always)]
    pub fn iostr(&self) -> IostrR {
        IostrR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Selects IO current mode of this IO."]
    #[inline(always)]
    pub fn iocurr(&self) -> IocurrR {
        IocurrR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub fn slew_red(&self) -> SlewRedR {
        SlewRedR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Pull control"]
    #[inline(always)]
    pub fn pull_ctl(&self) -> PullCtlR {
        PullCtlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable generation of edge detection events on this IO"]
    #[inline(always)]
    pub fn edge_det(&self) -> EdgeDetR {
        EdgeDetR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline(always)]
    pub fn edge_irq_en(&self) -> EdgeIrqEnR {
        EdgeIrqEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
    #[inline(always)]
    pub fn ioev_aon_prog0_en(&self) -> IoevAonProg0EnR {
        IoevAonProg0EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
    #[inline(always)]
    pub fn ioev_aon_prog1_en(&self) -> IoevAonProg1EnR {
        IoevAonProg1EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
    #[inline(always)]
    pub fn ioev_aon_prog2_en(&self) -> IoevAonProg2EnR {
        IoevAonProg2EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub fn iomode(&self) -> IomodeR {
        IomodeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub fn wu_cfg(&self) -> WuCfgR {
        WuCfgR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HystEnR {
        HystEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
    #[inline(always)]
    pub fn ioev_mcu_wu_en(&self) -> IoevMcuWuEnR {
        IoevMcuWuEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Selects usage for DIO34 Note: This field should not be written other than the times when PORT_ID value is specifically required to change."]
    #[inline(always)]
    #[must_use]
    pub fn port_id(&mut self) -> PortIdW<Iocfg34Spec> {
        PortIdW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert RTC event 1: Input edge detection asserts RTC event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_rtc_en(&mut self) -> IoevRtcEnW<Iocfg34Spec> {
        IoevRtcEnW::new(self, 8)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline(always)]
    #[must_use]
    pub fn iostr(&mut self) -> IostrW<Iocfg34Spec> {
        IostrW::new(self, 9)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Selects IO current mode of this IO."]
    #[inline(always)]
    #[must_use]
    pub fn iocurr(&mut self) -> IocurrW<Iocfg34Spec> {
        IocurrW::new(self, 11)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    #[must_use]
    pub fn slew_red(&mut self) -> SlewRedW<Iocfg34Spec> {
        SlewRedW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Pull control"]
    #[inline(always)]
    #[must_use]
    pub fn pull_ctl(&mut self) -> PullCtlW<Iocfg34Spec> {
        PullCtlW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable generation of edge detection events on this IO"]
    #[inline(always)]
    #[must_use]
    pub fn edge_det(&mut self) -> EdgeDetW<Iocfg34Spec> {
        EdgeDetW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn edge_irq_en(&mut self) -> EdgeIrqEnW<Iocfg34Spec> {
        EdgeIrqEnW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<Iocfg34Spec> {
        Reserved19W::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG0 event 1: Input edge detection asserts AON_PROG0 event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_aon_prog0_en(&mut self) -> IoevAonProg0EnW<Iocfg34Spec> {
        IoevAonProg0EnW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG1 event 1: Input edge detection asserts AON_PROG1 event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_aon_prog1_en(&mut self) -> IoevAonProg1EnW<Iocfg34Spec> {
        IoevAonProg1EnW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert AON_PROG2 event 1: Input edge detection asserts AON_PROG2 event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_aon_prog2_en(&mut self) -> IoevAonProg2EnW<Iocfg34Spec> {
        IoevAonProg2EnW::new(self, 23)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode Not applicable for IO configured for AON periph. signals and AUX PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn iomode(&mut self) -> IomodeW<Iocfg34Spec> {
        IomodeW::new(self, 24)
    }
    #[doc = "Bits 27:28 - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    #[must_use]
    pub fn wu_cfg(&mut self) -> WuCfgW<Iocfg34Spec> {
        WuCfgW::new(self, 27)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<Iocfg34Spec> {
        IeW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HystEnW<Iocfg34Spec> {
        HystEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Event asserted by this IO when edge detection is enabled 0: Input edge detection does not assert MCU_WU event 1: Input edge detection asserts MCU_WU event"]
    #[inline(always)]
    #[must_use]
    pub fn ioev_mcu_wu_en(&mut self) -> IoevMcuWuEnW<Iocfg34Spec> {
        IoevMcuWuEnW::new(self, 31)
    }
}
#[doc = "Configuration of DIO34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocfg34Spec;
impl crate::RegisterSpec for Iocfg34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocfg34::R`](R) reader structure"]
impl crate::Readable for Iocfg34Spec {}
#[doc = "`write(|w| ..)` method takes [`iocfg34::W`](W) writer structure"]
impl crate::Writable for Iocfg34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCFG34 to value 0xc000"]
impl crate::Resettable for Iocfg34Spec {
    const RESET_VALUE: u32 = 0xc000;
}
