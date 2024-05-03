#[doc = "Register `IRQMASK` reader"]
pub type R = crate::R<IrqmaskSpec>;
#[doc = "Register `IRQMASK` writer"]
pub type W = crate::W<IrqmaskSpec>;
#[doc = "Field `PTR_ERR` reader - 0:0\\]
IRQFLAGS.PTR_ERR interrupt mask. 0: Disable 1: Enable"]
pub type PtrErrR = crate::BitReader;
#[doc = "Field `PTR_ERR` writer - 0:0\\]
IRQFLAGS.PTR_ERR interrupt mask. 0: Disable 1: Enable"]
pub type PtrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_ERR` reader - 1:1\\]
IRQFLAGS.WCLK_ERR interrupt mask 0: Disable 1: Enable"]
pub type WclkErrR = crate::BitReader;
#[doc = "Field `WCLK_ERR` writer - 1:1\\]
IRQFLAGS.WCLK_ERR interrupt mask 0: Disable 1: Enable"]
pub type WclkErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERR` reader - 2:2\\]
IRQFLAGS.BUS_ERR interrupt mask 0: Disable 1: Enable"]
pub type BusErrR = crate::BitReader;
#[doc = "Field `BUS_ERR` writer - 2:2\\]
IRQFLAGS.BUS_ERR interrupt mask 0: Disable 1: Enable"]
pub type BusErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_TIMEOUT` reader - 3:3\\]
IRQFLAGS.WCLK_TIMEOUT interrupt mask 0: Disable 1: Enable"]
pub type WclkTimeoutR = crate::BitReader;
#[doc = "Field `WCLK_TIMEOUT` writer - 3:3\\]
IRQFLAGS.WCLK_TIMEOUT interrupt mask 0: Disable 1: Enable"]
pub type WclkTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIF_DMA_OUT` reader - 4:4\\]
IRQFLAGS.AIF_DMA_OUT interrupt mask 0: Disable 1: Enable"]
pub type AifDmaOutR = crate::BitReader;
#[doc = "Field `AIF_DMA_OUT` writer - 4:4\\]
IRQFLAGS.AIF_DMA_OUT interrupt mask 0: Disable 1: Enable"]
pub type AifDmaOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIF_DMA_IN` reader - 5:5\\]
IRQFLAGS.AIF_DMA_IN interrupt mask 0: Disable 1: Enable"]
pub type AifDmaInR = crate::BitReader;
#[doc = "Field `AIF_DMA_IN` writer - 5:5\\]
IRQFLAGS.AIF_DMA_IN interrupt mask 0: Disable 1: Enable"]
pub type AifDmaInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
IRQFLAGS.PTR_ERR interrupt mask. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn ptr_err(&self) -> PtrErrR {
        PtrErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
IRQFLAGS.WCLK_ERR interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn wclk_err(&self) -> WclkErrR {
        WclkErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
IRQFLAGS.BUS_ERR interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bus_err(&self) -> BusErrR {
        BusErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
IRQFLAGS.WCLK_TIMEOUT interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn wclk_timeout(&self) -> WclkTimeoutR {
        WclkTimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
IRQFLAGS.AIF_DMA_OUT interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn aif_dma_out(&self) -> AifDmaOutR {
        AifDmaOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
IRQFLAGS.AIF_DMA_IN interrupt mask 0: Disable 1: Enable"]
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
impl W {
    #[doc = "Bit 0 - 0:0\\]
IRQFLAGS.PTR_ERR interrupt mask. 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptr_err(&mut self) -> PtrErrW<IrqmaskSpec> {
        PtrErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
IRQFLAGS.WCLK_ERR interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_err(&mut self) -> WclkErrW<IrqmaskSpec> {
        WclkErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
IRQFLAGS.BUS_ERR interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bus_err(&mut self) -> BusErrW<IrqmaskSpec> {
        BusErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
IRQFLAGS.WCLK_TIMEOUT interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_timeout(&mut self) -> WclkTimeoutW<IrqmaskSpec> {
        WclkTimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
IRQFLAGS.AIF_DMA_OUT interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_out(&mut self) -> AifDmaOutW<IrqmaskSpec> {
        AifDmaOutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
IRQFLAGS.AIF_DMA_IN interrupt mask 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_in(&mut self) -> AifDmaInW<IrqmaskSpec> {
        AifDmaInW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<IrqmaskSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqmaskSpec;
impl crate::RegisterSpec for IrqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqmask::R`](R) reader structure"]
impl crate::Readable for IrqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`irqmask::W`](W) writer structure"]
impl crate::Writable for IrqmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQMASK to value 0"]
impl crate::Resettable for IrqmaskSpec {
    const RESET_VALUE: u32 = 0;
}
