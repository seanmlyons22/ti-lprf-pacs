#[doc = "Register `ID_DFR0` reader"]
pub type R = crate::R<IdDfr0Spec>;
#[doc = "Register `ID_DFR0` writer"]
pub type W = crate::W<IdDfr0Spec>;
#[doc = "Field `RESERVED0` reader - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `MICROCONTROLLER_DEBUG_MODEL` reader - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
pub type MicrocontrollerDebugModelR = crate::FieldReader;
#[doc = "Field `MICROCONTROLLER_DEBUG_MODEL` writer - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
pub type MicrocontrollerDebugModelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
    #[inline(always)]
    pub fn microcontroller_debug_model(&self) -> MicrocontrollerDebugModelR {
        MicrocontrollerDebugModelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<IdDfr0Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
    #[inline(always)]
    #[must_use]
    pub fn microcontroller_debug_model(&mut self) -> MicrocontrollerDebugModelW<IdDfr0Spec> {
        MicrocontrollerDebugModelW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<IdDfr0Spec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_dfr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_dfr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdDfr0Spec;
impl crate::RegisterSpec for IdDfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_dfr0::R`](R) reader structure"]
impl crate::Readable for IdDfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`id_dfr0::W`](W) writer structure"]
impl crate::Writable for IdDfr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_DFR0 to value 0x0010_0000"]
impl crate::Resettable for IdDfr0Spec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
