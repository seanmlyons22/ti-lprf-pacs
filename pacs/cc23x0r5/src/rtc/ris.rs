#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "0:0\\]
Raw interrupt status for Channel 0 event. This bit is set to 1 when a compare event occurs on Channel 0. This bit will be cleared. When the corresponding bit in ICLR.EV0 is set to 1. Or when a new compare value is written in CH0CC8U register\n\nValue on reset: 0"]
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
Raw interrupt status for Channel 0 event. This bit is set to 1 when a compare event occurs on Channel 0. This bit will be cleared. When the corresponding bit in ICLR.EV0 is set to 1. Or when a new compare value is written in CH0CC8U register"]
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
Raw interrupt status for Channel 0 event. This bit is set to 1 when a compare event occurs on Channel 0. This bit will be cleared. When the corresponding bit in ICLR.EV0 is set to 1. Or when a new compare value is written in CH0CC8U register"]
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
Raw interrupt status for Channel 1 event. This bit is set to 1 when a capture event is received on Channel 1. This bit will be cleared when the bit in ICLR.EV1 is set to 1 or when the captured time value is read from the CH1CC8U register.\n\nValue on reset: 0"]
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
Raw interrupt status for Channel 1 event. This bit is set to 1 when a capture event is received on Channel 1. This bit will be cleared when the bit in ICLR.EV1 is set to 1 or when the captured time value is read from the CH1CC8U register."]
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
Raw interrupt status for Channel 1 event. This bit is set to 1 when a capture event is received on Channel 1. This bit will be cleared when the bit in ICLR.EV1 is set to 1 or when the captured time value is read from the CH1CC8U register."]
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
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt status for Channel 0 event. This bit is set to 1 when a compare event occurs on Channel 0. This bit will be cleared. When the corresponding bit in ICLR.EV0 is set to 1. Or when a new compare value is written in CH0CC8U register"]
    #[inline(always)]
    pub fn ev0(&self) -> Ev0R {
        Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt status for Channel 1 event. This bit is set to 1 when a capture event is received on Channel 1. This bit will be cleared when the bit in ICLR.EV1 is set to 1 or when the captured time value is read from the CH1CC8U register."]
    #[inline(always)]
    pub fn ev1(&self) -> Ev1R {
        Ev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt status for Channel 0 event. This bit is set to 1 when a compare event occurs on Channel 0. This bit will be cleared. When the corresponding bit in ICLR.EV0 is set to 1. Or when a new compare value is written in CH0CC8U register"]
    #[inline(always)]
    #[must_use]
    pub fn ev0(&mut self) -> Ev0W<RisSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt status for Channel 1 event. This bit is set to 1 when a capture event is received on Channel 1. This bit will be cleared when the bit in ICLR.EV1 is set to 1 or when the captured time value is read from the CH1CC8U register."]
    #[inline(always)]
    #[must_use]
    pub fn ev1(&mut self) -> Ev1W<RisSpec> {
        Ev1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<RisSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
