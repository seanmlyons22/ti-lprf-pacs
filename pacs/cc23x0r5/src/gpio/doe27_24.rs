#[doc = "Register `DOE27_24` reader"]
pub type R = crate::R<Doe27_24Spec>;
#[doc = "Register `DOE27_24` writer"]
pub type W = crate::W<Doe27_24Spec>;
#[doc = "0:0\\]
Data output enable for DIO24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - 0:0\\]
Data output enable for DIO24"]
pub type Dio24R = crate::BitReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            true => Dio24::Enable,
            false => Dio24::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio24::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio24::Disable
    }
}
#[doc = "Field `DIO24` writer - 0:0\\]
Data output enable for DIO24"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Disable)
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "8:8\\]
Data output enable for DIO25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - 8:8\\]
Data output enable for DIO25"]
pub type Dio25R = crate::BitReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            true => Dio25::Enable,
            false => Dio25::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio25::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio25::Disable
    }
}
#[doc = "Field `DIO25` writer - 8:8\\]
Data output enable for DIO25"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Disable)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO24"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO25"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO24"]
    #[inline(always)]
    #[must_use]
    pub fn dio24(&mut self) -> Dio24W<Doe27_24Spec> {
        Dio24W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO25"]
    #[inline(always)]
    #[must_use]
    pub fn dio25(&mut self) -> Dio25W<Doe27_24Spec> {
        Dio25W::new(self, 8)
    }
}
#[doc = "Data out enable 27 to 24. Alias register for byte access to DOE31_0\\[27:24\\]
bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe27_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe27_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doe27_24Spec;
impl crate::RegisterSpec for Doe27_24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doe27_24::R`](R) reader structure"]
impl crate::Readable for Doe27_24Spec {}
#[doc = "`write(|w| ..)` method takes [`doe27_24::W`](W) writer structure"]
impl crate::Writable for Doe27_24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOE27_24 to value 0"]
impl crate::Resettable for Doe27_24Spec {
    const RESET_VALUE: u32 = 0;
}
