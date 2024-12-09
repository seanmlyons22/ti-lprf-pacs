#[doc = "Register `CPUIRQSEL7` reader"]
pub type R = crate::R<Cpuirqsel7Spec>;
#[doc = "Register `CPUIRQSEL7` writer"]
pub type W = crate::W<Cpuirqsel7Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 34"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "34: SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    Spi0Comb = 34,
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
            34 => Some(Ev::Spi0Comb),
            _ => None,
        }
    }
    #[doc = "SPI0 combined interrupt, interrupt flags are found here SPI0:MIS"]
    #[inline(always)]
    pub fn is_spi0_comb(&self) -> bool {
        *self == Ev::Spi0Comb
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
#[doc = "Output Selection for CPU Interrupt 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel7Spec;
impl crate::RegisterSpec for Cpuirqsel7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel7::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel7Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel7::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL7 to value 0x22"]
impl crate::Resettable for Cpuirqsel7Spec {
    const RESET_VALUE: u32 = 0x22;
}
