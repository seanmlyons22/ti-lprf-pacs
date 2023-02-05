#[doc = "Register `AONRCLR1` reader"]
pub type R = crate::R<Aonrclr1Spec>;
#[doc = "Register `AONRCLR1` writer"]
pub type W = crate::W<Aonrclr1Spec>;
#[doc = "17:0\\]
Write 1 to clear AONRSTA1.FLAG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Flag {
    #[doc = "262143: Clear all flags"]
    AllClr = 262143,
    #[doc = "0: No flags changed status"]
    Noeff = 0,
}
impl From<Flag> for u32 {
    #[inline(always)]
    fn from(variant: Flag) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flag {
    type Ux = u32;
}
impl crate::IsEnum for Flag {}
#[doc = "Field `FLAG` writer - 17:0\\]
Write 1 to clear AONRSTA1.FLAG"]
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 18, Flag>;
impl<'a, REG> FlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Clear all flags"]
    #[inline(always)]
    pub fn all_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AllClr)
    }
    #[doc = "No flags changed status"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::Noeff)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
Write 1 to clear AONRSTA1.FLAG"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FlagW<Aonrclr1Spec> {
        FlagW::new(self, 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<Aonrclr1Spec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "AON Register Clear 1. This register clears the AON flags that can be read through AONRSTA1.FLAG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonrclr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonrclr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aonrclr1Spec;
impl crate::RegisterSpec for Aonrclr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aonrclr1::R`](R) reader structure"]
impl crate::Readable for Aonrclr1Spec {}
#[doc = "`write(|w| ..)` method takes [`aonrclr1::W`](W) writer structure"]
impl crate::Writable for Aonrclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AONRCLR1 to value 0"]
impl crate::Resettable for Aonrclr1Spec {
    const RESET_VALUE: u32 = 0;
}
