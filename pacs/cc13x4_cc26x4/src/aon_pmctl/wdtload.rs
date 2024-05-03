#[doc = "Register `WDTLOAD` reader"]
pub type R = crate::R<WdtloadSpec>;
#[doc = "Register `WDTLOAD` writer"]
pub type W = crate::W<WdtloadSpec>;
#[doc = "Field `LOAD` reader - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
pub type LoadR = crate::FieldReader<u32>;
#[doc = "Field `LOAD` writer - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, a reset is immediately generated. Read from this register will return the current value of the counter"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<WdtloadSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "Configuration Load Value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtload::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtload::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtloadSpec;
impl crate::RegisterSpec for WdtloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtload::R`](R) reader structure"]
impl crate::Readable for WdtloadSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtload::W`](W) writer structure"]
impl crate::Writable for WdtloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTLOAD to value 0"]
impl crate::Resettable for WdtloadSpec {
    const RESET_VALUE: u32 = 0;
}
