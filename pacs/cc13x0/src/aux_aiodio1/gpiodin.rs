#[doc = "Register `GPIODIN` reader"]
pub type R = crate::R<GpiodinSpec>;
#[doc = "Register `GPIODIN` writer"]
pub type W = crate::W<GpiodinSpec>;
#[doc = "Field `IO7_0` reader - 7:0\\]
Bit n in this bit vector contains the value for AUXIO\\[8i+n\\]
when GPIODIE bit n is set. Otherwise, bit n value is old."]
pub type Io7_0R = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bit n in this bit vector contains the value for AUXIO\\[8i+n\\]
when GPIODIE bit n is set. Otherwise, bit n value is old."]
    #[inline(always)]
    pub fn io7_0(&self) -> Io7_0R {
        Io7_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiodinSpec;
impl crate::RegisterSpec for GpiodinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiodin::R`](R) reader structure"]
impl crate::Readable for GpiodinSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiodin::W`](W) writer structure"]
impl crate::Writable for GpiodinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIODIN to value 0"]
impl crate::Resettable for GpiodinSpec {
    const RESET_VALUE: u32 = 0;
}
