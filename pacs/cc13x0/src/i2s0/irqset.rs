#[doc = "Register `IRQSET` reader"]
pub type R = crate::R<IrqsetSpec>;
#[doc = "Register `IRQSET` writer"]
pub type W = crate::W<IrqsetSpec>;
#[doc = "Field `PTR_ERR` reader - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
pub type PtrErrR = crate::BitReader;
#[doc = "Field `PTR_ERR` writer - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
pub type PtrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_ERR` reader - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
pub type WclkErrR = crate::BitReader;
#[doc = "Field `WCLK_ERR` writer - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
pub type WclkErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERR` reader - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
pub type BusErrR = crate::BitReader;
#[doc = "Field `BUS_ERR` writer - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
pub type BusErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_TIMEOUT` reader - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
pub type WclkTimeoutR = crate::BitReader;
#[doc = "Field `WCLK_TIMEOUT` writer - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
pub type WclkTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIF_DMA_OUT` reader - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AifDmaOutR = crate::BitReader;
#[doc = "Field `AIF_DMA_OUT` writer - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AifDmaOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIF_DMA_IN` reader - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AifDmaInR = crate::BitReader;
#[doc = "Field `AIF_DMA_IN` writer - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
pub type AifDmaInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
    #[inline(always)]
    pub fn ptr_err(&self) -> PtrErrR {
        PtrErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
    #[inline(always)]
    pub fn wclk_err(&self) -> WclkErrR {
        WclkErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
    #[inline(always)]
    pub fn bus_err(&self) -> BusErrR {
        BusErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
    #[inline(always)]
    pub fn wclk_timeout(&self) -> WclkTimeoutR {
        WclkTimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_out(&self) -> AifDmaOutR {
        AifDmaOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
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
1: Sets the interrupt of IRQFLAGS.PTR_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn ptr_err(&mut self) -> PtrErrW<IrqsetSpec> {
        PtrErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Sets the interrupt of IRQFLAGS.WCLK_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_err(&mut self) -> WclkErrW<IrqsetSpec> {
        WclkErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Sets the interrupt of IRQFLAGS.BUS_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn bus_err(&mut self) -> BusErrW<IrqsetSpec> {
        BusErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_timeout(&mut self) -> WclkTimeoutW<IrqsetSpec> {
        WclkTimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_out(&mut self) -> AifDmaOutW<IrqsetSpec> {
        AifDmaOutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_in(&mut self) -> AifDmaInW<IrqsetSpec> {
        AifDmaInW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<IrqsetSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
