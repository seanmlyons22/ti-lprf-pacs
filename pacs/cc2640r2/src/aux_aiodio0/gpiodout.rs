#[doc = "Register `GPIODOUT` reader"]
pub type R = crate::R<GpiodoutSpec>;
#[doc = "Register `GPIODOUT` writer"]
pub type W = crate::W<GpiodoutSpec>;
#[doc = "Field `IO7_0` reader - 7:0\\]
Write 1 to bit index n in this bit vector to set AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to clear AUXIO\\[8i+n\\]."]
pub type Io7_0R = crate::FieldReader;
#[doc = "Field `IO7_0` writer - 7:0\\]
Write 1 to bit index n in this bit vector to set AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to clear AUXIO\\[8i+n\\]."]
pub type Io7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write 1 to bit index n in this bit vector to set AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to clear AUXIO\\[8i+n\\]."]
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
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write 1 to bit index n in this bit vector to set AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to clear AUXIO\\[8i+n\\]."]
    #[inline(always)]
    #[must_use]
    pub fn io7_0(&mut self) -> Io7_0W<GpiodoutSpec> {
        Io7_0W::new(self, 0)
    }
}
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiodout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiodout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiodoutSpec;
impl crate::RegisterSpec for GpiodoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiodout::R`](R) reader structure"]
impl crate::Readable for GpiodoutSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiodout::W`](W) writer structure"]
impl crate::Writable for GpiodoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIODOUT to value 0"]
impl crate::Resettable for GpiodoutSpec {
    const RESET_VALUE: u32 = 0;
}
