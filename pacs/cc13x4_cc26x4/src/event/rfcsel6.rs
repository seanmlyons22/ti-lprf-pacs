#[doc = "Register `RFCSEL6` reader"]
pub type R = crate::R<Rfcsel6Spec>;
#[doc = "Register `RFCSEL6` writer"]
pub type W = crate::W<Rfcsel6Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 67"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "67: GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    Gpt3aCmp = 67,
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
            67 => Some(Ev::Gpt3aCmp),
            _ => None,
        }
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == Ev::Gpt3aCmp
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
#[doc = "Output Selection for RFC Event 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel6Spec;
impl crate::RegisterSpec for Rfcsel6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel6::R`](R) reader structure"]
impl crate::Readable for Rfcsel6Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel6::W`](W) writer structure"]
impl crate::Writable for Rfcsel6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL6 to value 0x43"]
impl crate::Resettable for Rfcsel6Spec {
    const RESET_VALUE: u32 = 0x43;
}
