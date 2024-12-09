#[doc = "Register `CPUIRQSEL17` reader"]
pub type R = crate::R<Cpuirqsel17Spec>;
#[doc = "Register `CPUIRQSEL17` writer"]
pub type W = crate::W<Cpuirqsel17Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "18: GPT1A interrupt event, controlled by GPT1:TAMR"]
    Gpt1a = 18,
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
            18 => Some(Ev::Gpt1a),
            _ => None,
        }
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn is_gpt1a(&self) -> bool {
        *self == Ev::Gpt1a
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
#[doc = "Output Selection for CPU Interrupt 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel17Spec;
impl crate::RegisterSpec for Cpuirqsel17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel17::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel17Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel17::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL17 to value 0x12"]
impl crate::Resettable for Cpuirqsel17Spec {
    const RESET_VALUE: u32 = 0x12;
}
