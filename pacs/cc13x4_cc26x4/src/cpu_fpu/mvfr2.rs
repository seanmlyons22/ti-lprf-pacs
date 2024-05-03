#[doc = "Register `MVFR2` reader"]
pub type R = crate::R<Mvfr2Spec>;
#[doc = "Register `MVFR2` writer"]
pub type W = crate::W<Mvfr2Spec>;
#[doc = "Field `RESERVED0` reader - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPMisc` reader - 7:4\\]
Indicates support for miscellaneous FP features"]
pub type FpmiscR = crate::FieldReader;
#[doc = "Field `FPMisc` writer - 7:4\\]
Indicates support for miscellaneous FP features"]
pub type FpmiscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for miscellaneous FP features"]
    #[inline(always)]
    pub fn fpmisc(&self) -> FpmiscR {
        FpmiscR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Mvfr2Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates support for miscellaneous FP features"]
    #[inline(always)]
    #[must_use]
    pub fn fpmisc(&mut self) -> FpmiscW<Mvfr2Spec> {
        FpmiscW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Mvfr2Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Describes the features provided by the Floating-point Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mvfr2Spec;
impl crate::RegisterSpec for Mvfr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr2::R`](R) reader structure"]
impl crate::Readable for Mvfr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mvfr2::W`](W) writer structure"]
impl crate::Writable for Mvfr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MVFR2 to value 0"]
impl crate::Resettable for Mvfr2Spec {
    const RESET_VALUE: u32 = 0;
}
