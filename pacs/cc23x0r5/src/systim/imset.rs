#[doc = "Register `IMSET` reader"]
pub type R = crate::R<ImsetSpec>;
#[doc = "Register `IMSET` writer"]
pub type W = crate::W<ImsetSpec>;
#[doc = "0:0\\]
Sets channel0 Event Interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev0> for bool {
    #[inline(always)]
    fn from(variant: Ev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - 0:0\\]
Sets channel0 Event Interrupt mask"]
pub type Ev0R = crate::BitReader<Ev0>;
impl Ev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0 {
        match self.bits {
            true => Ev0::Set,
            false => Ev0::Noeff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev0::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ev0::Noeff
    }
}
#[doc = "Field `EV0` writer - 0:0\\]
Sets channel0 Event Interrupt mask"]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Ev0>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Noeff)
    }
}
#[doc = "1:1\\]
Sets channel1 Event Interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev1> for bool {
    #[inline(always)]
    fn from(variant: Ev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - 1:1\\]
Sets channel1 Event Interrupt mask"]
pub type Ev1R = crate::BitReader<Ev1>;
impl Ev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1 {
        match self.bits {
            true => Ev1::Set,
            false => Ev1::Noeff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev1::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ev1::Noeff
    }
}
#[doc = "Field `EV1` writer - 1:1\\]
Sets channel1 Event Interrupt mask"]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Ev1>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Noeff)
    }
}
#[doc = "2:2\\]
Sets channel2 Event Interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev2 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev2> for bool {
    #[inline(always)]
    fn from(variant: Ev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2` reader - 2:2\\]
Sets channel2 Event Interrupt mask"]
pub type Ev2R = crate::BitReader<Ev2>;
impl Ev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev2 {
        match self.bits {
            true => Ev2::Set,
            false => Ev2::Noeff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev2::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ev2::Noeff
    }
}
#[doc = "Field `EV2` writer - 2:2\\]
Sets channel2 Event Interrupt mask"]
pub type Ev2W<'a, REG> = crate::BitWriter<'a, REG, Ev2>;
impl<'a, REG> Ev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::Noeff)
    }
}
#[doc = "3:3\\]
Sets channel3 Event Interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev3 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev3> for bool {
    #[inline(always)]
    fn from(variant: Ev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV3` reader - 3:3\\]
Sets channel3 Event Interrupt mask"]
pub type Ev3R = crate::BitReader<Ev3>;
impl Ev3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev3 {
        match self.bits {
            true => Ev3::Set,
            false => Ev3::Noeff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev3::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ev3::Noeff
    }
}
#[doc = "Field `EV3` writer - 3:3\\]
Sets channel3 Event Interrupt mask"]
pub type Ev3W<'a, REG> = crate::BitWriter<'a, REG, Ev3>;
impl<'a, REG> Ev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev3::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev3::Noeff)
    }
}
#[doc = "4:4\\]
Sets channel4 Event Interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev4 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev4> for bool {
    #[inline(always)]
    fn from(variant: Ev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV4` reader - 4:4\\]
Sets channel4 Event Interrupt mask"]
pub type Ev4R = crate::BitReader<Ev4>;
impl Ev4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev4 {
        match self.bits {
            true => Ev4::Set,
            false => Ev4::Noeff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev4::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ev4::Noeff
    }
}
#[doc = "Field `EV4` writer - 4:4\\]
Sets channel4 Event Interrupt mask"]
pub type Ev4W<'a, REG> = crate::BitWriter<'a, REG, Ev4>;
impl<'a, REG> Ev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev4::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev4::Noeff)
    }
}
#[doc = "5:5\\]
Sets Timer Overflow Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfl {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ovfl> for bool {
    #[inline(always)]
    fn from(variant: Ovfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFL` reader - 5:5\\]
Sets Timer Overflow Event Interrupt Mask."]
pub type OvflR = crate::BitReader<Ovfl>;
impl OvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfl {
        match self.bits {
            true => Ovfl::Set,
            false => Ovfl::Noeff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ovfl::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Ovfl::Noeff
    }
}
#[doc = "Field `OVFL` writer - 5:5\\]
Sets Timer Overflow Event Interrupt Mask."]
pub type OvflW<'a, REG> = crate::BitWriter<'a, REG, Ovfl>;
impl<'a, REG> OvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfl::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfl::Noeff)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets channel0 Event Interrupt mask"]
    #[inline(always)]
    pub fn ev0(&self) -> Ev0R {
        Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sets channel1 Event Interrupt mask"]
    #[inline(always)]
    pub fn ev1(&self) -> Ev1R {
        Ev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Sets channel2 Event Interrupt mask"]
    #[inline(always)]
    pub fn ev2(&self) -> Ev2R {
        Ev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Sets channel3 Event Interrupt mask"]
    #[inline(always)]
    pub fn ev3(&self) -> Ev3R {
        Ev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sets channel4 Event Interrupt mask"]
    #[inline(always)]
    pub fn ev4(&self) -> Ev4R {
        Ev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Sets Timer Overflow Event Interrupt Mask."]
    #[inline(always)]
    pub fn ovfl(&self) -> OvflR {
        OvflR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sets channel0 Event Interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ev0(&mut self) -> Ev0W<ImsetSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sets channel1 Event Interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ev1(&mut self) -> Ev1W<ImsetSpec> {
        Ev1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Sets channel2 Event Interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ev2(&mut self) -> Ev2W<ImsetSpec> {
        Ev2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Sets channel3 Event Interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ev3(&mut self) -> Ev3W<ImsetSpec> {
        Ev3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Sets channel4 Event Interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ev4(&mut self) -> Ev4W<ImsetSpec> {
        Ev4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Sets Timer Overflow Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ovfl(&mut self) -> OvflW<ImsetSpec> {
        OvflW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<ImsetSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImsetSpec;
impl crate::RegisterSpec for ImsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imset::R`](R) reader structure"]
impl crate::Readable for ImsetSpec {}
#[doc = "`write(|w| ..)` method takes [`imset::W`](W) writer structure"]
impl crate::Writable for ImsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSET to value 0"]
impl crate::Resettable for ImsetSpec {
    const RESET_VALUE: u32 = 0;
}