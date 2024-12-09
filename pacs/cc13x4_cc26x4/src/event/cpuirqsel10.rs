#[doc = "Register `CPUIRQSEL10` reader"]
pub type R = crate::R<Cpuirqsel10Spec>;
#[doc = "Register `CPUIRQSEL10` writer"]
pub type W = crate::W<Cpuirqsel10Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 26"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "26: Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RfcHwComb = 26,
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
            26 => Some(Ev::RfcHwComb),
            _ => None,
        }
    }
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline(always)]
    pub fn is_rfc_hw_comb(&self) -> bool {
        *self == Ev::RfcHwComb
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
#[doc = "Output Selection for CPU Interrupt 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel10Spec;
impl crate::RegisterSpec for Cpuirqsel10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel10::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel10Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel10::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL10 to value 0x1a"]
impl crate::Resettable for Cpuirqsel10Spec {
    const RESET_VALUE: u32 = 0x1a;
}
