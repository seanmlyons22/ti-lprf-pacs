#[doc = "Register `CPUIRQSEL39` reader"]
pub type R = crate::R<Cpuirqsel39Spec>;
#[doc = "Register `CPUIRQSEL39` writer"]
pub type W = crate::W<Cpuirqsel39Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 123"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "123: SPI3 combined interrupt, interrupt flags are found here SPI3:MIS"]
    Spi3Comb = 123,
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
            123 => Some(Ev::Spi3Comb),
            _ => None,
        }
    }
    #[doc = "SPI3 combined interrupt, interrupt flags are found here SPI3:MIS"]
    #[inline(always)]
    pub fn is_spi3_comb(&self) -> bool {
        *self == Ev::Spi3Comb
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
#[doc = "Output Selection for CPU Interrupt 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel39Spec;
impl crate::RegisterSpec for Cpuirqsel39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel39::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel39Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel39::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL39 to value 0x7b"]
impl crate::Resettable for Cpuirqsel39Spec {
    const RESET_VALUE: u32 = 0x7b;
}
