#[doc = "Register `COMP6` reader"]
pub type R = crate::R<Comp6Spec>;
#[doc = "Register `COMP6` writer"]
pub type W = crate::W<Comp6Spec>;
#[doc = "Field `BE` reader - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<Comp6Spec> {
        BeW::new(self, 0)
    }
}
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp6Spec;
impl crate::RegisterSpec for Comp6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp6::R`](R) reader structure"]
impl crate::Readable for Comp6Spec {}
#[doc = "`write(|w| ..)` method takes [`comp6::W`](W) writer structure"]
impl crate::Writable for Comp6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP6 to value 0"]
impl crate::Resettable for Comp6Spec {
    const RESET_VALUE: u32 = 0;
}
