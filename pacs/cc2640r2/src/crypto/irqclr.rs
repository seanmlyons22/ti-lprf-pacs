#[doc = "Register `IRQCLR` reader"]
pub type R = crate::R<IrqclrSpec>;
#[doc = "Register `IRQCLR` writer"]
pub type W = crate::W<IrqclrSpec>;
#[doc = "Field `RESULT_AVAIL` reader - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
pub type ResultAvailR = crate::BitReader;
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
pub type ResultAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` reader - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
pub type DmaInDoneR = crate::BitReader;
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
pub type DmaInDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `KEY_ST_RD_ERR` reader - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
pub type KeyStRdErrR = crate::BitReader;
#[doc = "Field `KEY_ST_RD_ERR` writer - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
pub type KeyStRdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ST_WR_ERR` reader - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
pub type KeyStWrErrR = crate::BitReader;
#[doc = "Field `KEY_ST_WR_ERR` writer - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
pub type KeyStWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_BUS_ERR` reader - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
pub type DmaBusErrR = crate::BitReader;
#[doc = "Field `DMA_BUS_ERR` writer - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
pub type DmaBusErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
    #[inline(always)]
    pub fn result_avail(&self) -> ResultAvailR {
        ResultAvailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DmaInDoneR {
        DmaInDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x07ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KeyStRdErrR {
        KeyStRdErrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KeyStWrErrR {
        KeyStWrErrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DmaBusErrR {
        DmaBusErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn result_avail(&mut self) -> ResultAvailW<IrqclrSpec> {
        ResultAvailW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
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
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_rd_err(&mut self) -> KeyStRdErrW<IrqclrSpec> {
        KeyStRdErrW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_wr_err(&mut self) -> KeyStWrErrW<IrqclrSpec> {
        KeyStWrErrW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn dma_bus_err(&mut self) -> DmaBusErrW<IrqclrSpec> {
        DmaBusErrW::new(self, 31)
    }
}
#[doc = "Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
