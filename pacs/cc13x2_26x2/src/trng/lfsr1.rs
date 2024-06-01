#[doc = "Register `LFSR1` reader"]
pub type R = crate::R<Lfsr1Spec>;
#[doc = "Register `LFSR1` writer"]
pub type W = crate::W<Lfsr1Spec>;
#[doc = "Field `LFSR_63_32` reader - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type Lfsr63_32R = crate::FieldReader<u32>;
#[doc = "Field `LFSR_63_32` writer - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
pub type Lfsr63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub fn lfsr_63_32(&self) -> Lfsr63_32R {
        Lfsr63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits \\[63:32\\]
of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_63_32(&mut self) -> Lfsr63_32W<Lfsr1Spec> {
        Lfsr63_32W::new(self, 0)
    }
}
#[doc = "LFSR Readout Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfsr1Spec;
impl crate::RegisterSpec for Lfsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsr1::R`](R) reader structure"]
impl crate::Readable for Lfsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lfsr1::W`](W) writer structure"]
impl crate::Writable for Lfsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFSR1 to value 0"]
impl crate::Resettable for Lfsr1Spec {
    const RESET_VALUE: u32 = 0;
}
