#[doc = "Register `DOUT3_0` reader"]
pub type R = crate::R<Dout3_0Spec>;
#[doc = "Register `DOUT3_0` writer"]
pub type W = crate::W<Dout3_0Spec>;
#[doc = "0:0\\]
Data output for DIO0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` reader - 0:0\\]
Data output for DIO0"]
pub type Dio0R = crate::BitReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            true => Dio0::One,
            false => Dio0::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio0::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio0::Zero
    }
}
#[doc = "Field `DIO0` writer - 0:0\\]
Data output for DIO0"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Zero)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "8:8\\]
Data output for DIO1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` reader - 8:8\\]
Data output for DIO1"]
pub type Dio1R = crate::BitReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            true => Dio1::One,
            false => Dio1::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio1::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio1::Zero
    }
}
#[doc = "Field `DIO1` writer - 8:8\\]
Data output for DIO1"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Zero)
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "16:16\\]
Data output for DIO2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` reader - 16:16\\]
Data output for DIO2"]
pub type Dio2R = crate::BitReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            true => Dio2::One,
            false => Dio2::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio2::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio2::Zero
    }
}
#[doc = "Field `DIO2` writer - 16:16\\]
Data output for DIO2"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Zero)
    }
}
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "24:24\\]
Data output for DIO3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` reader - 24:24\\]
Data output for DIO3"]
pub type Dio3R = crate::BitReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            true => Dio3::One,
            false => Dio3::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio3::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio3::Zero
    }
}
#[doc = "Field `DIO3` writer - 24:24\\]
Data output for DIO3"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Zero)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output for DIO0"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output for DIO1"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output for DIO2"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output for DIO3"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data output for DIO0"]
    #[inline(always)]
    #[must_use]
    pub fn dio0(&mut self) -> Dio0W<Dout3_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output for DIO1"]
    #[inline(always)]
    #[must_use]
    pub fn dio1(&mut self) -> Dio1W<Dout3_0Spec> {
        Dio1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output for DIO2"]
    #[inline(always)]
    #[must_use]
    pub fn dio2(&mut self) -> Dio2W<Dout3_0Spec> {
        Dio2W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output for DIO3"]
    #[inline(always)]
    #[must_use]
    pub fn dio3(&mut self) -> Dio3W<Dout3_0Spec> {
        Dio3W::new(self, 24)
    }
}
#[doc = "Data out 3 to 0. Alias register for byte access to DOUT31_0\\[3:0\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout3_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout3_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout3_0Spec;
impl crate::RegisterSpec for Dout3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout3_0::R`](R) reader structure"]
impl crate::Readable for Dout3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`dout3_0::W`](W) writer structure"]
impl crate::Writable for Dout3_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT3_0 to value 0"]
impl crate::Resettable for Dout3_0Spec {
    const RESET_VALUE: u32 = 0;
}
