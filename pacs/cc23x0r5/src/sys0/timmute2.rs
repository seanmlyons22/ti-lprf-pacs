#[doc = "Register `TIMMUTE2` reader"]
pub type R = crate::R<Timmute2Spec>;
#[doc = "Register `TIMMUTE2` writer"]
pub type W = crate::W<Timmute2Spec>;
#[doc = "Field `ITDUMMY` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type ItdummyR = crate::FieldReader;
#[doc = "Field `ITDUMMY` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type ItdummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ITDIG` reader - 4:2\\]
Internal. Only to be used through TI provided API."]
pub type ItdigR = crate::FieldReader;
#[doc = "Field `ITDIG` writer - 4:2\\]
Internal. Only to be used through TI provided API."]
pub type ItdigW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITUDIG` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type ItudigR = crate::FieldReader;
#[doc = "Field `ITUDIG` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type ItudigW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RECOMPOS` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type RecomposR = crate::FieldReader;
#[doc = "Field `RECOMPOS` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type RecomposW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCDC_SA_MAX_HYS` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type DcdcSaMaxHysR = crate::BitReader;
#[doc = "Field `DCDC_SA_MAX_HYS` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type DcdcSaMaxHysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE0` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type Spare0R = crate::BitReader;
#[doc = "Field `SPARE0` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type Spare0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLDOISINKEN` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type GldoisinkenR = crate::BitReader;
#[doc = "Field `GLDOISINKEN` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type GldoisinkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLDOEACDIS` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type GldoeacdisR = crate::BitReader;
#[doc = "Field `GLDOEACDIS` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type GldoeacdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE1` reader - 18:17\\]
Internal. Only to be used through TI provided API."]
pub type Spare1R = crate::FieldReader;
#[doc = "Field `SPARE1` writer - 18:17\\]
Internal. Only to be used through TI provided API."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WSTGSEP` reader - 24:19\\]
Internal. Only to be used through TI provided API."]
pub type WstgsepR = crate::FieldReader;
#[doc = "Field `WSTGSEP` writer - 24:19\\]
Internal. Only to be used through TI provided API."]
pub type WstgsepW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WTOSSEP` reader - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type WtossepR = crate::FieldReader;
#[doc = "Field `WTOSSEP` writer - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type WtossepW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CKMDIGLOCKSEL` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type CkmdiglockselR = crate::BitReader;
#[doc = "Field `CKMDIGLOCKSEL` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type CkmdiglockselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itdummy(&self) -> ItdummyR {
        ItdummyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itdig(&self) -> ItdigR {
        ItdigR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itudig(&self) -> ItudigR {
        ItudigR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn recompos(&self) -> RecomposR {
        RecomposR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_sa_max_hys(&self) -> DcdcSaMaxHysR {
        DcdcSaMaxHysR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare0(&self) -> Spare0R {
        Spare0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldoisinken(&self) -> GldoisinkenR {
        GldoisinkenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldoeacdis(&self) -> GldoeacdisR {
        GldoeacdisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - 18:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare1(&self) -> Spare1R {
        Spare1R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:24 - 24:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wstgsep(&self) -> WstgsepR {
        WstgsepR::new(((self.bits >> 19) & 0x3f) as u8)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wtossep(&self) -> WtossepR {
        WtossepR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ckmdiglocksel(&self) -> CkmdiglockselR {
        CkmdiglockselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itdummy(&mut self) -> ItdummyW<Timmute2Spec> {
        ItdummyW::new(self, 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itdig(&mut self) -> ItdigW<Timmute2Spec> {
        ItdigW::new(self, 2)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itudig(&mut self) -> ItudigW<Timmute2Spec> {
        ItudigW::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn recompos(&mut self) -> RecomposW<Timmute2Spec> {
        RecomposW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_sa_max_hys(&mut self) -> DcdcSaMaxHysW<Timmute2Spec> {
        DcdcSaMaxHysW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare0(&mut self) -> Spare0W<Timmute2Spec> {
        Spare0W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gldoisinken(&mut self) -> GldoisinkenW<Timmute2Spec> {
        GldoisinkenW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gldoeacdis(&mut self) -> GldoeacdisW<Timmute2Spec> {
        GldoeacdisW::new(self, 16)
    }
    #[doc = "Bits 17:18 - 18:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> Spare1W<Timmute2Spec> {
        Spare1W::new(self, 17)
    }
    #[doc = "Bits 19:24 - 24:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wstgsep(&mut self) -> WstgsepW<Timmute2Spec> {
        WstgsepW::new(self, 19)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wtossep(&mut self) -> WtossepW<Timmute2Spec> {
        WtossepW::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ckmdiglocksel(&mut self) -> CkmdiglockselW<Timmute2Spec> {
        CkmdiglockselW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timmute2Spec;
impl crate::RegisterSpec for Timmute2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timmute2::R`](R) reader structure"]
impl crate::Readable for Timmute2Spec {}
#[doc = "`write(|w| ..)` method takes [`timmute2::W`](W) writer structure"]
impl crate::Writable for Timmute2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMMUTE2 to value 0x7ff8_0000"]
impl crate::Resettable for Timmute2Spec {
    const RESET_VALUE: u32 = 0x7ff8_0000;
}
