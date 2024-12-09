#[doc = "Register `IMCLR` reader"]
pub type R = crate::R<ImclrSpec>;
#[doc = "Register `IMCLR` writer"]
pub type W = crate::W<ImclrSpec>;
#[doc = "0:0\\]
Clears channel0 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev0> for bool {
    #[inline(always)]
    fn from(variant: Ev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` writer - 0:0\\]
Clears channel0 Event Interrupt Mask."]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Ev0>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Noeff)
    }
}
#[doc = "1:1\\]
Clears channel1 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev1> for bool {
    #[inline(always)]
    fn from(variant: Ev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` writer - 1:1\\]
Clears channel1 Event Interrupt Mask."]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Ev1>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Noeff)
    }
}
#[doc = "2:2\\]
Clears channel2 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev2 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev2> for bool {
    #[inline(always)]
    fn from(variant: Ev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2` writer - 2:2\\]
Clears channel2 Event Interrupt Mask."]
pub type Ev2W<'a, REG> = crate::BitWriter<'a, REG, Ev2>;
impl<'a, REG> Ev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::Noeff)
    }
}
#[doc = "3:3\\]
Clears channel3 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev3 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev3> for bool {
    #[inline(always)]
    fn from(variant: Ev3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV3` writer - 3:3\\]
Clears channel3 Event Interrupt Mask."]
pub type Ev3W<'a, REG> = crate::BitWriter<'a, REG, Ev3>;
impl<'a, REG> Ev3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev3::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev3::Noeff)
    }
}
#[doc = "4:4\\]
Clears channel4 Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev4 {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ev4> for bool {
    #[inline(always)]
    fn from(variant: Ev4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV4` writer - 4:4\\]
Clears channel4 Event Interrupt Mask."]
pub type Ev4W<'a, REG> = crate::BitWriter<'a, REG, Ev4>;
impl<'a, REG> Ev4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ev4::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev4::Noeff)
    }
}
#[doc = "5:5\\]
Clears Timer Overflow Event Interrupt Mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfl {
    #[doc = "1: Clear interrupt mask"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ovfl> for bool {
    #[inline(always)]
    fn from(variant: Ovfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFL` writer - 5:5\\]
Clears Timer Overflow Event Interrupt Mask."]
pub type OvflW<'a, REG> = crate::BitWriter<'a, REG, Ovfl>;
impl<'a, REG> OvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt mask"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ovfl::Clr)
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
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears channel0 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev0(&mut self) -> Ev0W<ImclrSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears channel1 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev1(&mut self) -> Ev1W<ImclrSpec> {
        Ev1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears channel2 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev2(&mut self) -> Ev2W<ImclrSpec> {
        Ev2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears channel3 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev3(&mut self) -> Ev3W<ImclrSpec> {
        Ev3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clears channel4 Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev4(&mut self) -> Ev4W<ImclrSpec> {
        Ev4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Clears Timer Overflow Event Interrupt Mask."]
    #[inline(always)]
    #[must_use]
    pub fn ovfl(&mut self) -> OvflW<ImclrSpec> {
        OvflW::new(self, 5)
    }
}
#[doc = "Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImclrSpec;
impl crate::RegisterSpec for ImclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imclr::R`](R) reader structure"]
impl crate::Readable for ImclrSpec {}
#[doc = "`write(|w| ..)` method takes [`imclr::W`](W) writer structure"]
impl crate::Writable for ImclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMCLR to value 0"]
impl crate::Resettable for ImclrSpec {
    const RESET_VALUE: u32 = 0;
}
