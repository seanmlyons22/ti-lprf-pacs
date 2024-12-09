#[doc = "Register `IRQFLAGS` reader"]
pub type R = crate::R<IrqflagsSpec>;
#[doc = "Register `IRQFLAGS` writer"]
pub type W = crate::W<IrqflagsSpec>;
#[doc = "Field `PTR_ERR` reader - 0:0\\]
Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR)."]
pub type PtrErrR = crate::BitReader;
#[doc = "Field `WCLK_ERR` reader - 1:1\\]
Set when: - An unexpected WCLK edge occurs during the data delay period of a phase. Note unexpected WCLK edges during the word and idle periods of the phase are not detected. - In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart. - In single-phase mode, when a WCLK pulse occurs before the last channel. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR)."]
pub type WclkErrR = crate::BitReader;
#[doc = "Field `BUS_ERR` reader - 2:2\\]
Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow). This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR). Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined."]
pub type BusErrR = crate::BitReader;
#[doc = "Field `WCLK_TIMEOUT` reader - 3:3\\]
Set when the sample stamp generator does not detect a positive WCLK edge for 65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT)."]
pub type WclkTimeoutR = crate::BitReader;
#[doc = "Field `AIF_DMA_OUT` reader - 4:4\\]
Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details"]
pub type AifDmaOutR = crate::BitReader;
#[doc = "Field `AIF_DMA_IN` reader - 5:5\\]
Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details."]
pub type AifDmaInR = crate::BitReader;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR)."]
    #[inline(always)]
    pub fn ptr_err(&self) -> PtrErrR {
        PtrErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set when: - An unexpected WCLK edge occurs during the data delay period of a phase. Note unexpected WCLK edges during the word and idle periods of the phase are not detected. - In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart. - In single-phase mode, when a WCLK pulse occurs before the last channel. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR)."]
    #[inline(always)]
    pub fn wclk_err(&self) -> WclkErrR {
        WclkErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow). This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR). Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined."]
    #[inline(always)]
    pub fn bus_err(&self) -> BusErrR {
        BusErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Set when the sample stamp generator does not detect a positive WCLK edge for 65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT)."]
    #[inline(always)]
    pub fn wclk_timeout(&self) -> WclkTimeoutR {
        WclkTimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details"]
    #[inline(always)]
    pub fn aif_dma_out(&self) -> AifDmaOutR {
        AifDmaOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details."]
    #[inline(always)]
    pub fn aif_dma_in(&self) -> AifDmaInR {
        AifDmaInR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqflagsSpec;
impl crate::RegisterSpec for IrqflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqflags::R`](R) reader structure"]
impl crate::Readable for IrqflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`irqflags::W`](W) writer structure"]
impl crate::Writable for IrqflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQFLAGS to value 0"]
impl crate::Resettable for IrqflagsSpec {
    const RESET_VALUE: u32 = 0;
}
