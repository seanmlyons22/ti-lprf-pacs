#[doc = "Register `IMSET` reader"]
pub type R = crate::R<ImsetSpec>;
#[doc = "Register `IMSET` writer"]
pub type W = crate::W<ImsetSpec>;
#[doc = "0:0\\]
Set channel 0 event interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEff = 0,
}
impl From<Ev0> for bool {
    #[inline(always)]
    fn from(variant: Ev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - 0:0\\]
Set channel 0 event interrupt mask."]
pub type Ev0R = crate::BitReader<Ev0>;
impl Ev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0 {
        match self.bits {
            true => Ev0::Set,
            false => Ev0::NoEff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev0::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_eff(&self) -> bool {
        *self == Ev0::NoEff
    }
}
#[doc = "Field `EV0` writer - 0:0\\]
Set channel 0 event interrupt mask."]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Ev0>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_eff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::NoEff)
    }
}
#[doc = "1:1\\]
Set channel 1 event interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1 {
    #[doc = "1: Set interrupt mask"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    NoEff = 0,
}
impl From<Ev1> for bool {
    #[inline(always)]
    fn from(variant: Ev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - 1:1\\]
Set channel 1 event interrupt mask."]
pub type Ev1R = crate::BitReader<Ev1>;
impl Ev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1 {
        match self.bits {
            true => Ev1::Set,
            false => Ev1::NoEff,
        }
    }
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ev1::Set
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn is_no_eff(&self) -> bool {
        *self == Ev1::NoEff
    }
}
#[doc = "Field `EV1` writer - 1:1\\]
Set channel 1 event interrupt mask."]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Ev1>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt mask"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_eff(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::NoEff)
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
Set channel 0 event interrupt mask."]
    #[inline(always)]
    pub fn ev0(&self) -> Ev0R {
        Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set channel 1 event interrupt mask."]
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
Set channel 0 event interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev0(&mut self) -> Ev0W<ImsetSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set channel 1 event interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn ev1(&mut self) -> Ev1W<ImsetSpec> {
        Ev1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ImsetSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImsetSpec;
impl crate::RegisterSpec for ImsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imset::R`](R) reader structure"]
impl crate::Readable for ImsetSpec {}
#[doc = "`write(|w| ..)` method takes [`imset::W`](W) writer structure"]
impl crate::Writable for ImsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSET to value 0"]
impl crate::Resettable for ImsetSpec {
    const RESET_VALUE: u32 = 0;
}