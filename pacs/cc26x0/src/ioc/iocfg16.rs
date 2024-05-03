#[doc = "Register `IOCFG16` reader"]
pub type R = crate::R<Iocfg16Spec>;
#[doc = "Register `IOCFG16` writer"]
pub type W = crate::W<Iocfg16Spec>;
#[doc = "5:0\\]
Selects usage for DIO16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PortId {
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
    #[doc = "36: SSI1 CLK"]
    Ssi1Clk = 36,
    #[doc = "35: SSI1 FSS"]
    Ssi1Fss = 35,
    #[doc = "34: SSI1 TX"]
    Ssi1Tx = 34,
    #[doc = "33: SSI1 RX"]
    Ssi1Rx = 33,
    #[doc = "32: CPU SWV"]
    CpuSwv = 32,
    #[doc = "30: PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent7 = 30,
    #[doc = "29: PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent6 = 29,
    #[doc = "28: PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent5 = 28,
    #[doc = "27: PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent4 = 27,
    #[doc = "26: PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent3 = 26,
    #[doc = "25: PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent2 = 25,
    #[doc = "24: PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent1 = 24,
    #[doc = "23: PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PortEvent0 = 23,
    #[doc = "18: UART0 RTS"]
    Uart0Rts = 18,
    #[doc = "17: UART0 CTS"]
    Uart0Cts = 17,
    #[doc = "16: UART0 TX"]
    Uart0Tx = 16,
    #[doc = "15: UART0 RX"]
    Uart0Rx = 15,
    #[doc = "14: I2C Clock"]
    I2cMsscl = 14,
    #[doc = "13: I2C Data"]
    I2cMssda = 13,
    #[doc = "12: SSI0 CLK"]
    Ssi0Clk = 12,
    #[doc = "11: SSI0 FSS"]
    Ssi0Fss = 11,
    #[doc = "10: SSI0 TX"]
    Ssi0Tx = 10,
    #[doc = "9: SSI0 RX"]
    Ssi0Rx = 9,
    #[doc = "8: AUX IO"]
    AuxIo = 8,
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
#[doc = "Field `PORT_ID` reader - 5:0\\]
Selects usage for DIO16"]
pub type PortIdR = crate::FieldReader<PortId>;
impl PortIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PortId> {
        match self.bits {
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
            36 => Some(PortId::Ssi1Clk),
            35 => Some(PortId::Ssi1Fss),
            34 => Some(PortId::Ssi1Tx),
            33 => Some(PortId::Ssi1Rx),
            32 => Some(PortId::CpuSwv),
            30 => Some(PortId::PortEvent7),
            29 => Some(PortId::PortEvent6),
            28 => Some(PortId::PortEvent5),
            27 => Some(PortId::PortEvent4),
            26 => Some(PortId::PortEvent3),
            25 => Some(PortId::PortEvent2),
            24 => Some(PortId::PortEvent1),
            23 => Some(PortId::PortEvent0),
            18 => Some(PortId::Uart0Rts),
            17 => Some(PortId::Uart0Cts),
            16 => Some(PortId::Uart0Tx),
            15 => Some(PortId::Uart0Rx),
            14 => Some(PortId::I2cMsscl),
            13 => Some(PortId::I2cMssda),
            12 => Some(PortId::Ssi0Clk),
            11 => Some(PortId::Ssi0Fss),
            10 => Some(PortId::Ssi0Tx),
            9 => Some(PortId::Ssi0Rx),
            8 => Some(PortId::AuxIo),
            7 => Some(PortId::AonClk32k),
            0 => Some(PortId::Gpio),
            _ => None,
        }
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
    #[doc = "SSI1 CLK"]
    #[inline(always)]
    pub fn is_ssi1_clk(&self) -> bool {
        *self == PortId::Ssi1Clk
    }
    #[doc = "SSI1 FSS"]
    #[inline(always)]
    pub fn is_ssi1_fss(&self) -> bool {
        *self == PortId::Ssi1Fss
    }
    #[doc = "SSI1 TX"]
    #[inline(always)]
    pub fn is_ssi1_tx(&self) -> bool {
        *self == PortId::Ssi1Tx
    }
    #[doc = "SSI1 RX"]
    #[inline(always)]
    pub fn is_ssi1_rx(&self) -> bool {
        *self == PortId::Ssi1Rx
    }
    #[doc = "CPU SWV"]
    #[inline(always)]
    pub fn is_cpu_swv(&self) -> bool {
        *self == PortId::CpuSwv
    }
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event7(&self) -> bool {
        *self == PortId::PortEvent7
    }
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event6(&self) -> bool {
        *self == PortId::PortEvent6
    }
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event5(&self) -> bool {
        *self == PortId::PortEvent5
    }
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event4(&self) -> bool {
        *self == PortId::PortEvent4
    }
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event3(&self) -> bool {
        *self == PortId::PortEvent3
    }
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event2(&self) -> bool {
        *self == PortId::PortEvent2
    }
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event1(&self) -> bool {
        *self == PortId::PortEvent1
    }
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn is_port_event0(&self) -> bool {
        *self == PortId::PortEvent0
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
    #[doc = "I2C Clock"]
    #[inline(always)]
    pub fn is_i2c_msscl(&self) -> bool {
        *self == PortId::I2cMsscl
    }
    #[doc = "I2C Data"]
    #[inline(always)]
    pub fn is_i2c_mssda(&self) -> bool {
        *self == PortId::I2cMssda
    }
    #[doc = "SSI0 CLK"]
    #[inline(always)]
    pub fn is_ssi0_clk(&self) -> bool {
        *self == PortId::Ssi0Clk
    }
    #[doc = "SSI0 FSS"]
    #[inline(always)]
    pub fn is_ssi0_fss(&self) -> bool {
        *self == PortId::Ssi0Fss
    }
    #[doc = "SSI0 TX"]
    #[inline(always)]
    pub fn is_ssi0_tx(&self) -> bool {
        *self == PortId::Ssi0Tx
    }
    #[doc = "SSI0 RX"]
    #[inline(always)]
    pub fn is_ssi0_rx(&self) -> bool {
        *self == PortId::Ssi0Rx
    }
    #[doc = "AUX IO"]
    #[inline(always)]
    pub fn is_aux_io(&self) -> bool {
        *self == PortId::AuxIo
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
#[doc = "Field `PORT_ID` writer - 5:0\\]
Selects usage for DIO16"]
pub type PortIdW<'a, REG> = crate::FieldWriter<'a, REG, 6, PortId>;
impl<'a, REG> PortIdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
    #[doc = "SSI1 CLK"]
    #[inline(always)]
    pub fn ssi1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi1Clk)
    }
    #[doc = "SSI1 FSS"]
    #[inline(always)]
    pub fn ssi1_fss(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi1Fss)
    }
    #[doc = "SSI1 TX"]
    #[inline(always)]
    pub fn ssi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi1Tx)
    }
    #[doc = "SSI1 RX"]
    #[inline(always)]
    pub fn ssi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi1Rx)
    }
    #[doc = "CPU SWV"]
    #[inline(always)]
    pub fn cpu_swv(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::CpuSwv)
    }
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event7(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent7)
    }
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event6(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent6)
    }
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event5(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent5)
    }
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event4(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent4)
    }
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event3(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent3)
    }
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event2(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent2)
    }
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event1(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent1)
    }
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline(always)]
    pub fn port_event0(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::PortEvent0)
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
    #[doc = "I2C Clock"]
    #[inline(always)]
    pub fn i2c_msscl(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2cMsscl)
    }
    #[doc = "I2C Data"]
    #[inline(always)]
    pub fn i2c_mssda(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::I2cMssda)
    }
    #[doc = "SSI0 CLK"]
    #[inline(always)]
    pub fn ssi0_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi0Clk)
    }
    #[doc = "SSI0 FSS"]
    #[inline(always)]
    pub fn ssi0_fss(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi0Fss)
    }
    #[doc = "SSI0 TX"]
    #[inline(always)]
    pub fn ssi0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi0Tx)
    }
    #[doc = "SSI0 RX"]
    #[inline(always)]
    pub fn ssi0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::Ssi0Rx)
    }
    #[doc = "AUX IO"]
    #[inline(always)]
    pub fn aux_io(self) -> &'a mut crate::W<REG> {
        self.variant(PortId::AuxIo)
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
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "9:8\\]
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
#[doc = "Field `IOSTR` reader - 9:8\\]
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
#[doc = "Field `IOSTR` writer - 9:8\\]
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
#[doc = "11:10\\]
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
#[doc = "Field `IOCURR` reader - 11:10\\]
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
#[doc = "Field `IOCURR` writer - 11:10\\]
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
#[doc = "Field `SLEW_RED` reader - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
pub type SlewRedR = crate::BitReader;
#[doc = "Field `SLEW_RED` writer - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
pub type SlewRedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "14:13\\]
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
#[doc = "Field `PULL_CTL` reader - 14:13\\]
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
#[doc = "Field `PULL_CTL` writer - 14:13\\]
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
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `RESERVED19` reader - 23:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader;
#[doc = "Field `RESERVED19` writer - 23:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "26:24\\]
IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior.\n\nValue on reset: 0"]
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
IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
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
IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
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
If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
pub type WuCfgR = crate::FieldReader;
#[doc = "Field `WU_CFG` writer - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
pub type WuCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IE` reader - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_EN` reader - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
pub type HystEnR = crate::BitReader;
#[doc = "Field `HYST_EN` writer - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
pub type HystEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects usage for DIO16"]
    #[inline(always)]
    pub fn port_id(&self) -> PortIdR {
        PortIdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline(always)]
    pub fn iostr(&self) -> IostrR {
        IostrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects IO current mode of this IO."]
    #[inline(always)]
    pub fn iocurr(&self) -> IocurrR {
        IocurrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub fn slew_red(&self) -> SlewRedR {
        SlewRedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Pull control"]
    #[inline(always)]
    pub fn pull_ctl(&self) -> PullCtlR {
        PullCtlR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bits 19:23 - 23:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub fn iomode(&self) -> IomodeR {
        IomodeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub fn wu_cfg(&self) -> WuCfgR {
        WuCfgR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects usage for DIO16"]
    #[inline(always)]
    #[must_use]
    pub fn port_id(&mut self) -> PortIdW<Iocfg16Spec> {
        PortIdW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Iocfg16Spec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline(always)]
    #[must_use]
    pub fn iostr(&mut self) -> IostrW<Iocfg16Spec> {
        IostrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects IO current mode of this IO."]
    #[inline(always)]
    #[must_use]
    pub fn iocurr(&mut self) -> IocurrW<Iocfg16Spec> {
        IocurrW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    #[must_use]
    pub fn slew_red(&mut self) -> SlewRedW<Iocfg16Spec> {
        SlewRedW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Pull control"]
    #[inline(always)]
    #[must_use]
    pub fn pull_ctl(&mut self) -> PullCtlW<Iocfg16Spec> {
        PullCtlW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Iocfg16Spec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable generation of edge detection events on this IO"]
    #[inline(always)]
    #[must_use]
    pub fn edge_det(&mut self) -> EdgeDetW<Iocfg16Spec> {
        EdgeDetW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn edge_irq_en(&mut self) -> EdgeIrqEnW<Iocfg16Spec> {
        EdgeIrqEnW::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<Iocfg16Spec> {
        Reserved19W::new(self, 19)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn iomode(&mut self) -> IomodeW<Iocfg16Spec> {
        IomodeW::new(self, 24)
    }
    #[doc = "Bits 27:28 - 28:27\\]
If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    #[must_use]
    pub fn wu_cfg(&mut self) -> WuCfgW<Iocfg16Spec> {
        WuCfgW::new(self, 27)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<Iocfg16Spec> {
        IeW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HystEnW<Iocfg16Spec> {
        HystEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Iocfg16Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Configuration of DIO16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocfg16Spec;
impl crate::RegisterSpec for Iocfg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocfg16::R`](R) reader structure"]
impl crate::Readable for Iocfg16Spec {}
#[doc = "`write(|w| ..)` method takes [`iocfg16::W`](W) writer structure"]
impl crate::Writable for Iocfg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCFG16 to value 0x0008_6000"]
impl crate::Resettable for Iocfg16Spec {
    const RESET_VALUE: u32 = 0x0008_6000;
}
