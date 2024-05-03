#[doc = "Register `AIFINPTRNEXT` reader"]
pub type R = crate::R<AifinptrnextSpec>;
#[doc = "Register `AIFINPTRNEXT` writer"]
pub type W = crate::W<AifinptrnextSpec>;
#[doc = "Field `PTR` reader - 31:0\\]
Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - 31:0\\]
Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<AifinptrnextSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "DMA Input Buffer Next Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifinptrnext::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifinptrnext::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifinptrnextSpec;
impl crate::RegisterSpec for AifinptrnextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifinptrnext::R`](R) reader structure"]
impl crate::Readable for AifinptrnextSpec {}
#[doc = "`write(|w| ..)` method takes [`aifinptrnext::W`](W) writer structure"]
impl crate::Writable for AifinptrnextSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFINPTRNEXT to value 0"]
impl crate::Resettable for AifinptrnextSpec {
    const RESET_VALUE: u32 = 0;
}
