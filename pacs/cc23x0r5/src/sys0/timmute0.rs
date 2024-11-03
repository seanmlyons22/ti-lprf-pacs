#[doc = "Register `TIMMUTE0` reader"]
pub type R = crate::R<Timmute0Spec>;
#[doc = "Register `TIMMUTE0` writer"]
pub type W = crate::W<Timmute0Spec>;
#[doc = "Field `VREF` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VrefR = crate::FieldReader;
#[doc = "Field `VREF` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VBG` reader - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type VbgR = crate::FieldReader;
#[doc = "Field `VBG` writer - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type VbgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VDDSBOD` reader - 14:10\\]
Internal. Only to be used through TI provided API."]
pub type VddsbodR = crate::FieldReader;
#[doc = "Field `VDDSBOD` writer - 14:10\\]
Internal. Only to be used through TI provided API."]
pub type VddsbodW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BGTRIMEN` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type BgtrimenR = crate::BitReader;
#[doc = "Field `BGTRIMEN` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type BgtrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREF` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type IrefR = crate::FieldReader;
#[doc = "Field `IREF` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type IrefW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSENSE` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type TsenseR = crate::FieldReader;
#[doc = "Field `TSENSE` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type TsenseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDROKHYST` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrokhystR = crate::BitReader;
#[doc = "Field `VDDROKHYST` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrokhystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISSAHYST` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type DissahystR = crate::BitReader;
#[doc = "Field `DISSAHYST` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type DissahystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLDOCOMPDIS` reader - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type GldocompdisR = crate::BitReader;
#[doc = "Field `GLDOCOMPDIS` writer - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type GldocompdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLDODISANA` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type GldodisanaR = crate::BitReader;
#[doc = "Field `GLDODISANA` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type GldodisanaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vref(&self) -> VrefR {
        VrefR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vbg(&self) -> VbgR {
        VbgR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddsbod(&self) -> VddsbodR {
        VddsbodR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bgtrimen(&self) -> BgtrimenR {
        BgtrimenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iref(&self) -> IrefR {
        IrefR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tsense(&self) -> TsenseR {
        TsenseR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddrokhyst(&self) -> VddrokhystR {
        VddrokhystR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dissahyst(&self) -> DissahystR {
        DissahystR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldocompdis(&self) -> GldocompdisR {
        GldocompdisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldodisana(&self) -> GldodisanaR {
        GldodisanaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vref(&mut self) -> VrefW<Timmute0Spec> {
        VrefW::new(self, 0)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vbg(&mut self) -> VbgW<Timmute0Spec> {
        VbgW::new(self, 4)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddsbod(&mut self) -> VddsbodW<Timmute0Spec> {
        VddsbodW::new(self, 10)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bgtrimen(&mut self) -> BgtrimenW<Timmute0Spec> {
        BgtrimenW::new(self, 15)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iref(&mut self) -> IrefW<Timmute0Spec> {
        IrefW::new(self, 16)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tsense(&mut self) -> TsenseW<Timmute0Spec> {
        TsenseW::new(self, 21)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<Timmute0Spec> {
        SpareW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddrokhyst(&mut self) -> VddrokhystW<Timmute0Spec> {
        VddrokhystW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dissahyst(&mut self) -> DissahystW<Timmute0Spec> {
        DissahystW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gldocompdis(&mut self) -> GldocompdisW<Timmute0Spec> {
        GldocompdisW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gldodisana(&mut self) -> GldodisanaW<Timmute0Spec> {
        GldodisanaW::new(self, 27)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<Timmute0Spec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timmute0Spec;
impl crate::RegisterSpec for Timmute0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timmute0::R`](R) reader structure"]
impl crate::Readable for Timmute0Spec {}
#[doc = "`write(|w| ..)` method takes [`timmute0::W`](W) writer structure"]
impl crate::Writable for Timmute0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMMUTE0 to value 0"]
impl crate::Resettable for Timmute0Spec {
    const RESET_VALUE: u32 = 0;
}
