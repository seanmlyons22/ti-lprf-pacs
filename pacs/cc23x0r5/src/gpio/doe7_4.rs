#[doc = "Register `DOE7_4` reader"]
pub type R = crate::R<Doe7_4Spec>;
#[doc = "Register `DOE7_4` writer"]
pub type W = crate::W<Doe7_4Spec>;
#[doc = "0:0\\]
Data output enable for DIO4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` reader - 0:0\\]
Data output enable for DIO4"]
pub type Dio4R = crate::BitReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            true => Dio4::Enable,
            false => Dio4::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio4::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio4::Disable
    }
}
#[doc = "Field `DIO4` writer - 0:0\\]
Data output enable for DIO4"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Disable)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "8:8\\]
Data output enable for DIO5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` reader - 8:8\\]
Data output enable for DIO5"]
pub type Dio5R = crate::BitReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            true => Dio5::Enable,
            false => Dio5::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio5::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio5::Disable
    }
}
#[doc = "Field `DIO5` writer - 8:8\\]
Data output enable for DIO5"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Disable)
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "16:16\\]
Data output enable for DIO6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` reader - 16:16\\]
Data output enable for DIO6"]
pub type Dio6R = crate::BitReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            true => Dio6::Enable,
            false => Dio6::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio6::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio6::Disable
    }
}
#[doc = "Field `DIO6` writer - 16:16\\]
Data output enable for DIO6"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Disable)
    }
}
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "24:24\\]
Data output enable for DIO7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` reader - 24:24\\]
Data output enable for DIO7"]
pub type Dio7R = crate::BitReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            true => Dio7::Enable,
            false => Dio7::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio7::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio7::Disable
    }
}
#[doc = "Field `DIO7` writer - 24:24\\]
Data output enable for DIO7"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Disable)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO4"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO5"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output enable for DIO6"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output enable for DIO7"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 24) & 1) != 0)
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
Data output enable for DIO4"]
    #[inline(always)]
    #[must_use]
    pub fn dio4(&mut self) -> Dio4W<Doe7_4Spec> {
        Dio4W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO5"]
    #[inline(always)]
    #[must_use]
    pub fn dio5(&mut self) -> Dio5W<Doe7_4Spec> {
        Dio5W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output enable for DIO6"]
    #[inline(always)]
    #[must_use]
    pub fn dio6(&mut self) -> Dio6W<Doe7_4Spec> {
        Dio6W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output enable for DIO7"]
    #[inline(always)]
    #[must_use]
    pub fn dio7(&mut self) -> Dio7W<Doe7_4Spec> {
        Dio7W::new(self, 24)
    }
}
#[doc = "Data out enable 7 to 4. Alias register for byte access to DOE31_0\\[7:4\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe7_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe7_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doe7_4Spec;
impl crate::RegisterSpec for Doe7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doe7_4::R`](R) reader structure"]
impl crate::Readable for Doe7_4Spec {}
#[doc = "`write(|w| ..)` method takes [`doe7_4::W`](W) writer structure"]
impl crate::Writable for Doe7_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOE7_4 to value 0"]
impl crate::Resettable for Doe7_4Spec {
    const RESET_VALUE: u32 = 0;
}
