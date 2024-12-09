#[doc = "Register `ACCRESET` reader"]
pub type R = crate::R<AccresetSpec>;
#[doc = "Register `ACCRESET` writer"]
pub type W = crate::W<AccresetSpec>;
#[doc = "Field `TRG` writer - 15:0\\]
Write any value to this register to trigger a reset of all bits in the accumulator."]
pub type TrgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Write any value to this register to trigger a reset of all bits in the accumulator."]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TrgW<AccresetSpec> {
        TrgW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<AccresetSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Accumulator Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`accreset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`accreset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccresetSpec;
impl crate::RegisterSpec for AccresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accreset::R`](R) reader structure"]
impl crate::Readable for AccresetSpec {}
#[doc = "`write(|w| ..)` method takes [`accreset::W`](W) writer structure"]
impl crate::Writable for AccresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACCRESET to value 0"]
impl crate::Resettable for AccresetSpec {
    const RESET_VALUE: u32 = 0;
}
