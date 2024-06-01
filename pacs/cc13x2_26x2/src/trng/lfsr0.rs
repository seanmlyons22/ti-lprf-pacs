#[doc = "Register `LFSR0` reader"]
pub type R = crate::R<Lfsr0Spec>;
#[doc = "Register `LFSR0` writer"]
pub type W = crate::W<Lfsr0Spec>;
#[doc = "Field `LFSR_31_0` reader - 31:0\\]
Bits \\[31:0\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type Lfsr31_0R = crate::FieldReader<u32>;
#[doc = "Field `LFSR_31_0` writer - 31:0\\]
Bits \\[31:0\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type Lfsr31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[31:0\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_31_0(&self) -> Lfsr31_0R {
        Lfsr31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[31:0\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_31_0(&mut self) -> Lfsr31_0W<Lfsr0Spec> {
        Lfsr31_0W::new(self, 0)
    }
}
#[doc = "LFSR Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfsr0Spec;
impl crate::RegisterSpec for Lfsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsr0::R`](R) reader structure"]
impl crate::Readable for Lfsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfsr0::W`](W) writer structure"]
impl crate::Writable for Lfsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFSR0 to value 0"]
impl crate::Resettable for Lfsr0Spec {
    const RESET_VALUE: u32 = 0;
}
