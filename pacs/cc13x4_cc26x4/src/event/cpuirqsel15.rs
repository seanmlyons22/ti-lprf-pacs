#[doc = "Register `CPUIRQSEL15` reader"]
pub type R = crate::R<Cpuirqsel15Spec>;
#[doc = "Register `CPUIRQSEL15` writer"]
pub type W = crate::W<Cpuirqsel15Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "16: GPT0A interrupt event, controlled by GPT0:TAMR"]
    Gpt0a = 16,
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
            16 => Some(Ev::Gpt0a),
            _ => None,
        }
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn is_gpt0a(&self) -> bool {
        *self == Ev::Gpt0a
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
#[doc = "Output Selection for CPU Interrupt 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel15Spec;
impl crate::RegisterSpec for Cpuirqsel15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel15::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel15Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel15::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL15 to value 0x10"]
impl crate::Resettable for Cpuirqsel15Spec {
    const RESET_VALUE: u32 = 0x10;
}
