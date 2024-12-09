#[doc = "Register `ISER1` reader"]
pub type R = crate::R<Iser1Spec>;
#[doc = "Register `ISER1` writer"]
pub type W = crate::W<Iser1Spec>;
#[doc = "Field `SETENA` reader - 15:0\\]
For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
pub type SetenaR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
For SETENA\\[m\\]
in NVIC_ISER*n, indicates whether interrupt 32*n + m is enabled"]
    #[inline(always)]
    pub fn setena(&self) -> SetenaR {
        SetenaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Enables or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iser1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iser1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iser1Spec;
impl crate::RegisterSpec for Iser1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iser1::R`](R) reader structure"]
impl crate::Readable for Iser1Spec {}
#[doc = "`write(|w| ..)` method takes [`iser1::W`](W) writer structure"]
impl crate::Writable for Iser1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISER1 to value 0"]
impl crate::Resettable for Iser1Spec {
    const RESET_VALUE: u32 = 0;
}
