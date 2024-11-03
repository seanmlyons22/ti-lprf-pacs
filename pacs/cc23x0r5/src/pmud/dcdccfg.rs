#[doc = "Register `DCDCCFG` reader"]
pub type R = crate::R<DcdccfgSpec>;
#[doc = "Register `DCDCCFG` writer"]
pub type W = crate::W<DcdccfgSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lmen {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Enable = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Disable = 0,
}
impl From<Lmen> for bool {
    #[inline(always)]
    fn from(variant: Lmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMEN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type LmenR = crate::BitReader<Lmen>;
impl LmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lmen {
        match self.bits {
            true => Lmen::Enable,
            false => Lmen::Disable,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lmen::Enable
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lmen::Disable
    }
}
#[doc = "Field `LMEN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type LmenW<'a, REG> = crate::BitWriter<'a, REG, Lmen>;
impl<'a, REG> LmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lmen::Enable)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lmen::Disable)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "4:4\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdpIpeakEn {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Enable = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Disable = 0,
}
impl From<AdpIpeakEn> for bool {
    #[inline(always)]
    fn from(variant: AdpIpeakEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADP_IPEAK_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type AdpIpeakEnR = crate::BitReader<AdpIpeakEn>;
impl AdpIpeakEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdpIpeakEn {
        match self.bits {
            true => AdpIpeakEn::Enable,
            false => AdpIpeakEn::Disable,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AdpIpeakEn::Enable
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AdpIpeakEn::Disable
    }
}
#[doc = "Field `ADP_IPEAK_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type AdpIpeakEnW<'a, REG> = crate::BitWriter<'a, REG, AdpIpeakEn>;
impl<'a, REG> AdpIpeakEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AdpIpeakEn::Enable)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AdpIpeakEn::Disable)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "14:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LmLowth {
    #[doc = "127: Internal. Only to be used through TI provided API."]
    Max = 127,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Min = 0,
}
impl From<LmLowth> for u8 {
    #[inline(always)]
    fn from(variant: LmLowth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LmLowth {
    type Ux = u8;
}
impl crate::IsEnum for LmLowth {}
#[doc = "Field `LM_LOWTH` reader - 14:8\\]
Internal. Only to be used through TI provided API."]
pub type LmLowthR = crate::FieldReader<LmLowth>;
impl LmLowthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LmLowth> {
        match self.bits {
            127 => Some(LmLowth::Max),
            0 => Some(LmLowth::Min),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == LmLowth::Max
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == LmLowth::Min
    }
}
#[doc = "Field `LM_LOWTH` writer - 14:8\\]
Internal. Only to be used through TI provided API."]
pub type LmLowthW<'a, REG> = crate::FieldWriter<'a, REG, 7, LmLowth>;
impl<'a, REG> LmLowthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(LmLowth::Max)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(LmLowth::Min)
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "22:16\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LmHighth {
    #[doc = "127: Internal. Only to be used through TI provided API."]
    Max = 127,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Min = 0,
}
impl From<LmHighth> for u8 {
    #[inline(always)]
    fn from(variant: LmHighth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LmHighth {
    type Ux = u8;
}
impl crate::IsEnum for LmHighth {}
#[doc = "Field `LM_HIGHTH` reader - 22:16\\]
Internal. Only to be used through TI provided API."]
pub type LmHighthR = crate::FieldReader<LmHighth>;
impl LmHighthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LmHighth> {
        match self.bits {
            127 => Some(LmHighth::Max),
            0 => Some(LmHighth::Min),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == LmHighth::Max
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == LmHighth::Min
    }
}
#[doc = "Field `LM_HIGHTH` writer - 22:16\\]
Internal. Only to be used through TI provided API."]
pub type LmHighthW<'a, REG> = crate::FieldWriter<'a, REG, 7, LmHighth>;
impl<'a, REG> LmHighthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(LmHighth::Max)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(LmHighth::Min)
    }
}
#[doc = "Field `RESERVED23` reader - 31:23\\]
Internal. Only to be used through TI provided API."]
pub type Reserved23R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED23` writer - 31:23\\]
Internal. Only to be used through TI provided API."]
pub type Reserved23W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lmen(&self) -> LmenR {
        LmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adp_ipeak_en(&self) -> AdpIpeakEnR {
        AdpIpeakEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lm_lowth(&self) -> LmLowthR {
        LmLowthR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lm_highth(&self) -> LmHighthR {
        LmHighthR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LmenW<DcdccfgSpec> {
        LmenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DcdccfgSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adp_ipeak_en(&mut self) -> AdpIpeakEnW<DcdccfgSpec> {
        AdpIpeakEnW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<DcdccfgSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lm_lowth(&mut self) -> LmLowthW<DcdccfgSpec> {
        LmLowthW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<DcdccfgSpec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lm_highth(&mut self) -> LmHighthW<DcdccfgSpec> {
        LmHighthW::new(self, 16)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<DcdccfgSpec> {
        Reserved23W::new(self, 23)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdccfgSpec;
impl crate::RegisterSpec for DcdccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdccfg::R`](R) reader structure"]
impl crate::Readable for DcdccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdccfg::W`](W) writer structure"]
impl crate::Writable for DcdccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCCFG to value 0"]
impl crate::Resettable for DcdccfgSpec {
    const RESET_VALUE: u32 = 0;
}
