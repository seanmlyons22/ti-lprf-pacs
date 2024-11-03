#[doc = "Register `AONRSTA1` reader"]
pub type R = crate::R<Aonrsta1Spec>;
#[doc = "Register `AONRSTA1` writer"]
pub type W = crate::W<Aonrsta1Spec>;
#[doc = "17:0\\]
State of the AON register flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Flag {
    #[doc = "262143: All flags set"]
    AllSet = 262143,
    #[doc = "0: All flags cleared"]
    AllClr = 0,
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
State of the AON register flags"]
pub type FlagR = crate::FieldReader<Flag>;
impl FlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flag> {
        match self.bits {
            262143 => Some(Flag::AllSet),
            0 => Some(Flag::AllClr),
            _ => None,
        }
    }
    #[doc = "All flags set"]
    #[inline(always)]
    pub fn is_all_set(&self) -> bool {
        *self == Flag::AllSet
    }
    #[doc = "All flags cleared"]
    #[inline(always)]
    pub fn is_all_clr(&self) -> bool {
        *self == Flag::AllClr
    }
}
#[doc = "Field `FLAG` writer - 17:0\\]
State of the AON register flags"]
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 18, Flag>;
impl<'a, REG> FlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "All flags set"]
    #[inline(always)]
    pub fn all_set(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AllSet)
    }
    #[doc = "All flags cleared"]
    #[inline(always)]
    pub fn all_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AllClr)
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
State of the AON register flags"]
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
State of the AON register flags"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FlagW<Aonrsta1Spec> {
        FlagW::new(self, 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<Aonrsta1Spec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "AON Register Status 1. This register contains the general purpose AON flags for SW, and is updated through AONRSET1.FLAG and AONRCLR1.FLAG. The register is only reset on a POR event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonrsta1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonrsta1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aonrsta1Spec;
impl crate::RegisterSpec for Aonrsta1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aonrsta1::R`](R) reader structure"]
impl crate::Readable for Aonrsta1Spec {}
#[doc = "`write(|w| ..)` method takes [`aonrsta1::W`](W) writer structure"]
impl crate::Writable for Aonrsta1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AONRSTA1 to value 0"]
impl crate::Resettable for Aonrsta1Spec {
    const RESET_VALUE: u32 = 0;
}
