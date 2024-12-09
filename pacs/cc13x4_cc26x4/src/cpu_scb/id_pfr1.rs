#[doc = "Register `ID_PFR1` reader"]
pub type R = crate::R<IdPfr1Spec>;
#[doc = "Register `ID_PFR1` writer"]
pub type W = crate::W<IdPfr1Spec>;
#[doc = "Field `RESERVED0` reader - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `SECURITY` reader - 7:4\\]
Security. Identifies whether the Security Extension is implemented"]
pub type SecurityR = crate::FieldReader;
#[doc = "Field `MICROCONTROLLER_PROGRAMMERS_MODEL` reader - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
pub type MicrocontrollerProgrammersModelR = crate::FieldReader;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Security. Identifies whether the Security Extension is implemented"]
    #[inline(always)]
    pub fn security(&self) -> SecurityR {
        SecurityR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&self) -> MicrocontrollerProgrammersModelR {
        MicrocontrollerProgrammersModelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "Processor Feature 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_pfr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_pfr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdPfr1Spec;
impl crate::RegisterSpec for IdPfr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_pfr1::R`](R) reader structure"]
impl crate::Readable for IdPfr1Spec {}
#[doc = "`write(|w| ..)` method takes [`id_pfr1::W`](W) writer structure"]
impl crate::Writable for IdPfr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_PFR1 to value 0x0210"]
impl crate::Resettable for IdPfr1Spec {
    const RESET_VALUE: u32 = 0x0210;
}
