#[doc = "Register `IOSEGCLR` reader"]
pub type R = crate::R<IosegclrSpec>;
#[doc = "Register `IOSEGCLR` writer"]
pub type W = crate::W<IosegclrSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdds2 {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Dis = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Vdds2> for bool {
    #[inline(always)]
    fn from(variant: Vdds2) -> Self {
        variant as u8 != 0
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
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vdds2::Dis)
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
    Dis = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Noeff = 0,
}
impl From<Vdds3> for bool {
    #[inline(always)]
    fn from(variant: Vdds3) -> Self {
        variant as u8 != 0
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
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Vdds3::Dis)
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
impl R {
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
    pub fn vdds2(&mut self) -> Vdds2W<IosegclrSpec> {
        Vdds2W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vdds3(&mut self) -> Vdds3W<IosegclrSpec> {
        Vdds3W::new(self, 1)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iosegclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iosegclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IosegclrSpec;
impl crate::RegisterSpec for IosegclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iosegclr::R`](R) reader structure"]
impl crate::Readable for IosegclrSpec {}
#[doc = "`write(|w| ..)` method takes [`iosegclr::W`](W) writer structure"]
impl crate::Writable for IosegclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOSEGCLR to value 0"]
impl crate::Resettable for IosegclrSpec {
    const RESET_VALUE: u32 = 0;
}
