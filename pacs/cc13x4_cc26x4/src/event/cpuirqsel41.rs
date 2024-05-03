#[doc = "Register `CPUIRQSEL41` reader"]
pub type R = crate::R<Cpuirqsel41Spec>;
#[doc = "Register `CPUIRQSEL41` writer"]
pub type W = crate::W<Cpuirqsel41Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 125"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "125: UART3 combined interrupt, interrupt flags are found here UART3:MIS"]
    Uart3Comb = 125,
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
            125 => Some(Ev::Uart3Comb),
            _ => None,
        }
    }
    #[doc = "UART3 combined interrupt, interrupt flags are found here UART3:MIS"]
    #[inline(always)]
    pub fn is_uart3_comb(&self) -> bool {
        *self == Ev::Uart3Comb
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART3 combined interrupt, interrupt flags are found here UART3:MIS"]
    #[inline(always)]
    pub fn uart3_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Uart3Comb)
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
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Cpuirqsel41Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel41Spec;
impl crate::RegisterSpec for Cpuirqsel41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel41::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel41Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel41::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL41 to value 0x7d"]
impl crate::Resettable for Cpuirqsel41Spec {
    const RESET_VALUE: u32 = 0x7d;
}
