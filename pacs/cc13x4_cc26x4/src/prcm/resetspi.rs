#[doc = "Register `RESETSPI` reader"]
pub struct R(crate::R<RESETSPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETSPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETSPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETSPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETSPI` writer"]
pub struct W(crate::W<RESETSPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETSPI_SPEC>;
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
impl From<crate::W<RESETSPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETSPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0` reader - 0:0\\]
0: No action 1: Reset SPI0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI0_R = crate::BitReader<bool>;
#[doc = "Field `SPI0` writer - 0:0\\]
0: No action 1: Reset SPI0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSPI_SPEC, bool, O>;
#[doc = "Field `SPI1` reader - 1:1\\]
0: No action 1: Reset SPI1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1` writer - 1:1\\]
0: No action 1: Reset SPI1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSPI_SPEC, bool, O>;
#[doc = "Field `SPI2` reader - 2:2\\]
0: No action 1: Reset SPI2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI2_R = crate::BitReader<bool>;
#[doc = "Field `SPI2` writer - 2:2\\]
0: No action 1: Reset SPI2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSPI_SPEC, bool, O>;
#[doc = "Field `SPI3` reader - 3:3\\]
0: No action 1: Reset SPI3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI3_R = crate::BitReader<bool>;
#[doc = "Field `SPI3` writer - 3:3\\]
0: No action 1: Reset SPI3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type SPI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETSPI_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETSPI_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset SPI0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No action 1: Reset SPI1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: Reset SPI2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: No action 1: Reset SPI3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset SPI0. HW cleared. Access will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<0> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No action 1: Reset SPI1. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<1> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: Reset SPI2. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<2> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: No action 1: Reset SPI3. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<3> {
        SPI3_W::new(self)
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
#[doc = "RESET For SPI IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetspi](index.html) module"]
pub struct RESETSPI_SPEC;
impl crate::RegisterSpec for RESETSPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetspi::R](R) reader structure"]
impl crate::Readable for RESETSPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetspi::W](W) writer structure"]
impl crate::Writable for RESETSPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETSPI to value 0"]
impl crate::Resettable for RESETSPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
