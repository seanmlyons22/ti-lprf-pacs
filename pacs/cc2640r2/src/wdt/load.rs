#[doc = "Register `LOAD` reader"]
pub type R = crate::R<LoadSpec>;
#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LoadSpec>;
#[doc = "Field `WDTLOAD` reader - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
pub type WdtloadR = crate::FieldReader<u32>;
#[doc = "Field `WDTLOAD` writer - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
pub type WdtloadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
    #[inline(always)]
    pub fn wdtload(&self) -> WdtloadR {
        WdtloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
    #[inline(always)]
    #[must_use]
    pub fn wdtload(&mut self) -> WdtloadW<LoadSpec> {
        WdtloadW::new(self, 0)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadSpec;
impl crate::RegisterSpec for LoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load::R`](R) reader structure"]
impl crate::Readable for LoadSpec {}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LoadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOAD to value 0xffff_ffff"]
impl crate::Resettable for LoadSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
