#[doc = "Register `DOE23_20` reader"]
pub type R = crate::R<Doe23_20Spec>;
#[doc = "Register `DOE23_20` writer"]
pub type W = crate::W<Doe23_20Spec>;
#[doc = "0:0\\]
Data output enable for DIO20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - 0:0\\]
Data output enable for DIO20"]
pub type Dio20R = crate::BitReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            true => Dio20::Enable,
            false => Dio20::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio20::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio20::Disable
    }
}
#[doc = "Field `DIO20` writer - 0:0\\]
Data output enable for DIO20"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Disable)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "8:8\\]
Data output enable for DIO21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - 8:8\\]
Data output enable for DIO21"]
pub type Dio21R = crate::BitReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            true => Dio21::Enable,
            false => Dio21::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio21::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio21::Disable
    }
}
#[doc = "Field `DIO21` writer - 8:8\\]
Data output enable for DIO21"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Disable)
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "16:16\\]
Data output enable for DIO22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - 16:16\\]
Data output enable for DIO22"]
pub type Dio22R = crate::BitReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            true => Dio22::Enable,
            false => Dio22::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio22::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio22::Disable
    }
}
#[doc = "Field `DIO22` writer - 16:16\\]
Data output enable for DIO22"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Disable)
    }
}
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "24:24\\]
Data output enable for DIO23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - 24:24\\]
Data output enable for DIO23"]
pub type Dio23R = crate::BitReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            true => Dio23::Enable,
            false => Dio23::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio23::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio23::Disable
    }
}
#[doc = "Field `DIO23` writer - 24:24\\]
Data output enable for DIO23"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Disable)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO20"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO21"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output enable for DIO22"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output enable for DIO23"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 24) & 1) != 0)
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
Data output enable for DIO20"]
    #[inline(always)]
    #[must_use]
    pub fn dio20(&mut self) -> Dio20W<Doe23_20Spec> {
        Dio20W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO21"]
    #[inline(always)]
    #[must_use]
    pub fn dio21(&mut self) -> Dio21W<Doe23_20Spec> {
        Dio21W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output enable for DIO22"]
    #[inline(always)]
    #[must_use]
    pub fn dio22(&mut self) -> Dio22W<Doe23_20Spec> {
        Dio22W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output enable for DIO23"]
    #[inline(always)]
    #[must_use]
    pub fn dio23(&mut self) -> Dio23W<Doe23_20Spec> {
        Dio23W::new(self, 24)
    }
}
#[doc = "Data out enable 23 to 20. Alias register for byte access to DOE31_0\\[23:20\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe23_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe23_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doe23_20Spec;
impl crate::RegisterSpec for Doe23_20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doe23_20::R`](R) reader structure"]
impl crate::Readable for Doe23_20Spec {}
#[doc = "`write(|w| ..)` method takes [`doe23_20::W`](W) writer structure"]
impl crate::Writable for Doe23_20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOE23_20 to value 0"]
impl crate::Resettable for Doe23_20Spec {
    const RESET_VALUE: u32 = 0;
}
