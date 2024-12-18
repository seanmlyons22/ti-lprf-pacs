#[doc = "Register `CPUIRQSEL35` reader"]
pub type R = crate::R<Cpuirqsel35Spec>;
#[doc = "Register `CPUIRQSEL35` writer"]
pub type W = crate::W<Cpuirqsel35Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 56"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "56: Not used tied to 0"]
    TieLow56 = 56,
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
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            56 => Some(Ev::TieLow56),
            _ => None,
        }
    }
    #[doc = "Not used tied to 0"]
    #[inline(always)]
    pub fn is_tie_low56(&self) -> bool {
        *self == Ev::TieLow56
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Output Selection for CPU Interrupt 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel35Spec;
impl crate::RegisterSpec for Cpuirqsel35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel35::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel35Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel35::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL35 to value 0x38"]
impl crate::Resettable for Cpuirqsel35Spec {
    const RESET_VALUE: u32 = 0x38;
}
