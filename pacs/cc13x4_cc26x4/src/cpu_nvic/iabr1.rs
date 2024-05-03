#[doc = "Register `IABR1` reader"]
pub type R = crate::R<Iabr1Spec>;
#[doc = "Register `IABR1` writer"]
pub type W = crate::W<Iabr1Spec>;
#[doc = "Field `ACTIVE` reader - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ActiveR = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE` writer - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ActiveW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 0xffff) as u16)
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
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<Iabr1Spec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Iabr1Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iabr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iabr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iabr1Spec;
impl crate::RegisterSpec for Iabr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iabr1::R`](R) reader structure"]
impl crate::Readable for Iabr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iabr1::W`](W) writer structure"]
impl crate::Writable for Iabr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IABR1 to value 0"]
impl crate::Resettable for Iabr1Spec {
    const RESET_VALUE: u32 = 0;
}
