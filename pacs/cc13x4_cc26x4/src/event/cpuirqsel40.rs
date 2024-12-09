#[doc = "Register `CPUIRQSEL40` reader"]
pub type R = crate::R<Cpuirqsel40Spec>;
#[doc = "Register `CPUIRQSEL40` writer"]
pub type W = crate::W<Cpuirqsel40Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 124"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "124: UART2 combined interrupt, interrupt flags are found here UART2:MIS"]
    Uart2Comb = 124,
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
            124 => Some(Ev::Uart2Comb),
            _ => None,
        }
    }
    #[doc = "UART2 combined interrupt, interrupt flags are found here UART2:MIS"]
    #[inline(always)]
    pub fn is_uart2_comb(&self) -> bool {
        *self == Ev::Uart2Comb
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
#[doc = "Output Selection for CPU Interrupt 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel40Spec;
impl crate::RegisterSpec for Cpuirqsel40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel40::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel40Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel40::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL40 to value 0x7c"]
impl crate::Resettable for Cpuirqsel40Spec {
    const RESET_VALUE: u32 = 0x7c;
}
