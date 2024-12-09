#[doc = "Register `CPUIRQSEL19` reader"]
pub type R = crate::R<Cpuirqsel19Spec>;
#[doc = "Register `CPUIRQSEL19` writer"]
pub type W = crate::W<Cpuirqsel19Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "12: GPT2A interrupt event, controlled by GPT2:TAMR"]
    Gpt2a = 12,
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
            12 => Some(Ev::Gpt2a),
            _ => None,
        }
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn is_gpt2a(&self) -> bool {
        *self == Ev::Gpt2a
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
impl W {}
#[doc = "Output Selection for CPU Interrupt 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel19Spec;
impl crate::RegisterSpec for Cpuirqsel19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel19::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel19Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel19::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL19 to value 0x0c"]
impl crate::Resettable for Cpuirqsel19Spec {
    const RESET_VALUE: u32 = 0x0c;
}
