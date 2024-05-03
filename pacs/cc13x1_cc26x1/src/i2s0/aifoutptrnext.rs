#[doc = "Register `AIFOUTPTRNEXT` reader"]
pub type R = crate::R<AifoutptrnextSpec>;
#[doc = "Register `AIFOUTPTRNEXT` writer"]
pub type W = crate::W<AifoutptrnextSpec>;
#[doc = "Field `PTR` reader - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<AifoutptrnextSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "DMA Output Buffer Next Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifoutptrnext::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifoutptrnext::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifoutptrnextSpec;
impl crate::RegisterSpec for AifoutptrnextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifoutptrnext::R`](R) reader structure"]
impl crate::Readable for AifoutptrnextSpec {}
#[doc = "`write(|w| ..)` method takes [`aifoutptrnext::W`](W) writer structure"]
impl crate::Writable for AifoutptrnextSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFOUTPTRNEXT to value 0"]
impl crate::Resettable for AifoutptrnextSpec {
    const RESET_VALUE: u32 = 0;
}
