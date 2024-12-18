#[doc = "Register `IRQSTAT` reader"]
pub type R = crate::R<IrqstatSpec>;
#[doc = "Register `IRQSTAT` writer"]
pub type W = crate::W<IrqstatSpec>;
#[doc = "Field `RESULT_AVAIL` reader - 0:0\\]
This bit is set high when the Crypto peripheral has a result available."]
pub type ResultAvailR = crate::BitReader;
#[doc = "Field `DMA_IN_DONE` reader - 1:1\\]
This bit returns the status of DMA data in done interrupt."]
pub type DmaInDoneR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `KEY_ST_RD_ERR` reader - 29:29\\]
This bit will be set when a read error is detected during the read of a key from the key store, while copying it to the AES engine. The value of this register is held until it is cleared via IRQCLR.KEY_ST_RD_ERR. Note: This error is asserted if a key location is selected in the key store that is not available. Note: This is not an interrupt source."]
pub type KeyStRdErrR = crate::BitReader;
#[doc = "Field `KEY_ST_WR_ERR` reader - 30:30\\]
This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared via IRQCLR.KEY_ST_WR_ERR Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected. Note: This is not an interrupt source."]
pub type KeyStWrErrR = crate::BitReader;
#[doc = "Field `DMA_BUS_ERR` reader - 31:31\\]
This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared via IRQCLR.DMA_BUS_ERR Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation. Note: This is not an interrupt source."]
pub type DmaBusErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is set high when the Crypto peripheral has a result available."]
    #[inline(always)]
    pub fn result_avail(&self) -> ResultAvailR {
        ResultAvailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit returns the status of DMA data in done interrupt."]
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
This bit will be set when a read error is detected during the read of a key from the key store, while copying it to the AES engine. The value of this register is held until it is cleared via IRQCLR.KEY_ST_RD_ERR. Note: This error is asserted if a key location is selected in the key store that is not available. Note: This is not an interrupt source."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KeyStRdErrR {
        KeyStRdErrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared via IRQCLR.KEY_ST_WR_ERR Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected. Note: This is not an interrupt source."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KeyStWrErrR {
        KeyStWrErrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared via IRQCLR.DMA_BUS_ERR Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation. Note: This is not an interrupt source."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DmaBusErrR {
        DmaBusErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqstatSpec;
impl crate::RegisterSpec for IrqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqstat::R`](R) reader structure"]
impl crate::Readable for IrqstatSpec {}
#[doc = "`write(|w| ..)` method takes [`irqstat::W`](W) writer structure"]
impl crate::Writable for IrqstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSTAT to value 0"]
impl crate::Resettable for IrqstatSpec {
    const RESET_VALUE: u32 = 0;
}
