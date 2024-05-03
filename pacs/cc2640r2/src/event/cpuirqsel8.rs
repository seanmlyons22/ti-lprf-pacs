#[doc = "Register `CPUIRQSEL8` reader"]
pub type R = crate::R<Cpuirqsel8Spec>;
#[doc = "Register `CPUIRQSEL8` writer"]
pub type W = crate::W<Cpuirqsel8Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 35"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "35: SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    Ssi1Comb = 35,
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
            35 => Some(Ev::Ssi1Comb),
            _ => None,
        }
    }
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn is_ssi1_comb(&self) -> bool {
        *self == Ev::Ssi1Comb
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 7, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn ssi1_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Ssi1Comb)
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
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Cpuirqsel8Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel8Spec;
impl crate::RegisterSpec for Cpuirqsel8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel8::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel8Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel8::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL8 to value 0x23"]
impl crate::Resettable for Cpuirqsel8Spec {
    const RESET_VALUE: u32 = 0x23;
}
