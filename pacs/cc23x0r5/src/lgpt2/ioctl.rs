#[doc = "Register `IOCTL` reader"]
pub type R = crate::R<IoctlSpec>;
#[doc = "Register `IOCTL` writer"]
pub type W = crate::W<IoctlSpec>;
#[doc = "1:0\\]
IO output 0 control This bit field controls IO output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out0 {
    #[doc = "3: Inverted value. The IO output is inverted."]
    Inv = 3,
    #[doc = "2: Driven high. The IO output is driven high."]
    High = 2,
    #[doc = "1: Driven low. The IO output is driven low."]
    Low = 1,
    #[doc = "0: Normal output. The IO output is not changed."]
    Nrm = 0,
}
impl From<Out0> for u8 {
    #[inline(always)]
    fn from(variant: Out0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out0 {
    type Ux = u8;
}
impl crate::IsEnum for Out0 {}
#[doc = "Field `OUT0` reader - 1:0\\]
IO output 0 control This bit field controls IO output 0."]
pub type Out0R = crate::FieldReader<Out0>;
impl Out0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out0 {
        match self.bits {
            3 => Out0::Inv,
            2 => Out0::High,
            1 => Out0::Low,
            0 => Out0::Nrm,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverted value. The IO output is inverted."]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Out0::Inv
    }
    #[doc = "Driven high. The IO output is driven high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Out0::High
    }
    #[doc = "Driven low. The IO output is driven low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Out0::Low
    }
    #[doc = "Normal output. The IO output is not changed."]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == Out0::Nrm
    }
}
#[doc = "Field `OUT0` writer - 1:0\\]
IO output 0 control This bit field controls IO output 0."]
pub type Out0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Out0, crate::Safe>;
impl<'a, REG> Out0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverted value. The IO output is inverted."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Inv)
    }
    #[doc = "Driven high. The IO output is driven high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::High)
    }
    #[doc = "Driven low. The IO output is driven low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Low)
    }
    #[doc = "Normal output. The IO output is not changed."]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Nrm)
    }
}
#[doc = "3:2\\]
IO complementary output 0 control This bit field controls IO complementary output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cout0 {
    #[doc = "3: Inverted value. The IO complementary output is inverted."]
    Inv = 3,
    #[doc = "2: Driven high. The IO complementary output is driven high."]
    High = 2,
    #[doc = "1: Driven low. The IO complementary output is driven low."]
    Low = 1,
    #[doc = "0: Normal output. The IO complementary output is not changed."]
    Nrm = 0,
}
impl From<Cout0> for u8 {
    #[inline(always)]
    fn from(variant: Cout0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cout0 {
    type Ux = u8;
}
impl crate::IsEnum for Cout0 {}
#[doc = "Field `COUT0` reader - 3:2\\]
IO complementary output 0 control This bit field controls IO complementary output 0."]
pub type Cout0R = crate::FieldReader<Cout0>;
impl Cout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cout0 {
        match self.bits {
            3 => Cout0::Inv,
            2 => Cout0::High,
            1 => Cout0::Low,
            0 => Cout0::Nrm,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverted value. The IO complementary output is inverted."]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Cout0::Inv
    }
    #[doc = "Driven high. The IO complementary output is driven high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cout0::High
    }
    #[doc = "Driven low. The IO complementary output is driven low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cout0::Low
    }
    #[doc = "Normal output. The IO complementary output is not changed."]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == Cout0::Nrm
    }
}
#[doc = "Field `COUT0` writer - 3:2\\]
IO complementary output 0 control This bit field controls IO complementary output 0."]
pub type Cout0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cout0, crate::Safe>;
impl<'a, REG> Cout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverted value. The IO complementary output is inverted."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Cout0::Inv)
    }
    #[doc = "Driven high. The IO complementary output is driven high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cout0::High)
    }
    #[doc = "Driven low. The IO complementary output is driven low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cout0::Low)
    }
    #[doc = "Normal output. The IO complementary output is not changed."]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut crate::W<REG> {
        self.variant(Cout0::Nrm)
    }
}
#[doc = "5:4\\]
IO output 1 control This bit field controls IO output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out1 {
    #[doc = "3: Inverted value. The IO output is inverted."]
    Inv = 3,
    #[doc = "2: Driven high. The IO output is driven high."]
    High = 2,
    #[doc = "1: Driven low. The IO output is driven low."]
    Low = 1,
    #[doc = "0: Normal output. The IO output is not changed."]
    Nrm = 0,
}
impl From<Out1> for u8 {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out1 {
    type Ux = u8;
}
impl crate::IsEnum for Out1 {}
#[doc = "Field `OUT1` reader - 5:4\\]
IO output 1 control This bit field controls IO output 1."]
pub type Out1R = crate::FieldReader<Out1>;
impl Out1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out1 {
        match self.bits {
            3 => Out1::Inv,
            2 => Out1::High,
            1 => Out1::Low,
            0 => Out1::Nrm,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverted value. The IO output is inverted."]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Out1::Inv
    }
    #[doc = "Driven high. The IO output is driven high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Out1::High
    }
    #[doc = "Driven low. The IO output is driven low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Out1::Low
    }
    #[doc = "Normal output. The IO output is not changed."]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == Out1::Nrm
    }
}
#[doc = "Field `OUT1` writer - 5:4\\]
IO output 1 control This bit field controls IO output 1."]
pub type Out1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Out1, crate::Safe>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverted value. The IO output is inverted."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Inv)
    }
    #[doc = "Driven high. The IO output is driven high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::High)
    }
    #[doc = "Driven low. The IO output is driven low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Low)
    }
    #[doc = "Normal output. The IO output is not changed."]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Nrm)
    }
}
#[doc = "7:6\\]
IO complementary output 1 control This bit field controls IO complementary output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cout1 {
    #[doc = "3: Inverted value. The IO complementary output is inverted."]
    Inv = 3,
    #[doc = "2: Driven high. The IO complementary output is driven high."]
    High = 2,
    #[doc = "1: Driven low. The IO complementary output is driven low."]
    Low = 1,
    #[doc = "0: Normal output. The IO complementary output is not changed."]
    Nrm = 0,
}
impl From<Cout1> for u8 {
    #[inline(always)]
    fn from(variant: Cout1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cout1 {
    type Ux = u8;
}
impl crate::IsEnum for Cout1 {}
#[doc = "Field `COUT1` reader - 7:6\\]
IO complementary output 1 control This bit field controls IO complementary output 1."]
pub type Cout1R = crate::FieldReader<Cout1>;
impl Cout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cout1 {
        match self.bits {
            3 => Cout1::Inv,
            2 => Cout1::High,
            1 => Cout1::Low,
            0 => Cout1::Nrm,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverted value. The IO complementary output is inverted."]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Cout1::Inv
    }
    #[doc = "Driven high. The IO complementary output is driven high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cout1::High
    }
    #[doc = "Driven low. The IO complementary output is driven low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cout1::Low
    }
    #[doc = "Normal output. The IO complementary output is not changed."]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == Cout1::Nrm
    }
}
#[doc = "Field `COUT1` writer - 7:6\\]
IO complementary output 1 control This bit field controls IO complementary output 1."]
pub type Cout1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cout1, crate::Safe>;
impl<'a, REG> Cout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverted value. The IO complementary output is inverted."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Cout1::Inv)
    }
    #[doc = "Driven high. The IO complementary output is driven high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cout1::High)
    }
    #[doc = "Driven low. The IO complementary output is driven low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cout1::Low)
    }
    #[doc = "Normal output. The IO complementary output is not changed."]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut crate::W<REG> {
        self.variant(Cout1::Nrm)
    }
}
#[doc = "9:8\\]
IO output 2 control This bit field controls IO output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Out2 {
    #[doc = "3: Inverted value. The IO output is inverted."]
    Inv = 3,
    #[doc = "2: Driven high. The IO output is driven high."]
    High = 2,
    #[doc = "1: Driven low. The IO output is driven low."]
    Low = 1,
    #[doc = "0: Normal output. The IO output is not changed."]
    Nrm = 0,
}
impl From<Out2> for u8 {
    #[inline(always)]
    fn from(variant: Out2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Out2 {
    type Ux = u8;
}
impl crate::IsEnum for Out2 {}
#[doc = "Field `OUT2` reader - 9:8\\]
IO output 2 control This bit field controls IO output 2."]
pub type Out2R = crate::FieldReader<Out2>;
impl Out2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out2 {
        match self.bits {
            3 => Out2::Inv,
            2 => Out2::High,
            1 => Out2::Low,
            0 => Out2::Nrm,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverted value. The IO output is inverted."]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Out2::Inv
    }
    #[doc = "Driven high. The IO output is driven high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Out2::High
    }
    #[doc = "Driven low. The IO output is driven low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Out2::Low
    }
    #[doc = "Normal output. The IO output is not changed."]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == Out2::Nrm
    }
}
#[doc = "Field `OUT2` writer - 9:8\\]
IO output 2 control This bit field controls IO output 2."]
pub type Out2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Out2, crate::Safe>;
impl<'a, REG> Out2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverted value. The IO output is inverted."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Inv)
    }
    #[doc = "Driven high. The IO output is driven high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::High)
    }
    #[doc = "Driven low. The IO output is driven low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Low)
    }
    #[doc = "Normal output. The IO output is not changed."]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Nrm)
    }
}
#[doc = "11:10\\]
IO complementary output 2 control This bit field controls IO complementary output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cout2 {
    #[doc = "3: Inverted value. The IO complementary output is inverted."]
    Inv = 3,
    #[doc = "2: Driven high. The IO complementary output is driven high."]
    High = 2,
    #[doc = "1: Driven low. The IO complementary output is driven low."]
    Low = 1,
    #[doc = "0: Normal output. The IO complementary output is not changed."]
    Nrm = 0,
}
impl From<Cout2> for u8 {
    #[inline(always)]
    fn from(variant: Cout2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cout2 {
    type Ux = u8;
}
impl crate::IsEnum for Cout2 {}
#[doc = "Field `COUT2` reader - 11:10\\]
IO complementary output 2 control This bit field controls IO complementary output 2."]
pub type Cout2R = crate::FieldReader<Cout2>;
impl Cout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cout2 {
        match self.bits {
            3 => Cout2::Inv,
            2 => Cout2::High,
            1 => Cout2::Low,
            0 => Cout2::Nrm,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverted value. The IO complementary output is inverted."]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Cout2::Inv
    }
    #[doc = "Driven high. The IO complementary output is driven high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cout2::High
    }
    #[doc = "Driven low. The IO complementary output is driven low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cout2::Low
    }
    #[doc = "Normal output. The IO complementary output is not changed."]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == Cout2::Nrm
    }
}
#[doc = "Field `COUT2` writer - 11:10\\]
IO complementary output 2 control This bit field controls IO complementary output 2."]
pub type Cout2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cout2, crate::Safe>;
impl<'a, REG> Cout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverted value. The IO complementary output is inverted."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Cout2::Inv)
    }
    #[doc = "Driven high. The IO complementary output is driven high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cout2::High)
    }
    #[doc = "Driven low. The IO complementary output is driven low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cout2::Low)
    }
    #[doc = "Normal output. The IO complementary output is not changed."]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut crate::W<REG> {
        self.variant(Cout2::Nrm)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
