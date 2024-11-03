#[doc = "Register `DBCTL` reader"]
pub type R = crate::R<DbctlSpec>;
#[doc = "Register `DBCTL` writer"]
pub type W = crate::W<DbctlSpec>;
#[doc = "0:0\\]
Enable dead band on IO and IO complementary output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Io0 {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<Io0> for bool {
    #[inline(always)]
    fn from(variant: Io0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO0` reader - 0:0\\]
Enable dead band on IO and IO complementary output 0."]
pub type Io0R = crate::BitReader<Io0>;
impl Io0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io0 {
        match self.bits {
            true => Io0::En,
            false => Io0::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Io0::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Io0::Dis
    }
}
#[doc = "Field `IO0` writer - 0:0\\]
Enable dead band on IO and IO complementary output 0."]
pub type Io0W<'a, REG> = crate::BitWriter<'a, REG, Io0>;
impl<'a, REG> Io0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Io0::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Io0::Dis)
    }
}
#[doc = "1:1\\]
Enable dead band on IO and IO complementary output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Io1 {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<Io1> for bool {
    #[inline(always)]
    fn from(variant: Io1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO1` reader - 1:1\\]
Enable dead band on IO and IO complementary output 1."]
pub type Io1R = crate::BitReader<Io1>;
impl Io1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io1 {
        match self.bits {
            true => Io1::En,
            false => Io1::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Io1::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Io1::Dis
    }
}
#[doc = "Field `IO1` writer - 1:1\\]
Enable dead band on IO and IO complementary output 1."]
pub type Io1W<'a, REG> = crate::BitWriter<'a, REG, Io1>;
impl<'a, REG> Io1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Io1::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Io1::Dis)
    }
}
#[doc = "2:2\\]
Enable dead band on IO and IO complementary output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Io2 {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<Io2> for bool {
    #[inline(always)]
    fn from(variant: Io2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO2` reader - 2:2\\]
Enable dead band on IO and IO complementary output 2."]
pub type Io2R = crate::BitReader<Io2>;
impl Io2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io2 {
        match self.bits {
            true => Io2::En,
            false => Io2::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Io2::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Io2::Dis
    }
}
#[doc = "Field `IO2` writer - 2:2\\]
Enable dead band on IO and IO complementary output 2."]
pub type Io2W<'a, REG> = crate::BitWriter<'a, REG, Io2>;
impl<'a, REG> Io2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Io2::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Io2::Dis)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable dead band on IO and IO complementary output 0."]
    #[inline(always)]
    pub fn io0(&self) -> Io0R {
        Io0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable dead band on IO and IO complementary output 1."]
    #[inline(always)]
    pub fn io1(&self) -> Io1R {
        Io1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable dead band on IO and IO complementary output 2."]
    #[inline(always)]
    pub fn io2(&self) -> Io2R {
        Io2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable dead band on IO and IO complementary output 0."]
    #[inline(always)]
    #[must_use]
    pub fn io0(&mut self) -> Io0W<DbctlSpec> {
        Io0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable dead band on IO and IO complementary output 1."]
    #[inline(always)]
    #[must_use]
    pub fn io1(&mut self) -> Io1W<DbctlSpec> {
        Io1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable dead band on IO and IO complementary output 2."]
    #[inline(always)]
    #[must_use]
    pub fn io2(&mut self) -> Io2W<DbctlSpec> {
        Io2W::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<DbctlSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Dead Band Control This register is used to enable dead band for IOC outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbctlSpec;
impl crate::RegisterSpec for DbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbctl::R`](R) reader structure"]
impl crate::Readable for DbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbctl::W`](W) writer structure"]
impl crate::Writable for DbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBCTL to value 0"]
impl crate::Resettable for DbctlSpec {
    const RESET_VALUE: u32 = 0;
}
