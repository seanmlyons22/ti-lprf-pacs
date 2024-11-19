#[doc = "Register `ICLR1` reader"]
pub type R = crate::R<Iclr1Spec>;
#[doc = "Register `ICLR1` writer"]
pub type W = crate::W<Iclr1Spec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "2:2\\]
Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highifg {
    #[doc = "1: Interrupt is pending."]
    Clr = 1,
    #[doc = "0: Interrupt is not pending."]
    NoEffect = 0,
}
impl From<Highifg> for bool {
    #[inline(always)]
    fn from(variant: Highifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHIFG` reader - 2:2\\]
Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type HighifgR = crate::BitReader<Highifg>;
impl HighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highifg {
        match self.bits {
            true => Highifg::Clr,
            false => Highifg::NoEffect,
        }
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Highifg::Clr
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Highifg::NoEffect
    }
}
#[doc = "Field `HIGHIFG` writer - 2:2\\]
Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type HighifgW<'a, REG> = crate::BitWriter<'a, REG, Highifg>;
impl<'a, REG> HighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Highifg::Clr)
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Highifg::NoEffect)
    }
}
#[doc = "3:3\\]
Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowifg {
    #[doc = "1: Interrupt is pending."]
    Clr = 1,
    #[doc = "0: Interrupt is not pending."]
    NoEffect = 0,
}
impl From<Lowifg> for bool {
    #[inline(always)]
    fn from(variant: Lowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWIFG` reader - 3:3\\]
Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type LowifgR = crate::BitReader<Lowifg>;
impl LowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowifg {
        match self.bits {
            true => Lowifg::Clr,
            false => Lowifg::NoEffect,
        }
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Lowifg::Clr
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Lowifg::NoEffect
    }
}
#[doc = "Field `LOWIFG` writer - 3:3\\]
Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type LowifgW<'a, REG> = crate::BitWriter<'a, REG, Lowifg>;
impl<'a, REG> LowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lowifg::Clr)
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lowifg::NoEffect)
    }
}
#[doc = "4:4\\]
Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inifg {
    #[doc = "1: Interrupt is pending."]
    Clr = 1,
    #[doc = "0: Interrupt is not pending."]
    NoEffect = 0,
}
impl From<Inifg> for bool {
    #[inline(always)]
    fn from(variant: Inifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIFG` reader - 4:4\\]
Mask INIFG in MIS_EX register."]
pub type InifgR = crate::BitReader<Inifg>;
impl InifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inifg {
        match self.bits {
            true => Inifg::Clr,
            false => Inifg::NoEffect,
        }
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Inifg::Clr
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Inifg::NoEffect
    }
}
#[doc = "Field `INIFG` writer - 4:4\\]
Mask INIFG in MIS_EX register."]
pub type InifgW<'a, REG> = crate::BitWriter<'a, REG, Inifg>;
impl<'a, REG> InifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Inifg::Clr)
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Inifg::NoEffect)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg0 {
    #[doc = "1: A new data is ready to be read."]
    Clr = 1,
    #[doc = "0: No new data ready."]
    NoEffect = 0,
}
impl From<Memresifg0> for bool {
    #[inline(always)]
    fn from(variant: Memresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG0` reader - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0R = crate::BitReader<Memresifg0>;
impl Memresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg0 {
        match self.bits {
            true => Memresifg0::Clr,
            false => Memresifg0::NoEffect,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg0::Clr
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Memresifg0::NoEffect
    }
}
#[doc = "Field `MEMRESIFG0` writer - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0W<'a, REG> = crate::BitWriter<'a, REG, Memresifg0>;
impl<'a, REG> Memresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Clr)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::NoEffect)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn highifg(&self) -> HighifgR {
        HighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn lowifg(&self) -> LowifgR {
        LowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn inifg(&self) -> InifgR {
        InifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg0(&self) -> Memresifg0R {
        Memresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Iclr1Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn highifg(&mut self) -> HighifgW<Iclr1Spec> {
        HighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn lowifg(&mut self) -> LowifgW<Iclr1Spec> {
        LowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Mask INIFG in MIS_EX register."]
    #[inline(always)]
    #[must_use]
    pub fn inifg(&mut self) -> InifgW<Iclr1Spec> {
        InifgW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Iclr1Spec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg0(&mut self) -> Memresifg0W<Iclr1Spec> {
        Memresifg0W::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Iclr1Spec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iclr1Spec;
impl crate::RegisterSpec for Iclr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iclr1::R`](R) reader structure"]
impl crate::Readable for Iclr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iclr1::W`](W) writer structure"]
impl crate::Writable for Iclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR1 to value 0"]
impl crate::Resettable for Iclr1Spec {
    const RESET_VALUE: u32 = 0;
}