IO output 0 control This bit field controls IO output 0."]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
IO complementary output 0 control This bit field controls IO complementary output 0."]
    #[inline(always)]
    pub fn cout0(&self) -> Cout0R {
        Cout0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
IO output 1 control This bit field controls IO output 1."]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
IO complementary output 1 control This bit field controls IO complementary output 1."]
    #[inline(always)]
    pub fn cout1(&self) -> Cout1R {
        Cout1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
IO output 2 control This bit field controls IO output 2."]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
IO complementary output 2 control This bit field controls IO complementary output 2."]
    #[inline(always)]
    pub fn cout2(&self) -> Cout2R {
        Cout2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
IO output 0 control This bit field controls IO output 0."]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<IoctlSpec> {
        Out0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
IO complementary output 0 control This bit field controls IO complementary output 0."]
    #[inline(always)]
    #[must_use]
    pub fn cout0(&mut self) -> Cout0W<IoctlSpec> {
        Cout0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
IO output 1 control This bit field controls IO output 1."]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<IoctlSpec> {
        Out1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
IO complementary output 1 control This bit field controls IO complementary output 1."]
    #[inline(always)]
    #[must_use]
    pub fn cout1(&mut self) -> Cout1W<IoctlSpec> {
        Cout1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
IO output 2 control This bit field controls IO output 2."]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<IoctlSpec> {
        Out2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
IO complementary output 2 control This bit field controls IO complementary output 2."]
    #[inline(always)]
    #[must_use]
    pub fn cout2(&mut self) -> Cout2W<IoctlSpec> {
        Cout2W::new(self, 10)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<IoctlSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "IO Controller This register controls the IO outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoctlSpec;
impl crate::RegisterSpec for IoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioctl::R`](R) reader structure"]
impl crate::Readable for IoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ioctl::W`](W) writer structure"]
impl crate::Writable for IoctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCTL to value 0"]
impl crate::Resettable for IoctlSpec {
    const RESET_VALUE: u32 = 0;
}
