#[doc = "Register `CPUIRQSEL2` reader"]
pub type R = crate::R<Cpuirqsel2Spec>;
#[doc = "Register `CPUIRQSEL2` writer"]
pub type W = crate::W<Cpuirqsel2Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 30"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "30: Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RfcCpe1 = 30,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            30 => Some(Ev::RfcCpe1),
            _ => None,
        }
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    #[inline(always)]
    pub fn is_rfc_cpe_1(&self) -> bool {
        *self == Ev::RfcCpe1
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Output Selection for CPU Interrupt 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel2Spec;
impl crate::RegisterSpec for Cpuirqsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel2::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel2::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL2 to value 0x1e"]
impl crate::Resettable for Cpuirqsel2Spec {
    const RESET_VALUE: u32 = 0x1e;
}
