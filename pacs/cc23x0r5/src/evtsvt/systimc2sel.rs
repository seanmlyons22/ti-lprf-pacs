#[doc = "Register `SYSTIMC2SEL` reader"]
pub type R = crate::R<Systimc2selSpec>;
#[doc = "Register `SYSTIMC2SEL` writer"]
pub type W = crate::W<Systimc2selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 42"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "42: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    LrfdEvt0 = 42,
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
            42 => Some(Pubid::LrfdEvt0),
            _ => None,
        }
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    #[inline(always)]
    pub fn is_lrfd_evt0(&self) -> bool {
        *self == Pubid::LrfdEvt0
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
#[doc = "Output Selection for CPU Interrupt SYSTIMC2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimc2sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimc2sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Systimc2selSpec;
impl crate::RegisterSpec for Systimc2selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimc2sel::R`](R) reader structure"]
impl crate::Readable for Systimc2selSpec {}
#[doc = "`write(|w| ..)` method takes [`systimc2sel::W`](W) writer structure"]
impl crate::Writable for Systimc2selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTIMC2SEL to value 0x2a"]
impl crate::Resettable for Systimc2selSpec {
    const RESET_VALUE: u32 = 0x2a;
}
