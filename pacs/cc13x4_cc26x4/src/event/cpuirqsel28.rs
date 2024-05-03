#[doc = "Register `CPUIRQSEL28` reader"]
pub type R = crate::R<Cpuirqsel28Spec>;
#[doc = "Register `CPUIRQSEL28` writer"]
pub type W = crate::W<Cpuirqsel28Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "11: AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AuxComb = 11,
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
            11 => Some(Ev::AuxComb),
            _ => None,
        }
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn is_aux_comb(&self) -> bool {
        *self == Ev::AuxComb
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
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline(always)]
    pub fn aux_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxComb)
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
    pub fn ev(&mut self) -> EvW<Cpuirqsel28Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel28Spec;
impl crate::RegisterSpec for Cpuirqsel28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel28::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel28Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel28::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL28 to value 0x0b"]
impl crate::Resettable for Cpuirqsel28Spec {
    const RESET_VALUE: u32 = 0x0b;
}
