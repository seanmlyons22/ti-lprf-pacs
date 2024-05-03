#[doc = "Register `UARTCLKGDS` reader"]
pub type R = crate::R<UartclkgdsSpec>;
#[doc = "Register `UARTCLKGDS` writer"]
pub type W = crate::W<UartclkgdsSpec>;
#[doc = "3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
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
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<UartclkgdsSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<UartclkgdsSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "UART Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkgds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkgds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartclkgdsSpec;
impl crate::RegisterSpec for UartclkgdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartclkgds::R`](R) reader structure"]
impl crate::Readable for UartclkgdsSpec {}
#[doc = "`write(|w| ..)` method takes [`uartclkgds::W`](W) writer structure"]
impl crate::Writable for UartclkgdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTCLKGDS to value 0"]
impl crate::Resettable for UartclkgdsSpec {
    const RESET_VALUE: u32 = 0;
}
