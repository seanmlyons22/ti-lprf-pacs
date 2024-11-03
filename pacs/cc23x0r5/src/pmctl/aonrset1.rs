#[doc = "Register `AONRSET1` reader"]
pub type R = crate::R<Aonrset1Spec>;
#[doc = "Register `AONRSET1` writer"]
pub type W = crate::W<Aonrset1Spec>;
#[doc = "17:0\\]
Write 1 to set AONRSTA1.FLAG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Flag {
    #[doc = "262143: Set all flags"]
    AllSet = 262143,
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
#[doc = "Field `FLAG` reader - 17:0\\]
Write 1 to set AONRSTA1.FLAG"]
pub type FlagR = crate::FieldReader<Flag>;
impl FlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flag> {
        match self.bits {
            262143 => Some(Flag::AllSet),
            0 => Some(Flag::Noeff),
            _ => None,
        }
    }
    #[doc = "Set all flags"]
    #[inline(always)]
    pub fn is_all_set(&self) -> bool {
        *self == Flag::AllSet
    }
    #[doc = "No flags changed status"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Flag::Noeff
    }
}
#[doc = "Field `FLAG` writer - 17:0\\]
Write 1 to set AONRSTA1.FLAG"]
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 18, Flag>;
impl<'a, REG> FlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Set all flags"]
    #[inline(always)]
    pub fn all_set(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AllSet)
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
    #[doc = "Bits 0:17 - 17:0\\]
Write 1 to set AONRSTA1.FLAG"]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
Write 1 to set AONRSTA1.FLAG"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FlagW<Aonrset1Spec> {
        FlagW::new(self, 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<Aonrset1Spec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "AON Register Set 1. This register sets the AON flags that can be read through AONRSTA1.FLAG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonrset1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonrset1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aonrset1Spec;
impl crate::RegisterSpec for Aonrset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aonrset1::R`](R) reader structure"]
impl crate::Readable for Aonrset1Spec {}
#[doc = "`write(|w| ..)` method takes [`aonrset1::W`](W) writer structure"]
impl crate::Writable for Aonrset1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AONRSET1 to value 0"]
impl crate::Resettable for Aonrset1Spec {
    const RESET_VALUE: u32 = 0;
}
