#[doc = "Register `COMP0` reader"]
pub type R = crate::R<Comp0Spec>;
#[doc = "Register `COMP0` writer"]
pub type W = crate::W<Comp0Spec>;
#[doc = "Field `COMP` reader - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<Comp0Spec> {
        CompW::new(self, 0)
    }
}
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0Spec;
impl crate::RegisterSpec for Comp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0::R`](R) reader structure"]
impl crate::Readable for Comp0Spec {}
#[doc = "`write(|w| ..)` method takes [`comp0::W`](W) writer structure"]
impl crate::Writable for Comp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP0 to value 0"]
impl crate::Resettable for Comp0Spec {
    const RESET_VALUE: u32 = 0;
}
