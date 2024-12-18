#[doc = "Register `ID_MMFR2` reader"]
pub type R = crate::R<IdMmfr2Spec>;
#[doc = "Register `ID_MMFR2` writer"]
pub type W = crate::W<IdMmfr2Spec>;
#[doc = "Field `RESERVED0` reader - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `WAIT_FOR_INTERRUPT_STALLING` reader - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
pub type WaitForInterruptStallingR = crate::BitReader;
#[doc = "Field `RESERVED28` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&self) -> WaitForInterruptStallingR {
        WaitForInterruptStallingR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_mmfr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_mmfr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdMmfr2Spec;
impl crate::RegisterSpec for IdMmfr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_mmfr2::R`](R) reader structure"]
impl crate::Readable for IdMmfr2Spec {}
#[doc = "`write(|w| ..)` method takes [`id_mmfr2::W`](W) writer structure"]
impl crate::Writable for IdMmfr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_MMFR2 to value 0x0100_0000"]
impl crate::Resettable for IdMmfr2Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
