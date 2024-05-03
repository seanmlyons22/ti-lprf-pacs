#[doc = "Register `LFSR2` reader"]
pub type R = crate::R<Lfsr2Spec>;
#[doc = "Register `LFSR2` writer"]
pub type W = crate::W<Lfsr2Spec>;
#[doc = "Field `LFSR_80_64` reader - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type Lfsr80_64R = crate::FieldReader<u32>;
#[doc = "Field `LFSR_80_64` writer - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type Lfsr80_64W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_80_64(&self) -> Lfsr80_64R {
        Lfsr80_64R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Bits \\[80:64\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_80_64(&mut self) -> Lfsr80_64W<Lfsr2Spec> {
        Lfsr80_64W::new(self, 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Lfsr2Spec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "LFSR Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfsr2Spec;
impl crate::RegisterSpec for Lfsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsr2::R`](R) reader structure"]
impl crate::Readable for Lfsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lfsr2::W`](W) writer structure"]
impl crate::Writable for Lfsr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFSR2 to value 0"]
impl crate::Resettable for Lfsr2Spec {
    const RESET_VALUE: u32 = 0;
}
