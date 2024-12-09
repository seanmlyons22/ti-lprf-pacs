#[doc = "Register `IRQCLR` reader"]
pub type R = crate::R<IrqclrSpec>;
#[doc = "Register `IRQCLR` writer"]
pub type W = crate::W<IrqclrSpec>;
#[doc = "Field `PTR_ERR` writer - 0:0\\]
1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub type PtrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_ERR` writer - 1:1\\]
1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub type WclkErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERR` writer - 2:2\\]
1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub type BusErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_TIMEOUT` writer - 3:3\\]
1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub type WclkTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIF_DMA_OUT` writer - 4:4\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub type AifDmaOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIF_DMA_IN` writer - 5:5\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub type AifDmaInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn ptr_err(&mut self) -> PtrErrW<IrqclrSpec> {
        PtrErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_err(&mut self) -> WclkErrW<IrqclrSpec> {
        WclkErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn bus_err(&mut self) -> BusErrW<IrqclrSpec> {
        BusErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_timeout(&mut self) -> WclkTimeoutW<IrqclrSpec> {
        WclkTimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_out(&mut self) -> AifDmaOutW<IrqclrSpec> {
        AifDmaOutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn aif_dma_in(&mut self) -> AifDmaInW<IrqclrSpec> {
        AifDmaInW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<IrqclrSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
