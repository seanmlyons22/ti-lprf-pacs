#[doc = "Register `RFCSEL7` reader"]
pub type R = crate::R<Rfcsel7Spec>;
#[doc = "Register `RFCSEL7` writer"]
pub type W = crate::W<Rfcsel7Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 68"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "68: GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    Gpt3bCmp = 68,
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
            68 => Some(Ev::Gpt3bCmp),
            _ => None,
        }
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == Ev::Gpt3bCmp
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
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt3b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3bCmp)
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
    pub fn ev(&mut self) -> EvW<Rfcsel7Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for RFC Event 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel7Spec;
impl crate::RegisterSpec for Rfcsel7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel7::R`](R) reader structure"]
impl crate::Readable for Rfcsel7Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel7::W`](W) writer structure"]
impl crate::Writable for Rfcsel7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL7 to value 0x44"]
impl crate::Resettable for Rfcsel7Spec {
    const RESET_VALUE: u32 = 0x44;
}
