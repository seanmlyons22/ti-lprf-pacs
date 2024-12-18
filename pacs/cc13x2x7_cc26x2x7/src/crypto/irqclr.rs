#[doc = "Register `IRQCLR` reader"]
pub type R = crate::R<IrqclrSpec>;
#[doc = "Register `IRQCLR` writer"]
pub type W = crate::W<IrqclrSpec>;
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If 1 is written to this bit, the result available (irq_result_av) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
pub type ResultAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If 1 is written to this bit, the DMA in done (irq_dma_in_done) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
pub type DmaInDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` writer - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `KEY_ST_RD_ERR` writer - 29:29\\]
If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
pub type KeyStRdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ST_WR_ERR` writer - 30:30\\]
If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
pub type KeyStWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_BUS_ERR` writer - 31:31\\]
If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
pub type DmaBusErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, the result available (irq_result_av) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
    #[inline(always)]
    #[must_use]
    pub fn result_avail(&mut self) -> ResultAvailW<IrqclrSpec> {
        ResultAvailW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, the DMA in done (irq_dma_in_done) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DmaInDoneW<IrqclrSpec> {
        DmaInDoneW::new(self, 1)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IrqclrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 29 - 29:29\\]
If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_rd_err(&mut self) -> KeyStRdErrW<IrqclrSpec> {
        KeyStRdErrW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_wr_err(&mut self) -> KeyStWrErrW<IrqclrSpec> {
        KeyStWrErrW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn dma_bus_err(&mut self) -> DmaBusErrW<IrqclrSpec> {
        DmaBusErrW::new(self, 31)
    }
}
#[doc = "Control Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqclrSpec;
impl crate::RegisterSpec for IrqclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqclr::R`](R) reader structure"]
impl crate::Readable for IrqclrSpec {}
#[doc = "`write(|w| ..)` method takes [`irqclr::W`](W) writer structure"]
impl crate::Writable for IrqclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQCLR to value 0"]
impl crate::Resettable for IrqclrSpec {
    const RESET_VALUE: u32 = 0;
}
