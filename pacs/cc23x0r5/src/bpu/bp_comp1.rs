#[doc = "Register `BP_COMP1` reader"]
pub type R = crate::R<BpComp1Spec>;
#[doc = "Register `BP_COMP1` writer"]
pub type W = crate::W<BpComp1Spec>;
#[doc = "0:0\\]
Comparison address, UNKNOWN on reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: Breakpoint Comparator Register 1 compare enabled"]
    BkptCompDis = 1,
    #[doc = "0: Breakpoint Comparator Register 1 compare disabled"]
    BkptCompEn = 0,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Comparison address, UNKNOWN on reset."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            true => Enable::BkptCompDis,
            false => Enable::BkptCompEn,
        }
    }
    #[doc = "Breakpoint Comparator Register 1 compare enabled"]
    #[inline(always)]
    pub fn is_bkpt_comp_dis(&self) -> bool {
        *self == Enable::BkptCompDis
    }
    #[doc = "Breakpoint Comparator Register 1 compare disabled"]
    #[inline(always)]
    pub fn is_bkpt_comp_en(&self) -> bool {
        *self == Enable::BkptCompEn
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Comparison address, UNKNOWN on reset."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Breakpoint Comparator Register 1 compare enabled"]
    #[inline(always)]
    pub fn bkpt_comp_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::BkptCompDis)
    }
    #[doc = "Breakpoint Comparator Register 1 compare disabled"]
    #[inline(always)]
    pub fn bkpt_comp_en(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::BkptCompEn)
    }
}
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - 28:2\\]
Comparison address. Although it is architecturally Unpredictable whether breakpoint matches on the address of the second halfword of a 32-bit instruction to generates a debug event, in this processor it is predictable and a debug event is generated."]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - 28:2\\]
Comparison address. Although it is architecturally Unpredictable whether breakpoint matches on the address of the second halfword of a 32-bit instruction to generates a debug event, in this processor it is predictable and a debug event is generated."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `RESERVED29` reader - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::BitReader;
#[doc = "Field `RESERVED29` writer - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "31:30\\]
This selects what happens when the COMP address is matched\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BpMatch {
    #[doc = "3: Set breakpoint on both lower and upper halfwords"]
    BkptCompBoth = 3,
    #[doc = "2: Set breakpoint on upper halfword, lower is unaffected"]
    BkptCompHi = 2,
    #[doc = "1: Set breakpoint on lower halfword, upper is unaffected"]
    BkptCompLow = 1,
    #[doc = "0: No breakpoint generated"]
    BkptCompNone = 0,
}
impl From<BpMatch> for u8 {
    #[inline(always)]
    fn from(variant: BpMatch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BpMatch {
    type Ux = u8;
}
impl crate::IsEnum for BpMatch {}
#[doc = "Field `BP_MATCH` reader - 31:30\\]
This selects what happens when the COMP address is matched"]
pub type BpMatchR = crate::FieldReader<BpMatch>;
impl BpMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BpMatch {
        match self.bits {
            3 => BpMatch::BkptCompBoth,
            2 => BpMatch::BkptCompHi,
            1 => BpMatch::BkptCompLow,
            0 => BpMatch::BkptCompNone,
            _ => unreachable!(),
        }
    }
    #[doc = "Set breakpoint on both lower and upper halfwords"]
    #[inline(always)]
    pub fn is_bkpt_comp_both(&self) -> bool {
        *self == BpMatch::BkptCompBoth
    }
    #[doc = "Set breakpoint on upper halfword, lower is unaffected"]
    #[inline(always)]
    pub fn is_bkpt_comp_hi(&self) -> bool {
        *self == BpMatch::BkptCompHi
    }
    #[doc = "Set breakpoint on lower halfword, upper is unaffected"]
    #[inline(always)]
    pub fn is_bkpt_comp_low(&self) -> bool {
        *self == BpMatch::BkptCompLow
    }
    #[doc = "No breakpoint generated"]
    #[inline(always)]
    pub fn is_bkpt_comp_none(&self) -> bool {
        *self == BpMatch::BkptCompNone
    }
}
#[doc = "Field `BP_MATCH` writer - 31:30\\]
This selects what happens when the COMP address is matched"]
pub type BpMatchW<'a, REG> = crate::FieldWriter<'a, REG, 2, BpMatch, crate::Safe>;
impl<'a, REG> BpMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set breakpoint on both lower and upper halfwords"]
    #[inline(always)]
    pub fn bkpt_comp_both(self) -> &'a mut crate::W<REG> {
        self.variant(BpMatch::BkptCompBoth)
    }
    #[doc = "Set breakpoint on upper halfword, lower is unaffected"]
    #[inline(always)]
    pub fn bkpt_comp_hi(self) -> &'a mut crate::W<REG> {
        self.variant(BpMatch::BkptCompHi)
    }
    #[doc = "Set breakpoint on lower halfword, upper is unaffected"]
    #[inline(always)]
    pub fn bkpt_comp_low(self) -> &'a mut crate::W<REG> {
        self.variant(BpMatch::BkptCompLow)
    }
    #[doc = "No breakpoint generated"]
    #[inline(always)]
    pub fn bkpt_comp_none(self) -> &'a mut crate::W<REG> {
        self.variant(BpMatch::BkptCompNone)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Comparison address, UNKNOWN on reset."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address. Although it is architecturally Unpredictable whether breakpoint matches on the address of the second halfword of a 32-bit instruction to generates a debug event, in this processor it is predictable and a debug event is generated."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits >> 2) & 0x07ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched"]
    #[inline(always)]
    pub fn bp_match(&self) -> BpMatchR {
        BpMatchR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Comparison address, UNKNOWN on reset."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<BpComp1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<BpComp1Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address. Although it is architecturally Unpredictable whether breakpoint matches on the address of the second halfword of a 32-bit instruction to generates a debug event, in this processor it is predictable and a debug event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<BpComp1Spec> {
        CompW::new(self, 2)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<BpComp1Spec> {
        Reserved29W::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched"]
    #[inline(always)]
    #[must_use]
    pub fn bp_match(&mut self) -> BpMatchW<BpComp1Spec> {
        BpMatchW::new(self, 30)
    }
}
#[doc = "Use the Breakpoint Comparator Registers to store the values to compare with the instruction address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_comp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_comp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpComp1Spec;
impl crate::RegisterSpec for BpComp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bp_comp1::R`](R) reader structure"]
impl crate::Readable for BpComp1Spec {}
#[doc = "`write(|w| ..)` method takes [`bp_comp1::W`](W) writer structure"]
impl crate::Writable for BpComp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BP_COMP1 to value 0"]
impl crate::Resettable for BpComp1Spec {
    const RESET_VALUE: u32 = 0;
}
