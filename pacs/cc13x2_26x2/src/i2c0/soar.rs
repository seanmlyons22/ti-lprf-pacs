#[doc = "Register `SOAR` reader"]
pub type R = crate::R<SoarSpec>;
#[doc = "Register `SOAR` writer"]
pub type W = crate::W<SoarSpec>;
#[doc = "Field `OAR` reader - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
pub type OarR = crate::FieldReader;
#[doc = "Field `OAR` writer - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
pub type OarW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    pub fn oar(&self) -> OarR {
        OarR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OarW<SoarSpec> {
        OarW::new(self, 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<SoarSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoarSpec;
impl crate::RegisterSpec for SoarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soar::R`](R) reader structure"]
impl crate::Readable for SoarSpec {}
#[doc = "`write(|w| ..)` method takes [`soar::W`](W) writer structure"]
impl crate::Writable for SoarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOAR to value 0"]
impl crate::Resettable for SoarSpec {
    const RESET_VALUE: u32 = 0;
}
