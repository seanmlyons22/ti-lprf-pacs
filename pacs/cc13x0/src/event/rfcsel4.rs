#[doc = "Register `RFCSEL4` reader"]
pub type R = crate::R<Rfcsel4Spec>;
#[doc = "Register `RFCSEL4` writer"]
pub type W = crate::W<Rfcsel4Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "65: GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    Gpt2aCmp = 65,
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
            65 => Some(Ev::Gpt2aCmp),
            _ => None,
        }
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == Ev::Gpt2aCmp
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
#[doc = "Output Selection for RFC Event 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfcsel4Spec;
impl crate::RegisterSpec for Rfcsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcsel4::R`](R) reader structure"]
impl crate::Readable for Rfcsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcsel4::W`](W) writer structure"]
impl crate::Writable for Rfcsel4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCSEL4 to value 0x41"]
impl crate::Resettable for Rfcsel4Spec {
    const RESET_VALUE: u32 = 0x41;
}
