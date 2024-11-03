#[doc = "Register `LRFDIN1SEL` reader"]
pub type R = crate::R<Lrfdin1selSpec>;
#[doc = "Register `LRFDIN1SEL` writer"]
pub type W = crate::W<Lrfdin1selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 30"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "30: SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    Systim3 = 30,
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
            30 => Some(Pubid::Systim3),
            _ => None,
        }
    }
    #[doc = "SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    #[inline(always)]
    pub fn is_systim3(&self) -> bool {
        *self == Pubid::Systim3
    }
}
#[doc = "Field `PUBID` writer - 5:0\\]
Read only selection value"]
pub type PubidW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pubid>;
impl<'a, REG> PubidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    #[inline(always)]
    pub fn systim3(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim3)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
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
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn pubid(&mut self) -> PubidW<Lrfdin1selSpec> {
        PubidW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Lrfdin1selSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Output Selection for CPU Interrupt LRFDIN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrfdin1sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrfdin1sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lrfdin1selSpec;
impl crate::RegisterSpec for Lrfdin1selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrfdin1sel::R`](R) reader structure"]
impl crate::Readable for Lrfdin1selSpec {}
#[doc = "`write(|w| ..)` method takes [`lrfdin1sel::W`](W) writer structure"]
impl crate::Writable for Lrfdin1selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LRFDIN1SEL to value 0x1e"]
impl crate::Resettable for Lrfdin1selSpec {
    const RESET_VALUE: u32 = 0x1e;
}
