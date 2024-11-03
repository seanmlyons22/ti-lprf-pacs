#[doc = "Register `LGPT0IN0SEL` reader"]
pub type R = crate::R<Lgpt0in0selSpec>;
#[doc = "Register `LGPT0IN0SEL` writer"]
pub type W = crate::W<Lgpt0in0selSpec>;
#[doc = "5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "57: LGPT3 ADC trigger event, controlled by LGPT3:ADCTRG setting"]
    Lgpt3Adc = 57,
    #[doc = "56: LGPT3 DMA request event, controlled by LGPT3:DMA setting"]
    Lgpt3Dma = 56,
    #[doc = "55: LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    Lgpt3Comb = 55,
    #[doc = "54: LGPT3 compare/capture output event 2, controlled by LGPT3:C2CFG setting"]
    Lgpt3c2 = 54,
    #[doc = "53: LGPT3 compare/capture output event 1, controlled by LGPT3:C1CFG setting"]
    Lgpt3c1 = 53,
    #[doc = "52: LGPT3 compare/capture output event 0, controlled by LGPT3:C0CFG setting"]
    Lgpt3c0 = 52,
    #[doc = "51: LGPT2 ADC trigger event, controlled by LGPT2:ADCTRG setting"]
    Lgpt2Adc = 51,
    #[doc = "50: LGPT2 DMA request event, controlled by LGPT2:DMA setting"]
    Lgpt2Dma = 50,
    #[doc = "49: LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    Lgpt2Comb = 49,
    #[doc = "48: LGPT0 compare/capture output event 2, controlled by LGPT2:C2CFG setting"]
    Lgpt2c2 = 48,
    #[doc = "47: LGPT2 compare/capture output event 1, controlled by LGPT2:C1CFG setting"]
    Lgpt2c1 = 47,
    #[doc = "46: LGPT2 compare/capture output event 0, controlled by LGPT2:C0CFG setting"]
    Lgpt2c0 = 46,
    #[doc = "44: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC2"]
    LrfdEvt2 = 44,
    #[doc = "43: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC1"]
    LrfdEvt1 = 43,
    #[doc = "42: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    LrfdEvt0 = 42,
    #[doc = "41: LGPT1 ADC trigger event, controlled by LGPT1:ADCTRG setting"]
    Lgpt1Adc = 41,
    #[doc = "40: LGPT1 DMA request event, controlled by LGPT1:DMA setting"]
    Lgpt1Dma = 40,
    #[doc = "39: LGPT1 compare/capture output event 2, controlled by LGPT1:C2CFG setting"]
    Lgpt1c2 = 39,
    #[doc = "38: LGPT1 compare/capture output event 1, controlled by LGPT1:C1CFG setting"]
    Lgpt1c1 = 38,
    #[doc = "37: LGPT1 compare/capture output event 0, controlled by LGPT1:C0CFG setting"]
    Lgpt1c0 = 37,
    #[doc = "36: LGPT0 ADC trigger event, controlled by LGPT0:ADCTRG setting"]
    Lgpt0Adc = 36,
    #[doc = "35: LGPT0 DMA request event, controlled by LGPT0:DMA setting"]
    Lgpt0Dma = 35,
    #[doc = "34: LGPT0 compare/capture output event 2, controlled by LGPT0:C2CFG setting"]
    Lgpt0c2 = 34,
    #[doc = "33: LGPT0 compare/capture output event 1, controlled by LGPT0:C1CFG setting"]
    Lgpt0c1 = 33,
    #[doc = "32: LGPT0 compare/capture output event 0, controlled by LGPT0:C0CFG setting"]
    Lgpt0c0 = 32,
    #[doc = "31: SYSTIM Channel 4 event, event flag is SYSTIM:MIS.EVT4"]
    Systim4 = 31,
    #[doc = "30: SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    Systim3 = 30,
    #[doc = "29: SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    Systim2 = 29,
    #[doc = "28: SYSTIM Channel 1 event, event flag is SYSTIM:MIS.EVT1"]
    Systim1 = 28,
    #[doc = "27: SYSTIM Channel 0 event, event flag is SYSTIM:MIS.EVT0"]
    Systim0 = 27,
    #[doc = "26: SYSTIM interrupt driven by synchronizing LFTICK signal to SVT clock"]
    SystimLt = 26,
    #[doc = "25: SYSTIM heartbeat, can be set by SYSTIM:TIMEBIT"]
    SystimHb = 25,
    #[doc = "24: Interrupt event from I2C0, interrupt flags can be found here I2C0:MIS"]
    I2c0Irq = 24,
    #[doc = "23: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    Uart0Comb = 23,
    #[doc = "22: AES accelerator combined interrupt request, interrupt flags can be found here AES:MIS"]
    AesComb = 22,
    #[doc = "21: DMA bus error, corresponds to DMA:ERROR.STATUS"]
    DmaErr = 21,
    #[doc = "20: DMA combined done interrupt, corresponding flags can be found here DMA:REQDONE"]
    DmaDoneComb = 20,
    #[doc = "19: LGPT1 combined interrupt, interrupt flags are found here LGPT1:MIS"]
    Lgpt1Comb = 19,
    #[doc = "18: LGPT0 combined interrupt, interrupt flags are found here LGPT0:MIS"]
    Lgpt0Comb = 18,
    #[doc = "17: ADC general published event, interrupt flags can be found here ADC:MIS1"]
    AdcEvt = 17,
    #[doc = "16: ADC combined interrupt request, interrupt flags can be found here ADC:MIS0"]
    AdcComb = 16,
    #[doc = "15: SPI0 combined interrupt request, interrupt flags can be found here SPI0:MIS"]
    Spi0Comb = 15,
    #[doc = "14: LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS2"]
    LrfdIrq2 = 14,
    #[doc = "13: LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS1"]
    LrfdIrq1 = 13,
    #[doc = "12: LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS0"]
    LrfdIrq0 = 12,
    #[doc = "11: NoWrapper Flash interrupt indicating that the flash operation has completed, interrupt flags can be found here FLASH:MIS"]
    FlashIrq = 11,
    #[doc = "10: GPIO generic published event, controlled by GPIO:EVTCFG"]
    GpioEvt = 10,
    #[doc = "9: GPIO combined wake up interrupt, interrupt flags can be found here GPIO:MIS"]
    GpioComb = 9,
    #[doc = "8: SYSTIM combined interrupt, interrupt flags are found here SYSTIM:MIS"]
    SystimComb = 8,
    #[doc = "7: IOC synchronous combined event, controlled by IOC:EVTCFG"]
    AonIocComb = 7,
    #[doc = "6: AON LPCMP interrupt, controlled by SYS0:LPCMPCFG"]
    AonLpmcmpIrq = 6,
    #[doc = "5: DebugSS combined interrupt, interrupt flags can be found here DBGSS:MIS"]
    AonDbgComb = 5,
    #[doc = "4: AON_RTC event, controlled by the RTC:IMASK setting"]
    AonRtcComb = 4,
    #[doc = "3: CKMD combined interrupt request, interrupt flags can be found here CKMD:MIS"]
    AonCkmComb = 3,
    #[doc = "2: PMU combined interrupt request for BATMON, interrupt flags can be found here PMUD:EVENT"]
    AonPmuComb = 2,
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Pubid> for u8 {
    #[inline(always)]
    fn from(variant: Pubid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pubid {
    type Ux = u8;
}
impl crate::IsEnum for Pubid {}
#[doc = "Field `PUBID` reader - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type PubidR = crate::FieldReader<Pubid>;
impl PubidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pubid> {
        match self.bits {
            57 => Some(Pubid::Lgpt3Adc),
            56 => Some(Pubid::Lgpt3Dma),
            55 => Some(Pubid::Lgpt3Comb),
            54 => Some(Pubid::Lgpt3c2),
            53 => Some(Pubid::Lgpt3c1),
            52 => Some(Pubid::Lgpt3c0),
            51 => Some(Pubid::Lgpt2Adc),
            50 => Some(Pubid::Lgpt2Dma),
            49 => Some(Pubid::Lgpt2Comb),
            48 => Some(Pubid::Lgpt2c2),
            47 => Some(Pubid::Lgpt2c1),
            46 => Some(Pubid::Lgpt2c0),
            44 => Some(Pubid::LrfdEvt2),
            43 => Some(Pubid::LrfdEvt1),
            42 => Some(Pubid::LrfdEvt0),
            41 => Some(Pubid::Lgpt1Adc),
            40 => Some(Pubid::Lgpt1Dma),
            39 => Some(Pubid::Lgpt1c2),
            38 => Some(Pubid::Lgpt1c1),
            37 => Some(Pubid::Lgpt1c0),
            36 => Some(Pubid::Lgpt0Adc),
            35 => Some(Pubid::Lgpt0Dma),
            34 => Some(Pubid::Lgpt0c2),
            33 => Some(Pubid::Lgpt0c1),
            32 => Some(Pubid::Lgpt0c0),
            31 => Some(Pubid::Systim4),
            30 => Some(Pubid::Systim3),
            29 => Some(Pubid::Systim2),
            28 => Some(Pubid::Systim1),
            27 => Some(Pubid::Systim0),
            26 => Some(Pubid::SystimLt),
            25 => Some(Pubid::SystimHb),
            24 => Some(Pubid::I2c0Irq),
            23 => Some(Pubid::Uart0Comb),
            22 => Some(Pubid::AesComb),
            21 => Some(Pubid::DmaErr),
            20 => Some(Pubid::DmaDoneComb),
            19 => Some(Pubid::Lgpt1Comb),
            18 => Some(Pubid::Lgpt0Comb),
            17 => Some(Pubid::AdcEvt),
            16 => Some(Pubid::AdcComb),
            15 => Some(Pubid::Spi0Comb),
            14 => Some(Pubid::LrfdIrq2),
            13 => Some(Pubid::LrfdIrq1),
            12 => Some(Pubid::LrfdIrq0),
            11 => Some(Pubid::FlashIrq),
            10 => Some(Pubid::GpioEvt),
            9 => Some(Pubid::GpioComb),
            8 => Some(Pubid::SystimComb),
            7 => Some(Pubid::AonIocComb),
            6 => Some(Pubid::AonLpmcmpIrq),
            5 => Some(Pubid::AonDbgComb),
            4 => Some(Pubid::AonRtcComb),
            3 => Some(Pubid::AonCkmComb),
            2 => Some(Pubid::AonPmuComb),
            0 => Some(Pubid::None),
            _ => None,
        }
    }
    #[doc = "LGPT3 ADC trigger event, controlled by LGPT3:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt3_adc(&self) -> bool {
        *self == Pubid::Lgpt3Adc
    }
    #[doc = "LGPT3 DMA request event, controlled by LGPT3:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt3_dma(&self) -> bool {
        *self == Pubid::Lgpt3Dma
    }
    #[doc = "LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    #[inline(always)]
    pub fn is_lgpt3_comb(&self) -> bool {
        *self == Pubid::Lgpt3Comb
    }
    #[doc = "LGPT3 compare/capture output event 2, controlled by LGPT3:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt3c2(&self) -> bool {
        *self == Pubid::Lgpt3c2
    }
    #[doc = "LGPT3 compare/capture output event 1, controlled by LGPT3:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt3c1(&self) -> bool {
        *self == Pubid::Lgpt3c1
    }
    #[doc = "LGPT3 compare/capture output event 0, controlled by LGPT3:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt3c0(&self) -> bool {
        *self == Pubid::Lgpt3c0
    }
    #[doc = "LGPT2 ADC trigger event, controlled by LGPT2:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt2_adc(&self) -> bool {
        *self == Pubid::Lgpt2Adc
    }
    #[doc = "LGPT2 DMA request event, controlled by LGPT2:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt2_dma(&self) -> bool {
        *self == Pubid::Lgpt2Dma
    }
    #[doc = "LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    #[inline(always)]
    pub fn is_lgpt2_comb(&self) -> bool {
        *self == Pubid::Lgpt2Comb
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT2:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt2c2(&self) -> bool {
        *self == Pubid::Lgpt2c2
    }
    #[doc = "LGPT2 compare/capture output event 1, controlled by LGPT2:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt2c1(&self) -> bool {
        *self == Pubid::Lgpt2c1
    }
    #[doc = "LGPT2 compare/capture output event 0, controlled by LGPT2:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt2c0(&self) -> bool {
        *self == Pubid::Lgpt2c0
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC2"]
    #[inline(always)]
    pub fn is_lrfd_evt2(&self) -> bool {
        *self == Pubid::LrfdEvt2
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC1"]
    #[inline(always)]
    pub fn is_lrfd_evt1(&self) -> bool {
        *self == Pubid::LrfdEvt1
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    #[inline(always)]
    pub fn is_lrfd_evt0(&self) -> bool {
        *self == Pubid::LrfdEvt0
    }
    #[doc = "LGPT1 ADC trigger event, controlled by LGPT1:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt1_adc(&self) -> bool {
        *self == Pubid::Lgpt1Adc
    }
    #[doc = "LGPT1 DMA request event, controlled by LGPT1:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt1_dma(&self) -> bool {
        *self == Pubid::Lgpt1Dma
    }
    #[doc = "LGPT1 compare/capture output event 2, controlled by LGPT1:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt1c2(&self) -> bool {
        *self == Pubid::Lgpt1c2
    }
    #[doc = "LGPT1 compare/capture output event 1, controlled by LGPT1:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt1c1(&self) -> bool {
        *self == Pubid::Lgpt1c1
    }
    #[doc = "LGPT1 compare/capture output event 0, controlled by LGPT1:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt1c0(&self) -> bool {
        *self == Pubid::Lgpt1c0
    }
    #[doc = "LGPT0 ADC trigger event, controlled by LGPT0:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt0_adc(&self) -> bool {
        *self == Pubid::Lgpt0Adc
    }
    #[doc = "LGPT0 DMA request event, controlled by LGPT0:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt0_dma(&self) -> bool {
        *self == Pubid::Lgpt0Dma
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT0:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt0c2(&self) -> bool {
        *self == Pubid::Lgpt0c2
    }
    #[doc = "LGPT0 compare/capture output event 1, controlled by LGPT0:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt0c1(&self) -> bool {
        *self == Pubid::Lgpt0c1
    }
    #[doc = "LGPT0 compare/capture output event 0, controlled by LGPT0:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt0c0(&self) -> bool {
        *self == Pubid::Lgpt0c0
    }
    #[doc = "SYSTIM Channel 4 event, event flag is SYSTIM:MIS.EVT4"]
    #[inline(always)]
    pub fn is_systim4(&self) -> bool {
        *self == Pubid::Systim4
    }
    #[doc = "SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    #[inline(always)]
    pub fn is_systim3(&self) -> bool {
        *self == Pubid::Systim3
    }
    #[doc = "SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    #[inline(always)]
    pub fn is_systim2(&self) -> bool {
        *self == Pubid::Systim2
    }
    #[doc = "SYSTIM Channel 1 event, event flag is SYSTIM:MIS.EVT1"]
    #[inline(always)]
    pub fn is_systim1(&self) -> bool {
        *self == Pubid::Systim1
    }
    #[doc = "SYSTIM Channel 0 event, event flag is SYSTIM:MIS.EVT0"]
    #[inline(always)]
    pub fn is_systim0(&self) -> bool {
        *self == Pubid::Systim0
    }
    #[doc = "SYSTIM interrupt driven by synchronizing LFTICK signal to SVT clock"]
    #[inline(always)]
    pub fn is_systim_lt(&self) -> bool {
        *self == Pubid::SystimLt
    }
    #[doc = "SYSTIM heartbeat, can be set by SYSTIM:TIMEBIT"]
    #[inline(always)]
    pub fn is_systim_hb(&self) -> bool {
        *self == Pubid::SystimHb
    }
    #[doc = "Interrupt event from I2C0, interrupt flags can be found here I2C0:MIS"]
    #[inline(always)]
    pub fn is_i2c0_irq(&self) -> bool {
        *self == Pubid::I2c0Irq
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == Pubid::Uart0Comb
    }
    #[doc = "AES accelerator combined interrupt request, interrupt flags can be found here AES:MIS"]
    #[inline(always)]
    pub fn is_aes_comb(&self) -> bool {
        *self == Pubid::AesComb
    }
    #[doc = "DMA bus error, corresponds to DMA:ERROR.STATUS"]
    #[inline(always)]
    pub fn is_dma_err(&self) -> bool {
        *self == Pubid::DmaErr
    }
    #[doc = "DMA combined done interrupt, corresponding flags can be found here DMA:REQDONE"]
    #[inline(always)]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == Pubid::DmaDoneComb
    }
    #[doc = "LGPT1 combined interrupt, interrupt flags are found here LGPT1:MIS"]
    #[inline(always)]
    pub fn is_lgpt1_comb(&self) -> bool {
        *self == Pubid::Lgpt1Comb
    }
    #[doc = "LGPT0 combined interrupt, interrupt flags are found here LGPT0:MIS"]
    #[inline(always)]
    pub fn is_lgpt0_comb(&self) -> bool {
        *self == Pubid::Lgpt0Comb
    }
    #[doc = "ADC general published event, interrupt flags can be found here ADC:MIS1"]
    #[inline(always)]
    pub fn is_adc_evt(&self) -> bool {
        *self == Pubid::AdcEvt
    }
    #[doc = "ADC combined interrupt request, interrupt flags can be found here ADC:MIS0"]
    #[inline(always)]
    pub fn is_adc_comb(&self) -> bool {
        *self == Pubid::AdcComb
    }
    #[doc = "SPI0 combined interrupt request, interrupt flags can be found here SPI0:MIS"]
    #[inline(always)]
    pub fn is_spi0_comb(&self) -> bool {
        *self == Pubid::Spi0Comb
    }
    #[doc = "LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS2"]
    #[inline(always)]
    pub fn is_lrfd_irq2(&self) -> bool {
        *self == Pubid::LrfdIrq2
    }
    #[doc = "LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS1"]
    #[inline(always)]
    pub fn is_lrfd_irq1(&self) -> bool {
        *self == Pubid::LrfdIrq1
    }
    #[doc = "LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS0"]
    #[inline(always)]
    pub fn is_lrfd_irq0(&self) -> bool {
        *self == Pubid::LrfdIrq0
    }
    #[doc = "NoWrapper Flash interrupt indicating that the flash operation has completed, interrupt flags can be found here FLASH:MIS"]
    #[inline(always)]
    pub fn is_flash_irq(&self) -> bool {
        *self == Pubid::FlashIrq
    }
    #[doc = "GPIO generic published event, controlled by GPIO:EVTCFG"]
    #[inline(always)]
    pub fn is_gpio_evt(&self) -> bool {
        *self == Pubid::GpioEvt
    }
    #[doc = "GPIO combined wake up interrupt, interrupt flags can be found here GPIO:MIS"]
    #[inline(always)]
    pub fn is_gpio_comb(&self) -> bool {
        *self == Pubid::GpioComb
    }
    #[doc = "SYSTIM combined interrupt, interrupt flags are found here SYSTIM:MIS"]
    #[inline(always)]
    pub fn is_systim_comb(&self) -> bool {
        *self == Pubid::SystimComb
    }
    #[doc = "IOC synchronous combined event, controlled by IOC:EVTCFG"]
    #[inline(always)]
    pub fn is_aon_ioc_comb(&self) -> bool {
        *self == Pubid::AonIocComb
    }
    #[doc = "AON LPCMP interrupt, controlled by SYS0:LPCMPCFG"]
    #[inline(always)]
    pub fn is_aon_lpmcmp_irq(&self) -> bool {
        *self == Pubid::AonLpmcmpIrq
    }
    #[doc = "DebugSS combined interrupt, interrupt flags can be found here DBGSS:MIS"]
    #[inline(always)]
    pub fn is_aon_dbg_comb(&self) -> bool {
        *self == Pubid::AonDbgComb
    }
    #[doc = "AON_RTC event, controlled by the RTC:IMASK setting"]
    #[inline(always)]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == Pubid::AonRtcComb
    }
    #[doc = "CKMD combined interrupt request, interrupt flags can be found here CKMD:MIS"]
    #[inline(always)]
    pub fn is_aon_ckm_comb(&self) -> bool {
        *self == Pubid::AonCkmComb
    }
    #[doc = "PMU combined interrupt request for BATMON, interrupt flags can be found here PMUD:EVENT"]
    #[inline(always)]
    pub fn is_aon_pmu_comb(&self) -> bool {
        *self == Pubid::AonPmuComb
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pubid::None
    }
}
#[doc = "Field `PUBID` writer - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type PubidW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pubid>;
impl<'a, REG> PubidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LGPT3 ADC trigger event, controlled by LGPT3:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt3_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Adc)
    }
    #[doc = "LGPT3 DMA request event, controlled by LGPT3:DMA setting"]
    #[inline(always)]
    pub fn lgpt3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Dma)
    }
    #[doc = "LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    #[inline(always)]
    pub fn lgpt3_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Comb)
    }
    #[doc = "LGPT3 compare/capture output event 2, controlled by LGPT3:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt3c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3c2)
    }
    #[doc = "LGPT3 compare/capture output event 1, controlled by LGPT3:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt3c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3c1)
    }
    #[doc = "LGPT3 compare/capture output event 0, controlled by LGPT3:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt3c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3c0)
    }
    #[doc = "LGPT2 ADC trigger event, controlled by LGPT2:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt2_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2Adc)
    }
    #[doc = "LGPT2 DMA request event, controlled by LGPT2:DMA setting"]
    #[inline(always)]
    pub fn lgpt2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2Dma)
    }
    #[doc = "LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    #[inline(always)]
    pub fn lgpt2_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2Comb)
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT2:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt2c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2c2)
    }
    #[doc = "LGPT2 compare/capture output event 1, controlled by LGPT2:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt2c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2c1)
    }
    #[doc = "LGPT2 compare/capture output event 0, controlled by LGPT2:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt2c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2c0)
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC2"]
    #[inline(always)]
    pub fn lrfd_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdEvt2)
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC1"]
    #[inline(always)]
    pub fn lrfd_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdEvt1)
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    #[inline(always)]
    pub fn lrfd_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdEvt0)
    }
    #[doc = "LGPT1 ADC trigger event, controlled by LGPT1:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt1_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1Adc)
    }
    #[doc = "LGPT1 DMA request event, controlled by LGPT1:DMA setting"]
    #[inline(always)]
    pub fn lgpt1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1Dma)
    }
    #[doc = "LGPT1 compare/capture output event 2, controlled by LGPT1:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt1c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1c2)
    }
    #[doc = "LGPT1 compare/capture output event 1, controlled by LGPT1:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt1c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1c1)
    }
    #[doc = "LGPT1 compare/capture output event 0, controlled by LGPT1:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt1c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1c0)
    }
    #[doc = "LGPT0 ADC trigger event, controlled by LGPT0:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt0_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0Adc)
    }
    #[doc = "LGPT0 DMA request event, controlled by LGPT0:DMA setting"]
    #[inline(always)]
    pub fn lgpt0_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0Dma)
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT0:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt0c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0c2)
    }
    #[doc = "LGPT0 compare/capture output event 1, controlled by LGPT0:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt0c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0c1)
    }
    #[doc = "LGPT0 compare/capture output event 0, controlled by LGPT0:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt0c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0c0)
    }
    #[doc = "SYSTIM Channel 4 event, event flag is SYSTIM:MIS.EVT4"]
    #[inline(always)]
    pub fn systim4(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim4)
    }
    #[doc = "SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    #[inline(always)]
    pub fn systim3(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim3)
    }
    #[doc = "SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    #[inline(always)]
    pub fn systim2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim2)
    }
    #[doc = "SYSTIM Channel 1 event, event flag is SYSTIM:MIS.EVT1"]
    #[inline(always)]
    pub fn systim1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim1)
    }
    #[doc = "SYSTIM Channel 0 event, event flag is SYSTIM:MIS.EVT0"]
    #[inline(always)]
    pub fn systim0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim0)
    }
    #[doc = "SYSTIM interrupt driven by synchronizing LFTICK signal to SVT clock"]
    #[inline(always)]
    pub fn systim_lt(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::SystimLt)
    }
    #[doc = "SYSTIM heartbeat, can be set by SYSTIM:TIMEBIT"]
    #[inline(always)]
    pub fn systim_hb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::SystimHb)
    }
    #[doc = "Interrupt event from I2C0, interrupt flags can be found here I2C0:MIS"]
    #[inline(always)]
    pub fn i2c0_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::I2c0Irq)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn uart0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Uart0Comb)
    }
    #[doc = "AES accelerator combined interrupt request, interrupt flags can be found here AES:MIS"]
    #[inline(always)]
    pub fn aes_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AesComb)
    }
    #[doc = "DMA bus error, corresponds to DMA:ERROR.STATUS"]
    #[inline(always)]
    pub fn dma_err(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::DmaErr)
    }
    #[doc = "DMA combined done interrupt, corresponding flags can be found here DMA:REQDONE"]
    #[inline(always)]
    pub fn dma_done_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::DmaDoneComb)
    }
    #[doc = "LGPT1 combined interrupt, interrupt flags are found here LGPT1:MIS"]
    #[inline(always)]
    pub fn lgpt1_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1Comb)
    }
    #[doc = "LGPT0 combined interrupt, interrupt flags are found here LGPT0:MIS"]
    #[inline(always)]
    pub fn lgpt0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0Comb)
    }
    #[doc = "ADC general published event, interrupt flags can be found here ADC:MIS1"]
    #[inline(always)]
    pub fn adc_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AdcEvt)
    }
    #[doc = "ADC combined interrupt request, interrupt flags can be found here ADC:MIS0"]
    #[inline(always)]
    pub fn adc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AdcComb)
    }
    #[doc = "SPI0 combined interrupt request, interrupt flags can be found here SPI0:MIS"]
    #[inline(always)]
    pub fn spi0_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Spi0Comb)
    }
    #[doc = "LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS2"]
    #[inline(always)]
    pub fn lrfd_irq2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdIrq2)
    }
    #[doc = "LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS1"]
    #[inline(always)]
    pub fn lrfd_irq1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdIrq1)
    }
    #[doc = "LRFD combined event, interrupt flags can be found here LRFDDBELL:MIS0"]
    #[inline(always)]
    pub fn lrfd_irq0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdIrq0)
    }
    #[doc = "NoWrapper Flash interrupt indicating that the flash operation has completed, interrupt flags can be found here FLASH:MIS"]
    #[inline(always)]
    pub fn flash_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::FlashIrq)
    }
    #[doc = "GPIO generic published event, controlled by GPIO:EVTCFG"]
    #[inline(always)]
    pub fn gpio_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::GpioEvt)
    }
    #[doc = "GPIO combined wake up interrupt, interrupt flags can be found here GPIO:MIS"]
    #[inline(always)]
    pub fn gpio_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::GpioComb)
    }
    #[doc = "SYSTIM combined interrupt, interrupt flags are found here SYSTIM:MIS"]
    #[inline(always)]
    pub fn systim_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::SystimComb)
    }
    #[doc = "IOC synchronous combined event, controlled by IOC:EVTCFG"]
    #[inline(always)]
    pub fn aon_ioc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonIocComb)
    }
    #[doc = "AON LPCMP interrupt, controlled by SYS0:LPCMPCFG"]
    #[inline(always)]
    pub fn aon_lpmcmp_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonLpmcmpIrq)
    }
    #[doc = "DebugSS combined interrupt, interrupt flags can be found here DBGSS:MIS"]
    #[inline(always)]
    pub fn aon_dbg_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonDbgComb)
    }
    #[doc = "AON_RTC event, controlled by the RTC:IMASK setting"]
    #[inline(always)]
    pub fn aon_rtc_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonRtcComb)
    }
    #[doc = "CKMD combined interrupt request, interrupt flags can be found here CKMD:MIS"]
    #[inline(always)]
    pub fn aon_ckm_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonCkmComb)
    }
    #[doc = "PMU combined interrupt request for BATMON, interrupt flags can be found here PMUD:EVENT"]
    #[inline(always)]
    pub fn aon_pmu_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AonPmuComb)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::None)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn pubid(&self) -> PubidR {
        PubidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pubid(&mut self) -> PubidW<Lgpt0in0selSpec> {
        PubidW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Lgpt0in0selSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Output Selection for CPU Interrupt LGPT0IN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt0in0sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt0in0sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lgpt0in0selSpec;
impl crate::RegisterSpec for Lgpt0in0selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lgpt0in0sel::R`](R) reader structure"]
impl crate::Readable for Lgpt0in0selSpec {}
#[doc = "`write(|w| ..)` method takes [`lgpt0in0sel::W`](W) writer structure"]
impl crate::Writable for Lgpt0in0selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LGPT0IN0SEL to value 0"]
impl crate::Resettable for Lgpt0in0selSpec {
    const RESET_VALUE: u32 = 0;
}
