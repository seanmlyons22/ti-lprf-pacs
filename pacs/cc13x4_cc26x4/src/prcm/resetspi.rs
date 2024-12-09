#[doc = "Register `RESETSPI` reader"]
pub type R = crate::R<ResetspiSpec>;
#[doc = "Register `RESETSPI` writer"]
pub type W = crate::W<ResetspiSpec>;
#[doc = "Field `SPI0` writer - 0:0\\]
0: No action 1: Reset SPI0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` writer - 1:1\\]
0: No action 1: Reset SPI1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` writer - 2:2\\]
0: No action 1: Reset SPI2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` writer - 3:3\\]
0: No action 1: Reset SPI3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
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
0: No action 1: Reset SPI0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> Spi0W<ResetspiSpec> {
        Spi0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No action 1: Reset SPI1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> Spi1W<ResetspiSpec> {
        Spi1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: Reset SPI2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> Spi2W<ResetspiSpec> {
        Spi2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
0: No action 1: Reset SPI3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> Spi3W<ResetspiSpec> {
        Spi3W::new(self, 3)
    }
}
#[doc = "RESET For SPI IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetspi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetspi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetspiSpec;
impl crate::RegisterSpec for ResetspiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetspi::R`](R) reader structure"]
impl crate::Readable for ResetspiSpec {}
#[doc = "`write(|w| ..)` method takes [`resetspi::W`](W) writer structure"]
impl crate::Writable for ResetspiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETSPI to value 0"]
impl crate::Resettable for ResetspiSpec {
    const RESET_VALUE: u32 = 0;
}
