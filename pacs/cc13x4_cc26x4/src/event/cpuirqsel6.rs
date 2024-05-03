#[doc = "Register `CPUIRQSEL6` reader"]
pub type R = crate::R<Cpuirqsel6Spec>;
#[doc = "Register `CPUIRQSEL6` writer"]
pub type W = crate::W<Cpuirqsel6Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 28"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "28: AUX software event 0, triggered by AUX_EVCTL:SWEVSET.SWEV0, also available as AUX_EVENT0 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    AuxSwev0 = 28,
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
            28 => Some(Ev::AuxSwev0),
            _ => None,
        }
    }
    #[doc = "AUX software event 0, triggered by AUX_EVCTL:SWEVSET.SWEV0, also available as AUX_EVENT0 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn is_aux_swev0(&self) -> bool {
        *self == Ev::AuxSwev0
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUX software event 0, triggered by AUX_EVCTL:SWEVSET.SWEV0, also available as AUX_EVENT0 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL"]
    #[inline(always)]
    pub fn aux_swev0(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxSwev0)
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
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Cpuirqsel6Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel6Spec;
impl crate::RegisterSpec for Cpuirqsel6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel6::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel6Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel6::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL6 to value 0x1c"]
impl crate::Resettable for Cpuirqsel6Spec {
    const RESET_VALUE: u32 = 0x1c;
}
