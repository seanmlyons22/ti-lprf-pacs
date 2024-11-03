#[doc = "Register `OUT` reader"]
pub type R = crate::R<OutSpec>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OutSpec>;
#[doc = "0:0\\]
Output Value of channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out0 {
    #[doc = "1: Event occured"]
    Set = 1,
    #[doc = "0: Event did not occur."]
    Clr = 0,
}
impl From<Out0> for bool {
    #[inline(always)]
    fn from(variant: Out0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT0` reader - 0:0\\]
Output Value of channel 0."]
pub type Out0R = crate::BitReader<Out0>;
impl Out0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out0 {
        match self.bits {
            true => Out0::Set,
            false => Out0::Clr,
        }
    }
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Out0::Set
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Out0::Clr
    }
}
#[doc = "Field `OUT0` writer - 0:0\\]
Output Value of channel 0."]
pub type Out0W<'a, REG> = crate::BitWriter<'a, REG, Out0>;
impl<'a, REG> Out0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Set)
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Clr)
    }
}
#[doc = "1:1\\]
Output Value of channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out1 {
    #[doc = "1: Event occured"]
    Set = 1,
    #[doc = "0: Event did not occur."]
    Clr = 0,
}
impl From<Out1> for bool {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT1` reader - 1:1\\]
Output Value of channel 1."]
pub type Out1R = crate::BitReader<Out1>;
impl Out1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out1 {
        match self.bits {
            true => Out1::Set,
            false => Out1::Clr,
        }
    }
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Out1::Set
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Out1::Clr
    }
}
#[doc = "Field `OUT1` writer - 1:1\\]
Output Value of channel 1."]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG, Out1>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Set)
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Clr)
    }
}
#[doc = "2:2\\]
Output Value of channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out2 {
    #[doc = "1: Event occured"]
    Set = 1,
    #[doc = "0: Event did not occur."]
    Clr = 0,
}
impl From<Out2> for bool {
    #[inline(always)]
    fn from(variant: Out2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT2` reader - 2:2\\]
Output Value of channel 2."]
pub type Out2R = crate::BitReader<Out2>;
impl Out2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out2 {
        match self.bits {
            true => Out2::Set,
            false => Out2::Clr,
        }
    }
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Out2::Set
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Out2::Clr
    }
}
#[doc = "Field `OUT2` writer - 2:2\\]
Output Value of channel 2."]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG, Out2>;
impl<'a, REG> Out2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Set)
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Clr)
    }
}
#[doc = "3:3\\]
Output Value of channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out3 {
    #[doc = "1: Event occured"]
    Set = 1,
    #[doc = "0: Event did not occur."]
    Clr = 0,
}
impl From<Out3> for bool {
    #[inline(always)]
    fn from(variant: Out3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT3` reader - 3:3\\]
Output Value of channel 3."]
pub type Out3R = crate::BitReader<Out3>;
impl Out3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out3 {
        match self.bits {
            true => Out3::Set,
            false => Out3::Clr,
        }
    }
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Out3::Set
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Out3::Clr
    }
}
#[doc = "Field `OUT3` writer - 3:3\\]
Output Value of channel 3."]
pub type Out3W<'a, REG> = crate::BitWriter<'a, REG, Out3>;
impl<'a, REG> Out3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Out3::Set)
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Out3::Clr)
    }
}
#[doc = "4:4\\]
Output Value of channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out4 {
    #[doc = "1: Event occured"]
    Set = 1,
    #[doc = "0: Event did not occur."]
    Clr = 0,
}
impl From<Out4> for bool {
    #[inline(always)]
    fn from(variant: Out4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT4` reader - 4:4\\]
Output Value of channel 4."]
pub type Out4R = crate::BitReader<Out4>;
impl Out4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out4 {
        match self.bits {
            true => Out4::Set,
            false => Out4::Clr,
        }
    }
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Out4::Set
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Out4::Clr
    }
}
#[doc = "Field `OUT4` writer - 4:4\\]
Output Value of channel 4."]
pub type Out4W<'a, REG> = crate::BitWriter<'a, REG, Out4>;
impl<'a, REG> Out4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event occured"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Out4::Set)
    }
    #[doc = "Event did not occur."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Out4::Clr)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Output Value of channel 0."]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Output Value of channel 1."]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Output Value of channel 2."]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Output Value of channel 3."]
    #[inline(always)]
    pub fn out3(&self) -> Out3R {
        Out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Output Value of channel 4."]
    #[inline(always)]
    pub fn out4(&self) -> Out4R {
        Out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Output Value of channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<OutSpec> {
        Out0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Output Value of channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<OutSpec> {
        Out1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Output Value of channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<OutSpec> {
        Out2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Output Value of channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn out3(&mut self) -> Out3W<OutSpec> {
        Out3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Output Value of channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn out4(&mut self) -> Out4W<OutSpec> {
        Out4W::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<OutSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "Systimer's channel Output Event Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSpec;
impl crate::RegisterSpec for OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OutSpec {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OutSpec {
    const RESET_VALUE: u32 = 0;
}
