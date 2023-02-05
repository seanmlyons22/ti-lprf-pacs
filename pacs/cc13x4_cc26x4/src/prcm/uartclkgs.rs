#[doc = "Register `UARTCLKGS` reader"]
pub struct R(crate::R<UARTCLKGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTCLKGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTCLKGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTCLKGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTCLKGS` writer"]
pub struct W(crate::W<UARTCLKGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTCLKGS_SPEC>;
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
impl From<crate::W<UARTCLKGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTCLKGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_R = crate::FieldReader<u8, CLK_EN_A>;
#[doc = "3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "8: Enable clock for UART3"]
    UART3 = 8,
    #[doc = "4: Enable clock for UART2"]
    UART2 = 4,
    #[doc = "2: Enable clock for UART1"]
    UART1 = 2,
    #[doc = "1: Enable clock for UART0"]
    UART0 = 1,
}
impl From<CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_EN_A) -> Self {
        variant as _
    }
}
impl CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_EN_A> {
        match self.bits {
            8 => Some(CLK_EN_A::UART3),
            4 => Some(CLK_EN_A::UART2),
            2 => Some(CLK_EN_A::UART1),
            1 => Some(CLK_EN_A::UART0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART3`"]
    #[inline(always)]
    pub fn is_uart3(&self) -> bool {
        *self == CLK_EN_A::UART3
    }
    #[doc = "Checks if the value of the field is `UART2`"]
    #[inline(always)]
    pub fn is_uart2(&self) -> bool {
        *self == CLK_EN_A::UART2
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == CLK_EN_A::UART1
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == CLK_EN_A::UART0
    }
}
#[doc = "Field `CLK_EN` writer - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UARTCLKGS_SPEC, u8, CLK_EN_A, 4, O>;
impl<'a, const O: u8> CLK_EN_W<'a, O> {
    #[doc = "Enable clock for UART3"]
    #[inline(always)]
    pub fn uart3(self) -> &'a mut W {
        self.variant(CLK_EN_A::UART3)
    }
    #[doc = "Enable clock for UART2"]
    #[inline(always)]
    pub fn uart2(self) -> &'a mut W {
        self.variant(CLK_EN_A::UART2)
    }
    #[doc = "Enable clock for UART1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut W {
        self.variant(CLK_EN_A::UART1)
    }
    #[doc = "Enable clock for UART0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut W {
        self.variant(CLK_EN_A::UART0)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UARTCLKGS_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by UARTCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartclkgs](index.html) module"]
pub struct UARTCLKGS_SPEC;
impl crate::RegisterSpec for UARTCLKGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartclkgs::R](R) reader structure"]
impl crate::Readable for UARTCLKGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartclkgs::W](W) writer structure"]
impl crate::Writable for UARTCLKGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTCLKGS to value 0"]
impl crate::Resettable for UARTCLKGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
