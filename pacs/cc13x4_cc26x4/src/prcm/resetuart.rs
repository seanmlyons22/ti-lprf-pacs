#[doc = "Register `RESETUART` reader"]
pub type R = crate::R<ResetuartSpec>;
#[doc = "Register `RESETUART` writer"]
pub type W = crate::W<ResetuartSpec>;
#[doc = "Field `UART0` writer - 0:0\\]
0: No action 1: Reset UART0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` writer - 1:1\\]
0: No action 1: Reset UART1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` writer - 2:2\\]
0: No action 1: Reset UART2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3` writer - 3:3\\]
0: No action 1: Reset UART3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Uart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset UART0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> Uart0W<ResetuartSpec> {
        Uart0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No action 1: Reset UART1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> Uart1W<ResetuartSpec> {
        Uart1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: Reset UART2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> Uart2W<ResetuartSpec> {
        Uart2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
0: No action 1: Reset UART3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> Uart3W<ResetuartSpec> {
        Uart3W::new(self, 3)
    }
}
#[doc = "RESET For UART IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetuart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetuart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetuartSpec;
impl crate::RegisterSpec for ResetuartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetuart::R`](R) reader structure"]
impl crate::Readable for ResetuartSpec {}
#[doc = "`write(|w| ..)` method takes [`resetuart::W`](W) writer structure"]
impl crate::Writable for ResetuartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETUART to value 0"]
impl crate::Resettable for ResetuartSpec {
    const RESET_VALUE: u32 = 0;
}
