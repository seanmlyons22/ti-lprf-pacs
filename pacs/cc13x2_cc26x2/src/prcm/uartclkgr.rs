#[doc = "Register `UARTCLKGR` reader"]
pub type R = crate::R<UartclkgrSpec>;
#[doc = "Register `UARTCLKGR` writer"]
pub type W = crate::W<UartclkgrSpec>;
#[doc = "1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEn {
    #[doc = "2: Enable clock for UART1"]
    Uart1 = 2,
    #[doc = "1: Enable clock for UART0"]
    Uart0 = 1,
}
impl From<ClkEn> for u8 {
    #[inline(always)]
    fn from(variant: ClkEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkEn {
    type Ux = u8;
}
impl crate::IsEnum for ClkEn {}
#[doc = "Field `CLK_EN` reader - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::FieldReader<ClkEn>;
impl ClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEn> {
        match self.bits {
            2 => Some(ClkEn::Uart1),
            1 => Some(ClkEn::Uart0),
            _ => None,
        }
    }
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == ClkEn::Uart1
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == ClkEn::Uart0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkEn>;
impl<'a, REG> ClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Uart1)
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Uart0)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AmClkEn {
    #[doc = "2: Enable clock for UART1"]
    AmUart1 = 2,
    #[doc = "1: Enable clock for UART0"]
    AmUart0 = 1,
}
impl From<AmClkEn> for u8 {
    #[inline(always)]
    fn from(variant: AmClkEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AmClkEn {
    type Ux = u8;
}
impl crate::IsEnum for AmClkEn {}
#[doc = "Field `AM_CLK_EN` reader - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AmClkEnR = crate::FieldReader<AmClkEn>;
impl AmClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AmClkEn> {
        match self.bits {
            2 => Some(AmClkEn::AmUart1),
            1 => Some(AmClkEn::AmUart0),
            _ => None,
        }
    }
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn is_am_uart1(&self) -> bool {
        *self == AmClkEn::AmUart1
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn is_am_uart0(&self) -> bool {
        *self == AmClkEn::AmUart0
    }
}
#[doc = "Field `AM_CLK_EN` writer - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AmClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, AmClkEn>;
impl<'a, REG> AmClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn am_uart1(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::AmUart1)
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn am_uart0(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::AmUart0)
    }
}
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AmClkEnR {
        AmClkEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<UartclkgrSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<UartclkgrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn am_clk_en(&mut self) -> AmClkEnW<UartclkgrSpec> {
        AmClkEnW::new(self, 8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<UartclkgrSpec> {
        Reserved10W::new(self, 10)
    }
}
#[doc = "UART Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartclkgrSpec;
impl crate::RegisterSpec for UartclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartclkgr::R`](R) reader structure"]
impl crate::Readable for UartclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartclkgr::W`](W) writer structure"]
impl crate::Writable for UartclkgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTCLKGR to value 0"]
impl crate::Resettable for UartclkgrSpec {
    const RESET_VALUE: u32 = 0;
}
