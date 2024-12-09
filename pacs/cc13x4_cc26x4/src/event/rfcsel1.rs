#[doc = "Register `RFCSEL1` reader"]
pub type R = crate::R<Rfcsel1Spec>;
#[doc = "Register `RFCSEL1` writer"]
pub type W = crate::W<Rfcsel1Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 62"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    Gpt0bCmp = 62,
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
            62 => Some(Ev::Gpt0bCmp),
            _ => None,
        }
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == Ev::Gpt0bCmp
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
#[doc = "Output Selection for RFC Event 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel1Spec;
impl crate::RegisterSpec for Rfcsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel1::R`](R) reader structure"]
impl crate::Readable for Rfcsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel1::W`](W) writer structure"]
impl crate::Writable for Rfcsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL1 to value 0x3e"]
impl crate::Resettable for Rfcsel1Spec {
    const RESET_VALUE: u32 = 0x3e;
}
