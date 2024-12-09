#[doc = "Register `ANA2_TRIM` reader"]
pub type R = crate::R<Ana2TrimSpec>;
#[doc = "Register `ANA2_TRIM` writer"]
pub type W = crate::W<Ana2TrimSpec>;
#[doc = "Field `DCDC_HIGH_EN_SEL` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type DcdcHighEnSelR = crate::FieldReader;
#[doc = "Field `DCDC_LOW_EN_SEL` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type DcdcLowEnSelR = crate::FieldReader;
#[doc = "Field `DEAD_TIME_TRIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type DeadTimeTrimR = crate::FieldReader;
#[doc = "Field `DCDC_IPEAK` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type DcdcIpeakR = crate::FieldReader;
#[doc = "Field `DITHER_EN` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DitherEnR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `NANOAMP_RES_TRIM` reader - 21:15\\]
Internal. Only to be used through TI provided API."]
pub type NanoampResTrimR = crate::FieldReader;
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type AtestlfUdigldoIbiasTrimR = crate::BitReader;
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` reader - 24:23\\]
Internal. Only to be used through TI provided API."]
pub type SetRcoscHfFineResistorR = crate::FieldReader;
#[doc = "Field `RESERVED0` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 30:26\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractR = crate::FieldReader;
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfctrimfractEnR = crate::BitReader;
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
impl W {}
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
