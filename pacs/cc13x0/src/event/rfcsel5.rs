#[doc = "Register `RFCSEL5` reader"]
pub type R = crate::R<Rfcsel5Spec>;
#[doc = "Register `RFCSEL5` writer"]
pub type W = crate::W<Rfcsel5Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 66"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    Gpt2bCmp = 66,
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
            66 => Some(Ev::Gpt2bCmp),
            _ => None,
        }
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == Ev::Gpt2bCmp
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
impl W {}
#[doc = "Output Selection for RFC Event 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel5Spec;
impl crate::RegisterSpec for Rfcsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel5::R`](R) reader structure"]
impl crate::Readable for Rfcsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel5::W`](W) writer structure"]
impl crate::Writable for Rfcsel5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL5 to value 0x42"]
impl crate::Resettable for Rfcsel5Spec {
    const RESET_VALUE: u32 = 0x42;
}
