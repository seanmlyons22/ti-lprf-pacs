#[doc = "Register `CPUIRQSEL21` reader"]
pub type R = crate::R<Cpuirqsel21Spec>;
#[doc = "Register `CPUIRQSEL21` writer"]
pub type W = crate::W<Cpuirqsel21Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "14: GPT3A interrupt event, controlled by GPT3:TAMR"]
    Gpt3a = 14,
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
            14 => Some(Ev::Gpt3a),
            _ => None,
        }
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn is_gpt3a(&self) -> bool {
        *self == Ev::Gpt3a
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
#[doc = "Output Selection for CPU Interrupt 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel21Spec;
impl crate::RegisterSpec for Cpuirqsel21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel21::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel21Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel21::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL21 to value 0x0e"]
impl crate::Resettable for Cpuirqsel21Spec {
    const RESET_VALUE: u32 = 0x0e;
}
