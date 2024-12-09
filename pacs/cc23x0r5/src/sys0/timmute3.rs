#[doc = "Register `TIMMUTE3` reader"]
pub type R = crate::R<Timmute3Spec>;
#[doc = "Register `TIMMUTE3` writer"]
pub type W = crate::W<Timmute3Spec>;
#[doc = "Field `BGRSET` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type BgrsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "5:5\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmodeset {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Adc = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Atb = 0,
}
impl From<Tmodeset> for bool {
    #[inline(always)]
    fn from(variant: Tmodeset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMODESET` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type TmodesetW<'a, REG> = crate::BitWriter<'a, REG, Tmodeset>;
impl<'a, REG> TmodesetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Tmodeset::Adc)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atb(self) -> &'a mut crate::W<REG> {
        self.variant(Tmodeset::Atb)
    }
}
#[doc = "Field `TSENSSET` writer - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type TsenssetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "12:12\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hysdisset {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Dis = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    En = 0,
}
impl From<Hysdisset> for bool {
    #[inline(always)]
    fn from(variant: Hysdisset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYSDISSET` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type HysdissetW<'a, REG> = crate::BitWriter<'a, REG, Hysdisset>;
impl<'a, REG> HysdissetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hysdisset::Dis)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hysdisset::En)
    }
}
#[doc = "13:13\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atbresbypset {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Bypass = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Conn = 0,
}
impl From<Atbresbypset> for bool {
    #[inline(always)]
    fn from(variant: Atbresbypset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATBRESBYPSET` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type AtbresbypsetW<'a, REG> = crate::BitWriter<'a, REG, Atbresbypset>;
impl<'a, REG> AtbresbypsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Atbresbypset::Bypass)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn conn(self) -> &'a mut crate::W<REG> {
        self.variant(Atbresbypset::Conn)
    }
}
#[doc = "Field `BGRCLR` writer - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type BgrclrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TMODECLR` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type TmodeclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENSCLR` writer - 25:20\\]
Internal. Only to be used through TI provided API."]
pub type TsensclrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HYDISCLR` writer - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type HydisclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATBRESBYPCLR` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type AtbresbypclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bgrset(&mut self) -> BgrsetW<Timmute3Spec> {
        BgrsetW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tmodeset(&mut self) -> TmodesetW<Timmute3Spec> {
        TmodesetW::new(self, 5)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tsensset(&mut self) -> TsenssetW<Timmute3Spec> {
        TsenssetW::new(self, 6)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hysdisset(&mut self) -> HysdissetW<Timmute3Spec> {
        HysdissetW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atbresbypset(&mut self) -> AtbresbypsetW<Timmute3Spec> {
        AtbresbypsetW::new(self, 13)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bgrclr(&mut self) -> BgrclrW<Timmute3Spec> {
        BgrclrW::new(self, 14)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tmodeclr(&mut self) -> TmodeclrW<Timmute3Spec> {
        TmodeclrW::new(self, 19)
    }
    #[doc = "Bits 20:25 - 25:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tsensclr(&mut self) -> TsensclrW<Timmute3Spec> {
        TsensclrW::new(self, 20)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hydisclr(&mut self) -> HydisclrW<Timmute3Spec> {
        HydisclrW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atbresbypclr(&mut self) -> AtbresbypclrW<Timmute3Spec> {
        AtbresbypclrW::new(self, 27)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timmute3Spec;
impl crate::RegisterSpec for Timmute3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timmute3::R`](R) reader structure"]
impl crate::Readable for Timmute3Spec {}
#[doc = "`write(|w| ..)` method takes [`timmute3::W`](W) writer structure"]
impl crate::Writable for Timmute3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMMUTE3 to value 0"]
impl crate::Resettable for Timmute3Spec {
    const RESET_VALUE: u32 = 0;
}
