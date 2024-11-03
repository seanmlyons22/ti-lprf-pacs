#[doc = "Register `IOSEGSET` reader"]
pub type R = crate::R<IosegsetSpec>;
#[doc = "Register `IOSEGSET` writer"]
pub type W = crate::W<IosegsetSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdds2 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    En = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Vdds2> for bool {
    #[inline(always)]
    fn from(variant: Vdds2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDS2` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Vdds2R = crate::BitReader<Vdds2>;
impl Vdds2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdds2 {
        match self.bits {
            true => Vdds2::En,
            false => Vdds2::Noeff,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vdds2::En
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Vdds2::Noeff
    }
}
#[doc = "Field `VDDS2` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Vdds2W<'a, REG> = crate::BitWriter<'a, REG, Vdds2>;
impl<'a, REG> Vdds2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vdds2::En)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Vdds2::Noeff)
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdds3 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    En = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Vdds3> for bool {
    #[inline(always)]
    fn from(variant: Vdds3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDS3` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Vdds3R = crate::BitReader<Vdds3>;
impl Vdds3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdds3 {
        match self.bits {
            true => Vdds3::En,
            false => Vdds3::Noeff,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Vdds3::En
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Vdds3::Noeff
    }
}
#[doc = "Field `VDDS3` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Vdds3W<'a, REG> = crate::BitWriter<'a, REG, Vdds3>;
impl<'a, REG> Vdds3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Vdds3::En)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Vdds3::Noeff)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds2(&self) -> Vdds2R {
        Vdds2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds3(&self) -> Vdds3R {
        Vdds3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vdds2(&mut self) -> Vdds2W<IosegsetSpec> {
        Vdds2W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vdds3(&mut self) -> Vdds3W<IosegsetSpec> {
        Vdds3W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IosegsetSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iosegset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iosegset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IosegsetSpec;
impl crate::RegisterSpec for IosegsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iosegset::R`](R) reader structure"]
impl crate::Readable for IosegsetSpec {}
#[doc = "`write(|w| ..)` method takes [`iosegset::W`](W) writer structure"]
impl crate::Writable for IosegsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOSEGSET to value 0"]
impl crate::Resettable for IosegsetSpec {
    const RESET_VALUE: u32 = 0;
}
