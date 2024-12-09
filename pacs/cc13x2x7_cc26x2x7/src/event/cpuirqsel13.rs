#[doc = "Register `CPUIRQSEL13` reader"]
pub type R = crate::R<Cpuirqsel13Spec>;
#[doc = "Register `CPUIRQSEL13` writer"]
pub type W = crate::W<Cpuirqsel13Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 29"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "29: AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    AuxSwev1 = 29,
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
            29 => Some(Ev::AuxSwev1),
            _ => None,
        }
    }
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn is_aux_swev1(&self) -> bool {
        *self == Ev::AuxSwev1
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
#[doc = "Output Selection for CPU Interrupt 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel13Spec;
impl crate::RegisterSpec for Cpuirqsel13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel13::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel13Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel13::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL13 to value 0x1d"]
impl crate::Resettable for Cpuirqsel13Spec {
    const RESET_VALUE: u32 = 0x1d;
}
