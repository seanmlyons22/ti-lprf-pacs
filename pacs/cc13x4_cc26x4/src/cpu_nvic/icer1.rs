#[doc = "Register `ICER1` reader"]
pub type R = crate::R<Icer1Spec>;
#[doc = "Register `ICER1` writer"]
pub type W = crate::W<Icer1Spec>;
#[doc = "Field `CLRENA` reader - 15:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
pub type ClrenaR = crate::FieldReader<u16>;
#[doc = "Field `CLRENA` writer - 15:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
pub type ClrenaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    pub fn clrena(&self) -> ClrenaR {
        ClrenaR::new((self.bits & 0xffff) as u16)
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
For CLRENA\\[m\\]
in NVIC_ICER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clrena(&mut self) -> ClrenaW<Icer1Spec> {
        ClrenaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Icer1Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Clears or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icer1Spec;
impl crate::RegisterSpec for Icer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icer1::R`](R) reader structure"]
impl crate::Readable for Icer1Spec {}
#[doc = "`write(|w| ..)` method takes [`icer1::W`](W) writer structure"]
impl crate::Writable for Icer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICER1 to value 0"]
impl crate::Resettable for Icer1Spec {
    const RESET_VALUE: u32 = 0;
}
