#[doc = "Register `RFCSEL2` reader"]
pub type R = crate::R<Rfcsel2Spec>;
#[doc = "Register `RFCSEL2` writer"]
pub type W = crate::W<Rfcsel2Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "63: GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    Gpt1aCmp = 63,
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
            63 => Some(Ev::Gpt1aCmp),
            _ => None,
        }
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == Ev::Gpt1aCmp
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
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt1a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1aCmp)
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
    pub fn ev(&mut self) -> EvW<Rfcsel2Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for RFC Event 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel2Spec;
impl crate::RegisterSpec for Rfcsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel2::R`](R) reader structure"]
impl crate::Readable for Rfcsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel2::W`](W) writer structure"]
impl crate::Writable for Rfcsel2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL2 to value 0x3f"]
impl crate::Resettable for Rfcsel2Spec {
    const RESET_VALUE: u32 = 0x3f;
}
