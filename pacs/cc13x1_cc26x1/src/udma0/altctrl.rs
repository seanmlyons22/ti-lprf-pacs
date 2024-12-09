#[doc = "Register `ALTCTRL` reader"]
pub type R = crate::R<AltctrlSpec>;
#[doc = "Register `ALTCTRL` writer"]
pub type W = crate::W<AltctrlSpec>;
#[doc = "Field `BASEPTR` reader - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
pub type BaseptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
    #[inline(always)]
    pub fn baseptr(&self) -> BaseptrR {
        BaseptrR::new(self.bits)
    }
}
impl W {}
#[doc = "Channel Alternate Control Data Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltctrlSpec;
impl crate::RegisterSpec for AltctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altctrl::R`](R) reader structure"]
impl crate::Readable for AltctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`altctrl::W`](W) writer structure"]
impl crate::Writable for AltctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALTCTRL to value 0x0200"]
impl crate::Resettable for AltctrlSpec {
    const RESET_VALUE: u32 = 0x0200;
}
