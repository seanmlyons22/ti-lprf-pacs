#[doc = "Register `UARTCLKGR` reader"]
pub type R = crate::R<UartclkgrSpec>;
#[doc = "Register `UARTCLKGR` writer"]
pub type W = crate::W<UartclkgrSpec>;
#[doc = "3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEn {
    #[doc = "8: Enable clock for UART3"]
    Uart3 = 8,
    #[doc = "4: Enable clock for UART2"]
    Uart2 = 4,
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
#[doc = "Field `CLK_EN` reader - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::FieldReader<ClkEn>;
impl ClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEn> {
        match self.bits {
            8 => Some(ClkEn::Uart3),
            4 => Some(ClkEn::Uart2),
            2 => Some(ClkEn::Uart1),
            1 => Some(ClkEn::Uart0),
            _ => None,
        }
    }
    #[doc = "Enable clock for UART3"]
    #[inline(always)]
    pub fn is_uart3(&self) -> bool {
        *self == ClkEn::Uart3
    }
    #[doc = "Enable clock for UART2"]
    #[inline(always)]
    pub fn is_uart2(&self) -> bool {
        *self == ClkEn::Uart2
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
#[doc = "Field `CLK_EN` writer - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 4, ClkEn>;
impl<'a, REG> ClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for UART3"]
    #[inline(always)]
    pub fn uart3(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Uart3)
    }
    #[doc = "Enable clock for UART2"]
    #[inline(always)]
    pub fn uart2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::Uart2)
    }
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
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AmClkEn {
    #[doc = "8: Enable clock for UART3"]
    Uart3 = 8,
    #[doc = "4: Enable clock for UART2"]
    Uart2 = 4,
    #[doc = "2: Enable clock for UART1"]
    Uart1 = 2,
    #[doc = "1: Enable clock for UART0"]
    Uart0 = 1,
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
#[doc = "Field `AM_CLK_EN` reader - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AmClkEnR = crate::FieldReader<AmClkEn>;
impl AmClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AmClkEn> {
        match self.bits {
            8 => Some(AmClkEn::Uart3),
            4 => Some(AmClkEn::Uart2),
            2 => Some(AmClkEn::Uart1),
            1 => Some(AmClkEn::Uart0),
            _ => None,
        }
    }
    #[doc = "Enable clock for UART3"]
    #[inline(always)]
    pub fn is_uart3(&self) -> bool {
        *self == AmClkEn::Uart3
    }
    #[doc = "Enable clock for UART2"]
    #[inline(always)]
    pub fn is_uart2(&self) -> bool {
        *self == AmClkEn::Uart2
    }
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == AmClkEn::Uart1
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == AmClkEn::Uart0
    }
}
#[doc = "Field `AM_CLK_EN` writer - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AmClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 4, AmClkEn>;
impl<'a, REG> AmClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for UART3"]
    #[inline(always)]
    pub fn uart3(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::Uart3)
    }
    #[doc = "Enable clock for UART2"]
    #[inline(always)]
    pub fn uart2(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::Uart2)
    }
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::Uart1)
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::Uart0)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AmClkEnR {
        AmClkEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<UartclkgrSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<UartclkgrSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, UARTCLKGS.CLK_EN and UARTCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn am_clk_en(&mut self) -> AmClkEnW<UartclkgrSpec> {
        AmClkEnW::new(self, 8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<UartclkgrSpec> {
        Reserved12W::new(self, 12)
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
