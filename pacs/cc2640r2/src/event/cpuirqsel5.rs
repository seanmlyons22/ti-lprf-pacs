#[doc = "Register `CPUIRQSEL5` reader"]
pub type R = crate::R<Cpuirqsel5Spec>;
#[doc = "Register `CPUIRQSEL5` writer"]
pub type W = crate::W<Cpuirqsel5Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 36"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "36: UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    Uart0Comb = 36,
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
            36 => Some(Ev::Uart0Comb),
            _ => None,
        }
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline(always)]
    pub fn is_uart0_comb(&self) -> bool {
        *self == Ev::Uart0Comb
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
#[doc = "Output Selection for CPU Interrupt 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel5Spec;
impl crate::RegisterSpec for Cpuirqsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel5::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel5::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL5 to value 0x24"]
impl crate::Resettable for Cpuirqsel5Spec {
    const RESET_VALUE: u32 = 0x24;
}
