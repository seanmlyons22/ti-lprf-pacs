#[doc = "Register `RFCSEL0` reader"]
pub type R = crate::R<Rfcsel0Spec>;
#[doc = "Register `RFCSEL0` writer"]
pub type W = crate::W<Rfcsel0Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 61"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    Gpt0aCmp = 61,
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
            61 => Some(Ev::Gpt0aCmp),
            _ => None,
        }
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == Ev::Gpt0aCmp
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
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0aCmp)
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
    pub fn ev(&mut self) -> EvW<Rfcsel0Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for RFC Event 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel0Spec;
impl crate::RegisterSpec for Rfcsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel0::R`](R) reader structure"]
impl crate::Readable for Rfcsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel0::W`](W) writer structure"]
impl crate::Writable for Rfcsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL0 to value 0x3d"]
impl crate::Resettable for Rfcsel0Spec {
    const RESET_VALUE: u32 = 0x3d;
}
