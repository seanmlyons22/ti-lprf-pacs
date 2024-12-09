#[doc = "Register `IRQSET` reader"]
pub type R = crate::R<IrqsetSpec>;
#[doc = "Register `IRQSET` writer"]
pub type W = crate::W<IrqsetSpec>;
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If 1 is written to this bit, the result available (irq_result_av) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the result available (irq_result_av) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (IRQCLR.RESULT_AVAIL)."]
pub type ResultAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If 1 is written to this bit, the DMA data in done (irq_dma_in_done) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the DMA data in done (irq_dma_in_done) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (IRQCLR.DMA_IN_DONE)."]
pub type DmaInDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, the result available (irq_result_av) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the result available (irq_result_av) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (IRQCLR.RESULT_AVAIL)."]
    #[inline(always)]
    #[must_use]
    pub fn result_avail(&mut self) -> ResultAvailW<IrqsetSpec> {
        ResultAvailW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, the DMA data in done (irq_dma_in_done) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the DMA data in done (irq_dma_in_done) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (IRQCLR.DMA_IN_DONE)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DmaInDoneW<IrqsetSpec> {
        DmaInDoneW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IrqsetSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Control Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
