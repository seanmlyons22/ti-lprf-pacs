#[doc = "Register `IRQEN` reader"]
pub type R = crate::R<IrqenSpec>;
#[doc = "Register `IRQEN` writer"]
pub type W = crate::W<IrqenSpec>;
#[doc = "Field `RESULT_AVAIL` reader - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
pub type ResultAvailR = crate::BitReader;
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
pub type ResultAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` reader - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
pub type DmaInDoneR = crate::BitReader;
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
pub type DmaInDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
    #[inline(always)]
    pub fn result_avail(&self) -> ResultAvailR {
        ResultAvailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DmaInDoneR {
        DmaInDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn result_avail(&mut self) -> ResultAvailW<IrqenSpec> {
        ResultAvailW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DmaInDoneW<IrqenSpec> {
        DmaInDoneW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IrqenSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Control Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqenSpec;
impl crate::RegisterSpec for IrqenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqen::R`](R) reader structure"]
impl crate::Readable for IrqenSpec {}
#[doc = "`write(|w| ..)` method takes [`irqen::W`](W) writer structure"]
impl crate::Writable for IrqenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQEN to value 0"]
impl crate::Resettable for IrqenSpec {
    const RESET_VALUE: u32 = 0;
}
