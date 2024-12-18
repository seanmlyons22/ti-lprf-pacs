#[doc = "Register `CPUIRQSEL37` reader"]
pub type R = crate::R<Cpuirqsel37Spec>;
#[doc = "Register `CPUIRQSEL37` writer"]
pub type W = crate::W<Cpuirqsel37Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "5: Combined event from battery monitor"]
    BatmonComb = 5,
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
            5 => Some(Ev::BatmonComb),
            _ => None,
        }
    }
    #[doc = "Combined event from battery monitor"]
    #[inline(always)]
    pub fn is_batmon_comb(&self) -> bool {
        *self == Ev::BatmonComb
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
#[doc = "Output Selection for CPU Interrupt 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel37Spec;
impl crate::RegisterSpec for Cpuirqsel37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel37::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel37Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel37::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL37 to value 0x05"]
impl crate::Resettable for Cpuirqsel37Spec {
    const RESET_VALUE: u32 = 0x05;
}
