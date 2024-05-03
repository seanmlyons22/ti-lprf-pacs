#[doc = "Register `CPUIRQSEL16` reader"]
pub type R = crate::R<Cpuirqsel16Spec>;
#[doc = "Register `CPUIRQSEL16` writer"]
pub type W = crate::W<Cpuirqsel16Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "17: GPT0B interrupt event, controlled by GPT0:TBMR"]
    Gpt0b = 17,
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
            17 => Some(Ev::Gpt0b),
            _ => None,
        }
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn is_gpt0b(&self) -> bool {
        *self == Ev::Gpt0b
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
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn gpt0b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0b)
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
    pub fn ev(&mut self) -> EvW<Cpuirqsel16Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel16Spec;
impl crate::RegisterSpec for Cpuirqsel16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel16::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel16Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel16::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL16 to value 0x11"]
impl crate::Resettable for Cpuirqsel16Spec {
    const RESET_VALUE: u32 = 0x11;
}
