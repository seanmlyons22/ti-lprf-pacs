#[doc = "Register `CPUIRQSEL38` reader"]
pub type R = crate::R<Cpuirqsel38Spec>;
#[doc = "Register `CPUIRQSEL38` writer"]
pub type W = crate::W<Cpuirqsel38Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 122"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "122: SPI2 combined interrupt, interrupt flags are found here SPI2:MIS"]
    Spi2Comb = 122,
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
            122 => Some(Ev::Spi2Comb),
            _ => None,
        }
    }
    #[doc = "SPI2 combined interrupt, interrupt flags are found here SPI2:MIS"]
    #[inline(always)]
    pub fn is_spi2_comb(&self) -> bool {
        *self == Ev::Spi2Comb
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
    #[doc = "SPI2 combined interrupt, interrupt flags are found here SPI2:MIS"]
    #[inline(always)]
    pub fn spi2_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Spi2Comb)
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
    pub fn ev(&mut self) -> EvW<Cpuirqsel38Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel38Spec;
impl crate::RegisterSpec for Cpuirqsel38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel38::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel38Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel38::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL38 to value 0x7a"]
impl crate::Resettable for Cpuirqsel38Spec {
    const RESET_VALUE: u32 = 0x7a;
}
