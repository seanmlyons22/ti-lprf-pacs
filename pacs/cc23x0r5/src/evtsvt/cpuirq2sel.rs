#[doc = "Register `CPUIRQ2SEL` reader"]
pub type R = crate::R<Cpuirq2selSpec>;
#[doc = "Register `CPUIRQ2SEL` writer"]
pub type W = crate::W<Cpuirq2selSpec>;
#[doc = "5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "55: LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    Lgpt3Comb = 55,
    #[doc = "49: LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    Lgpt2Comb = 49,
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
            55 => Some(Pubid::Lgpt3Comb),
            49 => Some(Pubid::Lgpt2Comb),
            24 => Some(Pubid::I2c0Irq),
            23 => Some(Pubid::Uart0Comb),
            22 => Some(Pubid::AesComb),
            21 => Some(Pubid::DmaErr),
            20 => Some(Pubid::DmaDoneComb),
            19 => Some(Pubid::Lgpt1Comb),
            18 => Some(Pubid::Lgpt0Comb),
            16 => Some(Pubid::AdcComb),
            15 => Some(Pubid::Spi0Comb),
            14 => Some(Pubid::LrfdIrq2),
            13 => Some(Pubid::LrfdIrq1),
            12 => Some(Pubid::LrfdIrq0),
            11 => Some(Pubid::FlashIrq),
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
    #[doc = "LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    #[inline(always)]
    pub fn is_lgpt3_comb(&self) -> bool {
        *self == Pubid::Lgpt3Comb
    }
    #[doc = "LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    #[inline(always)]
    pub fn is_lgpt2_comb(&self) -> bool {
        *self == Pubid::Lgpt2Comb
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
    #[doc = "LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    #[inline(always)]
    pub fn lgpt3_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Comb)
    }
    #[doc = "LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    #[inline(always)]
    pub fn lgpt2_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2Comb)
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
    pub fn pubid(&mut self) -> PubidW<Cpuirq2selSpec> {
        PubidW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Cpuirq2selSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Output Selection for CPU Interrupt CPUIRQ2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq2sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq2sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirq2selSpec;
impl crate::RegisterSpec for Cpuirq2selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirq2sel::R`](R) reader structure"]
impl crate::Readable for Cpuirq2selSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuirq2sel::W`](W) writer structure"]
impl crate::Writable for Cpuirq2selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQ2SEL to value 0"]
impl crate::Resettable for Cpuirq2selSpec {
    const RESET_VALUE: u32 = 0;
}
