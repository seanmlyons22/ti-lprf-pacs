#[doc = "Register `CPUIRQSEL29` reader"]
pub type R = crate::R<Cpuirqsel29Spec>;
#[doc = "Register `CPUIRQSEL29` writer"]
pub type W = crate::W<Cpuirqsel29Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "1: AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    AonProg0 = 1,
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
            1 => Some(Ev::AonProg0),
            _ => None,
        }
    }
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    #[inline(always)]
    pub fn is_aon_prog0(&self) -> bool {
        *self == Ev::AonProg0
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
#[doc = "Output Selection for CPU Interrupt 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel29Spec;
impl crate::RegisterSpec for Cpuirqsel29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel29::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel29Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel29::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL29 to value 0x01"]
impl crate::Resettable for Cpuirqsel29Spec {
    const RESET_VALUE: u32 = 0x01;
}
