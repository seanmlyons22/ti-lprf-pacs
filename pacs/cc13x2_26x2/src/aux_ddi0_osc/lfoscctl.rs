#[doc = "Register `LFOSCCTL` reader"]
pub type R = crate::R<LfoscctlSpec>;
#[doc = "Register `LFOSCCTL` writer"]
pub type W = crate::W<LfoscctlSpec>;
#[doc = "Field `RCOSCLF_CTUNE_TRIM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RcosclfCtuneTrimR = crate::FieldReader;
#[doc = "Field `RCOSCLF_CTUNE_TRIM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RcosclfCtuneTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "9:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RcosclfRtuneTrim {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _6p0meg = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _6p5meg = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _7p0meg = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _7p5meg = 0,
}
impl From<RcosclfRtuneTrim> for u8 {
    #[inline(always)]
    fn from(variant: RcosclfRtuneTrim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RcosclfRtuneTrim {
    type Ux = u8;
}
impl crate::IsEnum for RcosclfRtuneTrim {}
#[doc = "Field `RCOSCLF_RTUNE_TRIM` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type RcosclfRtuneTrimR = crate::FieldReader<RcosclfRtuneTrim>;
impl RcosclfRtuneTrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RcosclfRtuneTrim {
        match self.bits {
            3 => RcosclfRtuneTrim::_6p0meg,
            2 => RcosclfRtuneTrim::_6p5meg,
            1 => RcosclfRtuneTrim::_7p0meg,
            0 => RcosclfRtuneTrim::_7p5meg,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_6p0meg(&self) -> bool {
        *self == RcosclfRtuneTrim::_6p0meg
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_6p5meg(&self) -> bool {
        *self == RcosclfRtuneTrim::_6p5meg
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_7p0meg(&self) -> bool {
        *self == RcosclfRtuneTrim::_7p0meg
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_7p5meg(&self) -> bool {
        *self == RcosclfRtuneTrim::_7p5meg
    }
}
#[doc = "Field `RCOSCLF_RTUNE_TRIM` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type RcosclfRtuneTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2, RcosclfRtuneTrim, crate::Safe>;
impl<'a, REG> RcosclfRtuneTrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _6p0meg(self) -> &'a mut crate::W<REG> {
        self.variant(RcosclfRtuneTrim::_6p0meg)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _6p5meg(self) -> &'a mut crate::W<REG> {
        self.variant(RcosclfRtuneTrim::_6p5meg)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _7p0meg(self) -> &'a mut crate::W<REG> {
        self.variant(RcosclfRtuneTrim::_7p0meg)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _7p5meg(self) -> &'a mut crate::W<REG> {
        self.variant(RcosclfRtuneTrim::_7p5meg)
    }
}
#[doc = "Field `RESERVED10` reader - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` reader - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type XosclfCmirrwrRatioR = crate::FieldReader;
#[doc = "Field `XOSCLF_CMIRRWR_RATIO` writer - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type XosclfCmirrwrRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type XosclfRegulatorTrimR = crate::FieldReader;
#[doc = "Field `XOSCLF_REGULATOR_TRIM` writer - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type XosclfRegulatorTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RcosclfCtuneTrimR {
        RcosclfCtuneTrimR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RcosclfRtuneTrimR {
        RcosclfRtuneTrimR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:17 - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XosclfCmirrwrRatioR {
        XosclfCmirrwrRatioR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XosclfRegulatorTrimR {
        XosclfRegulatorTrimR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosclf_ctune_trim(&mut self) -> RcosclfCtuneTrimW<LfoscctlSpec> {
        RcosclfCtuneTrimW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosclf_rtune_trim(&mut self) -> RcosclfRtuneTrimW<LfoscctlSpec> {
        RcosclfRtuneTrimW::new(self, 8)
    }
    #[doc = "Bits 10:17 - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<LfoscctlSpec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> XosclfCmirrwrRatioW<LfoscctlSpec> {
        XosclfCmirrwrRatioW::new(self, 18)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosclf_regulator_trim(&mut self) -> XosclfRegulatorTrimW<LfoscctlSpec> {
        XosclfRegulatorTrimW::new(self, 22)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<LfoscctlSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Low Frequency Oscillator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfoscctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfoscctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfoscctlSpec;
impl crate::RegisterSpec for LfoscctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfoscctl::R`](R) reader structure"]
impl crate::Readable for LfoscctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfoscctl::W`](W) writer structure"]
impl crate::Writable for LfoscctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFOSCCTL to value 0"]
impl crate::Resettable for LfoscctlSpec {
    const RESET_VALUE: u32 = 0;
}
