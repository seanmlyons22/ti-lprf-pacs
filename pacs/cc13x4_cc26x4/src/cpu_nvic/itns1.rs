#[doc = "Register `ITNS1` reader"]
pub type R = crate::R<Itns1Spec>;
#[doc = "Register `ITNS1` writer"]
pub type W = crate::W<Itns1Spec>;
#[doc = "Field `ITNS` reader - 15:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
pub type ItnsR = crate::FieldReader<u16>;
#[doc = "Field `ITNS` writer - 15:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
pub type ItnsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn itns(&self) -> ItnsR {
        ItnsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
For ITNS\\[m\\]
in NVIC_ITNS*n, the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn itns(&mut self) -> ItnsW<Itns1Spec> {
        ItnsW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Itns1Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itns1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itns1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itns1Spec;
impl crate::RegisterSpec for Itns1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itns1::R`](R) reader structure"]
impl crate::Readable for Itns1Spec {}
#[doc = "`write(|w| ..)` method takes [`itns1::W`](W) writer structure"]
impl crate::Writable for Itns1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITNS1 to value 0"]
impl crate::Resettable for Itns1Spec {
    const RESET_VALUE: u32 = 0;
}
