#[doc = "Register `COMP1` reader"]
pub type R = crate::R<Comp1Spec>;
#[doc = "Register `COMP1` writer"]
pub type W = crate::W<Comp1Spec>;
#[doc = "Field `COMP` reader - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION1. Comparator 1 can also compare data values. So this register can contain reference values for data matching."]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION1. Comparator 1 can also compare data values. So this register can contain reference values for data matching."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION1. Comparator 1 can also compare data values. So this register can contain reference values for data matching."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION1. Comparator 1 can also compare data values. So this register can contain reference values for data matching."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<Comp1Spec> {
        CompW::new(self, 0)
    }
}
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1Spec;
impl crate::RegisterSpec for Comp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1::R`](R) reader structure"]
impl crate::Readable for Comp1Spec {}
#[doc = "`write(|w| ..)` method takes [`comp1::W`](W) writer structure"]
impl crate::Writable for Comp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1 to value 0"]
impl crate::Resettable for Comp1Spec {
    const RESET_VALUE: u32 = 0;
}
