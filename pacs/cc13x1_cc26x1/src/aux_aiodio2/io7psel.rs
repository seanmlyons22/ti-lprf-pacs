#[doc = "Register `IO7PSEL` reader"]
pub type R = crate::R<Io7pselSpec>;
#[doc = "Register `IO7PSEL` writer"]
pub type W = crate::W<Io7pselSpec>;
#[doc = "2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    AuxEvObs = 0,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::AuxEvObs),
            _ => None,
        }
    }
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline(always)]
    pub fn is_aux_ev_obs(&self) -> bool {
        *self == Src::AuxEvObs
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline(always)]
    pub fn aux_ev_obs(self) -> &'a mut crate::W<REG> {
        self.variant(Src::AuxEvObs)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is set."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<Io7pselSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Input Output 7 Peripheral Select This register selects a peripheral signal that connects to AUXIO\\[8i+7\\]
when IOPOE bit 7 is 1. To avoid glitches on AUXIO\\[8i+7\\]
you must configure this register while IOPOE bit 7 is 0. In the formulas i = 0 for AUX_AIODIO0, i = 1 for AUX_AIODIO1, and so forth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io7psel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io7psel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Io7pselSpec;
impl crate::RegisterSpec for Io7pselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io7psel::R`](R) reader structure"]
impl crate::Readable for Io7pselSpec {}
#[doc = "`write(|w| ..)` method takes [`io7psel::W`](W) writer structure"]
impl crate::Writable for Io7pselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO7PSEL to value 0"]
impl crate::Resettable for Io7pselSpec {
    const RESET_VALUE: u32 = 0;
}
