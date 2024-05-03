#[doc = "Register `UARTCLKGDS` reader"]
pub type R = crate::R<UartclkgdsSpec>;
#[doc = "Register `UARTCLKGDS` writer"]
pub type W = crate::W<UartclkgdsSpec>;
#[doc = "1:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEn {
    #[doc = "1: Enable clock for UART0"]
    AmUart0 = 1,
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
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
pub type ClkEnR = crate::FieldReader<ClkEn>;
impl ClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEn> {
        match self.bits {
            1 => Some(ClkEn::AmUart0),
            _ => None,
        }
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn is_am_uart0(&self) -> bool {
        *self == ClkEn::AmUart0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkEn>;
impl<'a, REG> ClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn am_uart0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::AmUart0)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<UartclkgdsSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<UartclkgdsSpec> {
        Reserved2W::new(self, 2)
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
