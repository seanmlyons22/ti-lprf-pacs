#[doc = "Register `IRQSET` reader"]
pub type R = crate::R<IrqsetSpec>;
#[doc = "Register `IRQSET` writer"]
pub type W = crate::W<IrqsetSpec>;
#[doc = "Field `RDY` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RdyR = crate::FieldReader<u32>;
#[doc = "Field `RDY` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RdyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RdyW<IrqsetSpec> {
        RdyW::new(self, 0)
    }
}
#[doc = "Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqsetSpec;
impl crate::RegisterSpec for IrqsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqset::R`](R) reader structure"]
impl crate::Readable for IrqsetSpec {}
#[doc = "`write(|w| ..)` method takes [`irqset::W`](W) writer structure"]
impl crate::Writable for IrqsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSET to value 0"]
impl crate::Resettable for IrqsetSpec {
    const RESET_VALUE: u32 = 0;
}
