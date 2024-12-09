#[doc = "Register `CPUIRQSEL3` reader"]
pub type R = crate::R<Cpuirqsel3Spec>;
#[doc = "Register `CPUIRQSEL3` writer"]
pub type W = crate::W<Cpuirqsel3Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "31: Not used tied to 0"]
    TieLow31 = 31,
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
            31 => Some(Ev::TieLow31),
            _ => None,
        }
    }
    #[doc = "Not used tied to 0"]
    #[inline(always)]
    pub fn is_tie_low31(&self) -> bool {
        *self == Ev::TieLow31
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
#[doc = "Output Selection for CPU Interrupt 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel3Spec;
impl crate::RegisterSpec for Cpuirqsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel3::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel3::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL3 to value 0x1f"]
impl crate::Resettable for Cpuirqsel3Spec {
    const RESET_VALUE: u32 = 0x1f;
}
