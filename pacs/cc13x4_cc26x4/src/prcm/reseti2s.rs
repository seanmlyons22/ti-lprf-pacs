#[doc = "Register `RESETI2S` reader"]
pub type R = crate::R<Reseti2sSpec>;
#[doc = "Register `RESETI2S` writer"]
pub type W = crate::W<Reseti2sSpec>;
#[doc = "Field `I2S` writer - 0:0\\]
0: No action 1: Reset module. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type I2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset module. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2sW<Reseti2sSpec> {
        I2sW::new(self, 0)
    }
}
#[doc = "RESET For I2S IP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reseti2s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reseti2s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reseti2sSpec;
impl crate::RegisterSpec for Reseti2sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reseti2s::R`](R) reader structure"]
impl crate::Readable for Reseti2sSpec {}
#[doc = "`write(|w| ..)` method takes [`reseti2s::W`](W) writer structure"]
impl crate::Writable for Reseti2sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETI2S to value 0"]
impl crate::Resettable for Reseti2sSpec {
    const RESET_VALUE: u32 = 0;
}
