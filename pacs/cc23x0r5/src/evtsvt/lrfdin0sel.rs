#[doc = "Register `LRFDIN0SEL` reader"]
pub type R = crate::R<Lrfdin0selSpec>;
#[doc = "Register `LRFDIN0SEL` writer"]
pub type W = crate::W<Lrfdin0selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 29"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "29: SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    Systim2 = 29,
}
impl From<Pubid> for u8 {
    #[inline(always)]
    fn from(variant: Pubid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pubid {
    type Ux = u8;
}
impl crate::IsEnum for Pubid {}
#[doc = "Field `PUBID` reader - 5:0\\]
Read only selection value"]
pub type PubidR = crate::FieldReader<Pubid>;
impl PubidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pubid> {
        match self.bits {
            29 => Some(Pubid::Systim2),
            _ => None,
        }
    }
    #[doc = "SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    #[inline(always)]
    pub fn is_systim2(&self) -> bool {
        *self == Pubid::Systim2
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn pubid(&self) -> PubidR {
        PubidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
#[doc = "Output Selection for CPU Interrupt LRFDIN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrfdin0sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrfdin0sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lrfdin0selSpec;
impl crate::RegisterSpec for Lrfdin0selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrfdin0sel::R`](R) reader structure"]
impl crate::Readable for Lrfdin0selSpec {}
#[doc = "`write(|w| ..)` method takes [`lrfdin0sel::W`](W) writer structure"]
impl crate::Writable for Lrfdin0selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LRFDIN0SEL to value 0x1d"]
impl crate::Resettable for Lrfdin0selSpec {
    const RESET_VALUE: u32 = 0x1d;
}
