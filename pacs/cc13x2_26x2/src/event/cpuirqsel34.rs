#[doc = "Register `CPUIRQSEL34` reader"]
pub type R = crate::R<Cpuirqsel34Spec>;
#[doc = "Register `CPUIRQSEL34` writer"]
pub type W = crate::W<Cpuirqsel34Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "6: Combined event from Oscillator control"]
    OscComb = 6,
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
            6 => Some(Ev::OscComb),
            _ => None,
        }
    }
    #[doc = "Combined event from Oscillator control"]
    #[inline(always)]
    pub fn is_osc_comb(&self) -> bool {
        *self == Ev::OscComb
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
#[doc = "Output Selection for CPU Interrupt 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel34Spec;
impl crate::RegisterSpec for Cpuirqsel34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel34::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel34Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel34::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL34 to value 0x06"]
impl crate::Resettable for Cpuirqsel34Spec {
    const RESET_VALUE: u32 = 0x06;
}
