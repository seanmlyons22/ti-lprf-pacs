#[doc = "Register `PARTID` reader"]
pub type R = crate::R<PartidSpec>;
#[doc = "Register `PARTID` writer"]
pub type W = crate::W<PartidSpec>;
#[doc = "Field `PART` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type PartR = crate::FieldReader<u16>;
#[doc = "Field `PART` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type PartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VARIANT` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type VariantR = crate::FieldReader;
#[doc = "Field `VARIANT` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type VariantW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MINORREV` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type MinorrevR = crate::FieldReader;
#[doc = "Field `MINORREV` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type MinorrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJORREV` reader - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type MajorrevR = crate::FieldReader;
#[doc = "Field `MAJORREV` writer - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type MajorrevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "31:31\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Set = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Clr = 0,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            true => Start::Set,
            false => Start::Clr,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Start::Set
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Start::Clr
    }
}
#[doc = "Field `START` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Set)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Clr)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn part(&self) -> PartR {
        PartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn minorrev(&self) -> MinorrevR {
        MinorrevR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn majorrev(&self) -> MajorrevR {
        MajorrevR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn part(&mut self) -> PartW<PartidSpec> {
        PartW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn variant(&mut self) -> VariantW<PartidSpec> {
        VariantW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn minorrev(&mut self) -> MinorrevW<PartidSpec> {
        MinorrevW::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn majorrev(&mut self) -> MajorrevW<PartidSpec> {
        MajorrevW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<PartidSpec> {
        StartW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`partid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`partid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartidSpec;
impl crate::RegisterSpec for PartidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`partid::R`](R) reader structure"]
impl crate::Readable for PartidSpec {}
#[doc = "`write(|w| ..)` method takes [`partid::W`](W) writer structure"]
impl crate::Writable for PartidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARTID to value 0"]
impl crate::Resettable for PartidSpec {
    const RESET_VALUE: u32 = 0;
}
