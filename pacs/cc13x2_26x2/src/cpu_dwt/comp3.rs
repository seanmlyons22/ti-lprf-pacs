#[doc = "Register `COMP3` reader"]
pub type R = crate::R<Comp3Spec>;
#[doc = "Register `COMP3` writer"]
pub type W = crate::W<Comp3Spec>;
#[doc = "Field `COMP` reader - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<Comp3Spec> {
        CompW::new(self, 0)
    }
}
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp3Spec;
impl crate::RegisterSpec for Comp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp3::R`](R) reader structure"]
impl crate::Readable for Comp3Spec {}
#[doc = "`write(|w| ..)` method takes [`comp3::W`](W) writer structure"]
impl crate::Writable for Comp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP3 to value 0"]
impl crate::Resettable for Comp3Spec {
    const RESET_VALUE: u32 = 0;
}
