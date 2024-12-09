#[doc = "Register `DOUT19_16` reader"]
pub type R = crate::R<Dout19_16Spec>;
#[doc = "Register `DOUT19_16` writer"]
pub type W = crate::W<Dout19_16Spec>;
#[doc = "0:0\\]
Data output for DIO16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - 0:0\\]
Data output for DIO16"]
pub type Dio16R = crate::BitReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            true => Dio16::One,
            false => Dio16::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio16::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio16::Zero
    }
}
#[doc = "Field `DIO16` writer - 0:0\\]
Data output for DIO16"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Zero)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "8:8\\]
Data output for DIO17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - 8:8\\]
Data output for DIO17"]
pub type Dio17R = crate::BitReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            true => Dio17::One,
            false => Dio17::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio17::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio17::Zero
    }
}
#[doc = "Field `DIO17` writer - 8:8\\]
Data output for DIO17"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Zero)
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "16:16\\]
Data output for DIO18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - 16:16\\]
Data output for DIO18"]
pub type Dio18R = crate::BitReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            true => Dio18::One,
            false => Dio18::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio18::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio18::Zero
    }
}
#[doc = "Field `DIO18` writer - 16:16\\]
Data output for DIO18"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Zero)
    }
}
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "24:24\\]
Data output for DIO19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "1: Output is set to 1"]
    One = 1,
    #[doc = "0: Output is set to 0"]
    Zero = 0,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - 24:24\\]
Data output for DIO19"]
pub type Dio19R = crate::BitReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            true => Dio19::One,
            false => Dio19::Zero,
        }
    }
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dio19::One
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Dio19::Zero
    }
}
#[doc = "Field `DIO19` writer - 24:24\\]
Data output for DIO19"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set to 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::One)
    }
    #[doc = "Output is set to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Zero)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output for DIO16"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output for DIO17"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output for DIO18"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output for DIO19"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 24) & 1) != 0)
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
Data output for DIO16"]
    #[inline(always)]
    #[must_use]
    pub fn dio16(&mut self) -> Dio16W<Dout19_16Spec> {
        Dio16W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output for DIO17"]
    #[inline(always)]
    #[must_use]
    pub fn dio17(&mut self) -> Dio17W<Dout19_16Spec> {
        Dio17W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output for DIO18"]
    #[inline(always)]
    #[must_use]
    pub fn dio18(&mut self) -> Dio18W<Dout19_16Spec> {
        Dio18W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output for DIO19"]
    #[inline(always)]
    #[must_use]
    pub fn dio19(&mut self) -> Dio19W<Dout19_16Spec> {
        Dio19W::new(self, 24)
    }
}
#[doc = "Data out 19 to 16. Alias register for byte access to DOUT31_0\\[19:16\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout19_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout19_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout19_16Spec;
impl crate::RegisterSpec for Dout19_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout19_16::R`](R) reader structure"]
impl crate::Readable for Dout19_16Spec {}
#[doc = "`write(|w| ..)` method takes [`dout19_16::W`](W) writer structure"]
impl crate::Writable for Dout19_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT19_16 to value 0"]
impl crate::Resettable for Dout19_16Spec {
    const RESET_VALUE: u32 = 0;
}
