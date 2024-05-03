#[doc = "Register `ANA2_TRIM` reader"]
pub type R = crate::R<Ana2TrimSpec>;
#[doc = "Register `ANA2_TRIM` writer"]
pub type W = crate::W<Ana2TrimSpec>;
#[doc = "Field `DCDC_HIGH_EN_SEL` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DcdcHighEnSelR = crate::FieldReader;
#[doc = "Field `DCDC_HIGH_EN_SEL` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DcdcHighEnSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCDC_LOW_EN_SEL` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type DcdcLowEnSelR = crate::FieldReader;
#[doc = "Field `DCDC_LOW_EN_SEL` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type DcdcLowEnSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DEAD_TIME_TRIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type DeadTimeTrimR = crate::FieldReader;
#[doc = "Field `DEAD_TIME_TRIM` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type DeadTimeTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCDC_IPEAK` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type DcdcIpeakR = crate::FieldReader;
#[doc = "Field `DCDC_IPEAK` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type DcdcIpeakW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DITHER_EN` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DitherEnR = crate::BitReader;
#[doc = "Field `DITHER_EN` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DitherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NANOAMP_RES_TRIM` reader - 21:15\\]
Internal. Only to be used through TI provided API."]
pub type NanoampResTrimR = crate::FieldReader;
#[doc = "Field `NANOAMP_RES_TRIM` writer - 21:15\\]
Internal. Only to be used through TI provided API."]
pub type NanoampResTrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type AtestlfUdigldoIbiasTrimR = crate::BitReader;
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type AtestlfUdigldoIbiasTrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` reader - 24:23\\]
Internal. Only to be used through TI provided API."]
pub type SetRcoscHfFineResistorR = crate::FieldReader;
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` writer - 24:23\\]
Internal. Only to be used through TI provided API."]
pub type SetRcoscHfFineResistorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED0` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 30:26\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractR = crate::FieldReader;
#[doc = "Field `RCOSCHFCTRIMFRACT` writer - 30:26\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractEnR = crate::BitReader;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&self) -> DcdcHighEnSelR {
        DcdcHighEnSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&self) -> DcdcLowEnSelR {
        DcdcLowEnSelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dead_time_trim(&self) -> DeadTimeTrimR {
        DeadTimeTrimR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_ipeak(&self) -> DcdcIpeakR {
        DcdcIpeakR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dither_en(&self) -> DitherEnR {
        DitherEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:21 - 21:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_res_trim(&self) -> NanoampResTrimR {
        NanoampResTrimR::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&self) -> AtestlfUdigldoIbiasTrimR {
        AtestlfUdigldoIbiasTrimR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SetRcoscHfFineResistorR {
        SetRcoscHfFineResistorR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RcoschfctrimfractR {
        RcoschfctrimfractR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RcoschfctrimfractEnR {
        RcoschfctrimfractEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_high_en_sel(&mut self) -> DcdcHighEnSelW<Ana2TrimSpec> {
        DcdcHighEnSelW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_low_en_sel(&mut self) -> DcdcLowEnSelW<Ana2TrimSpec> {
        DcdcLowEnSelW::new(self, 3)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dead_time_trim(&mut self) -> DeadTimeTrimW<Ana2TrimSpec> {
        DeadTimeTrimW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ipeak(&mut self) -> DcdcIpeakW<Ana2TrimSpec> {
        DcdcIpeakW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dither_en(&mut self) -> DitherEnW<Ana2TrimSpec> {
        DitherEnW::new(self, 11)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Ana2TrimSpec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bits 15:21 - 21:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nanoamp_res_trim(&mut self) -> NanoampResTrimW<Ana2TrimSpec> {
        NanoampResTrimW::new(self, 15)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atestlf_udigldo_ibias_trim(&mut self) -> AtestlfUdigldoIbiasTrimW<Ana2TrimSpec> {
        AtestlfUdigldoIbiasTrimW::new(self, 22)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn set_rcosc_hf_fine_resistor(&mut self) -> SetRcoscHfFineResistorW<Ana2TrimSpec> {
        SetRcoscHfFineResistorW::new(self, 23)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Ana2TrimSpec> {
        Reserved0W::new(self, 25)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract(&mut self) -> RcoschfctrimfractW<Ana2TrimSpec> {
        RcoschfctrimfractW::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfctrimfract_en(&mut self) -> RcoschfctrimfractEnW<Ana2TrimSpec> {
        RcoschfctrimfractEnW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana2_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana2_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ana2TrimSpec;
impl crate::RegisterSpec for Ana2TrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana2_trim::R`](R) reader structure"]
impl crate::Readable for Ana2TrimSpec {}
#[doc = "`write(|w| ..)` method takes [`ana2_trim::W`](W) writer structure"]
impl crate::Writable for Ana2TrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA2_TRIM to value 0x8240_787f"]
impl crate::Resettable for Ana2TrimSpec {
    const RESET_VALUE: u32 = 0x8240_787f;
}
