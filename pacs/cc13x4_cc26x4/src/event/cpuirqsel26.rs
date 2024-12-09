#[doc = "Register `CPUIRQSEL26` reader"]
pub type R = crate::R<Cpuirqsel26Spec>;
#[doc = "Register `CPUIRQSEL26` writer"]
pub type W = crate::W<Cpuirqsel26Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 21"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "21: FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    Flash = 21,
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
            21 => Some(Ev::Flash),
            _ => None,
        }
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == Ev::Flash
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
#[doc = "Output Selection for CPU Interrupt 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel26Spec;
impl crate::RegisterSpec for Cpuirqsel26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel26::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel26Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel26::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL26 to value 0x15"]
impl crate::Resettable for Cpuirqsel26Spec {
    const RESET_VALUE: u32 = 0x15;
}
