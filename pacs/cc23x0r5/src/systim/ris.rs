#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "0:0\\]
Raw interrupt status for channel 0 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ev0> for bool {
    #[inline(always)]
    fn from(variant: Ev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - 0:0\\]
Raw interrupt status for channel 0 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 0."]
pub type Ev0R = crate::BitReader<Ev0>;
impl Ev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0 {
        match self.bits {
            true => Ev0::Set,
            false => Ev0::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev0::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ev0::Clr
    }
}
#[doc = "Field `EV0` writer - 0:0\\]
Raw interrupt status for channel 0 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 0."]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Ev0>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Set)
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Clr)
    }
}
#[doc = "1:1\\]
Raw interrupt status for channel 1 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ev1> for bool {
    #[inline(always)]
    fn from(variant: Ev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - 1:1\\]
Raw interrupt status for channel 1 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 1."]
pub type Ev1R = crate::BitReader<Ev1>;
impl Ev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1 {
        match self.bits {
            true => Ev1::Set,
            false => Ev1::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev1::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ev1::Clr
    }
}
#[doc = "Field `EV1` writer - 1:1\\]
Raw interrupt status for channel 1 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 1."]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Ev1>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Set)
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Clr)
    }
}
#[doc = "2:2\\]
Raw interrupt status for channel 2 Event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev2 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ev2> for bool {
    #[inline(always)]
    fn from(variant: Ev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2` reader - 2:2\\]
Raw interrupt status for channel 2 Event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 2."]
pub type Ev2R = crate::BitReader<Ev2>;
impl Ev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev2 {
        match self.bits {
            true => Ev2::Set,
            false => Ev2::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev2::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ev2::Clr
    }
}
#[doc = "Field `EV2` writer - 2:2\\]
Raw interrupt status for channel 2 Event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 2."]
pub type Ev2W<'a, REG> = crate::BitWriter<'a, REG, Ev2>;
impl<'a, REG> Ev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::Set)
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::Clr)
    }
}
#[doc = "3:3\\]
Raw interrupt status for channel 3 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev3 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ev3> for bool {
    #[inline(always)]
    fn from(variant: Ev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV3` reader - 3:3\\]
Raw interrupt status for channel 3 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 3."]
pub type Ev3R = crate::BitReader<Ev3>;
impl Ev3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev3 {
        match self.bits {
            true => Ev3::Set,
            false => Ev3::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev3::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ev3::Clr
    }
}
#[doc = "Field `EV3` writer - 3:3\\]
Raw interrupt status for channel 3 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 3."]
pub type Ev3W<'a, REG> = crate::BitWriter<'a, REG, Ev3>;
impl<'a, REG> Ev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev3::Set)
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev3::Clr)
    }
}
#[doc = "4:4\\]
Raw interrupt status for channel 4 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev4 {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ev4> for bool {
    #[inline(always)]
    fn from(variant: Ev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV4` reader - 4:4\\]
Raw interrupt status for channel 4 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 4."]
pub type Ev4R = crate::BitReader<Ev4>;
impl Ev4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev4 {
        match self.bits {
            true => Ev4::Set,
            false => Ev4::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev4::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ev4::Clr
    }
}
#[doc = "Field `EV4` writer - 4:4\\]
Raw interrupt status for channel 4 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 4."]
pub type Ev4W<'a, REG> = crate::BitWriter<'a, REG, Ev4>;
impl<'a, REG> Ev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev4::Set)
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev4::Clr)
    }
}
#[doc = "5:5\\]
Raw interrupt status for Systimer counter overflow event. This bit is set to 1 when an event is received on SysTimer Overflow occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfl {
    #[doc = "1: Interrupt occured"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ovfl> for bool {
    #[inline(always)]
    fn from(variant: Ovfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFL` reader - 5:5\\]
Raw interrupt status for Systimer counter overflow event. This bit is set to 1 when an event is received on SysTimer Overflow occurs."]
pub type OvflR = crate::BitReader<Ovfl>;
impl OvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfl {
        match self.bits {
            true => Ovfl::Set,
            false => Ovfl::Clr,
        }
    }
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ovfl::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ovfl::Clr
    }
}
#[doc = "Field `OVFL` writer - 5:5\\]
Raw interrupt status for Systimer counter overflow event. This bit is set to 1 when an event is received on SysTimer Overflow occurs."]
pub type OvflW<'a, REG> = crate::BitWriter<'a, REG, Ovfl>;
impl<'a, REG> OvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfl::Set)
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfl::Clr)
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
Raw interrupt status for channel 0 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 0."]
    #[inline(always)]
    pub fn ev0(&self) -> Ev0R {
        Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt status for channel 1 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 1."]
    #[inline(always)]
    pub fn ev1(&self) -> Ev1R {
        Ev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt status for channel 2 Event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 2."]
    #[inline(always)]
    pub fn ev2(&self) -> Ev2R {
        Ev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw interrupt status for channel 3 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 3."]
    #[inline(always)]
    pub fn ev3(&self) -> Ev3R {
        Ev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw interrupt status for channel 4 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 4."]
    #[inline(always)]
    pub fn ev4(&self) -> Ev4R {
        Ev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw interrupt status for Systimer counter overflow event. This bit is set to 1 when an event is received on SysTimer Overflow occurs."]
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
Raw interrupt status for channel 0 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn ev0(&mut self) -> Ev0W<RisSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt status for channel 1 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn ev1(&mut self) -> Ev1W<RisSpec> {
        Ev1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt status for channel 2 Event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn ev2(&mut self) -> Ev2W<RisSpec> {
        Ev2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw interrupt status for channel 3 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn ev3(&mut self) -> Ev3W<RisSpec> {
        Ev3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw interrupt status for channel 4 event. This bit is set to 1 when a CAPTURE or COMPARE event is received on channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn ev4(&mut self) -> Ev4W<RisSpec> {
        Ev4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw interrupt status for Systimer counter overflow event. This bit is set to 1 when an event is received on SysTimer Overflow occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ovfl(&mut self) -> OvflW<RisSpec> {
        OvflW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<RisSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
