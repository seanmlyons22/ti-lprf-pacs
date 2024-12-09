#[doc = "Register `RESETI2C` reader"]
pub type R = crate::R<Reseti2cSpec>;
#[doc = "Register `RESETI2C` writer"]
pub type W = crate::W<Reseti2cSpec>;
#[doc = "Field `I2C` writer - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type I2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1R = crate::FieldReader<u32>;
#[doc = "Field `SPARE1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> Spare1R {
        Spare1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<Reseti2cSpec> {
        I2cW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> Spare1W<Reseti2cSpec> {
        Spare1W::new(self, 1)
    }
}
#[doc = "RESET For I2C IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reseti2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reseti2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reseti2cSpec;
impl crate::RegisterSpec for Reseti2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reseti2c::R`](R) reader structure"]
impl crate::Readable for Reseti2cSpec {}
#[doc = "`write(|w| ..)` method takes [`reseti2c::W`](W) writer structure"]
impl crate::Writable for Reseti2cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETI2C to value 0"]
impl crate::Resettable for Reseti2cSpec {
    const RESET_VALUE: u32 = 0;
}
