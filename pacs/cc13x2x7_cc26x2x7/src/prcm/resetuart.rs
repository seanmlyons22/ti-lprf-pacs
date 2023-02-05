#[doc = "Register `RESETUART` reader"]
pub struct R(crate::R<RESETUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETUART` writer"]
pub struct W(crate::W<RESETUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETUART_SPEC>;
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
impl From<crate::W<RESETUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - 0:0\\]
0: No action 1: Reset UART0. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - 0:0\\]
0: No action 1: Reset UART0. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETUART_SPEC, bool, O>;
#[doc = "Field `UART1` reader - 1:1\\]
0: No action 1: Reset UART1. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - 1:1\\]
0: No action 1: Reset UART1. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETUART_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESETUART_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset UART0. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No action 1: Reset UART1. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset UART0. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<0> {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No action 1: Reset UART1. HW cleared. Acess will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<1> {
        UART1_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESET For UART IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetuart](index.html) module"]
pub struct RESETUART_SPEC;
impl crate::RegisterSpec for RESETUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetuart::R](R) reader structure"]
impl crate::Readable for RESETUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetuart::W](W) writer structure"]
impl crate::Writable for RESETUART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETUART to value 0"]
impl crate::Resettable for RESETUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
