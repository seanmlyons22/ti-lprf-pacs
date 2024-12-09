#[doc = "Register `AUXCLK` reader"]
pub type R = crate::R<AuxclkSpec>;
#[doc = "Register `AUXCLK` writer"]
pub type W = crate::W<AuxclkSpec>;
#[doc = "2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "4: LF Clock (SCLK_LF)"]
    SclkLf = 4,
    #[doc = "1: HF Clock (SCLK_HF)"]
    SclkHf = 1,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            4 => Some(Src::SclkLf),
            1 => Some(Src::SclkHf),
            _ => None,
        }
    }
    #[doc = "LF Clock (SCLK_LF)"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == Src::SclkLf
    }
    #[doc = "HF Clock (SCLK_HF)"]
    #[inline(always)]
    pub fn is_sclk_hf(&self) -> bool {
        *self == Src::SclkHf
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LF Clock (SCLK_LF)"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkLf)
    }
    #[doc = "HF Clock (SCLK_HF)"]
    #[inline(always)]
    pub fn sclk_hf(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkHf)
    }
}
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SclkHfDiv {
    #[doc = "7: Divide by 256"]
    Div256 = 7,
    #[doc = "6: Divide by 128"]
    Div128 = 6,
    #[doc = "5: Divide by 64"]
    Div64 = 5,
    #[doc = "4: Divide by 32"]
    Div32 = 4,
    #[doc = "3: Divide by 16"]
    Div16 = 3,
    #[doc = "2: Divide by 8"]
    Div8 = 2,
    #[doc = "1: Divide by 4"]
    Div4 = 1,
    #[doc = "0: Divide by 2"]
    Div2 = 0,
}
impl From<SclkHfDiv> for u8 {
    #[inline(always)]
    fn from(variant: SclkHfDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SclkHfDiv {
    type Ux = u8;
}
impl crate::IsEnum for SclkHfDiv {}
#[doc = "Field `SCLK_HF_DIV` reader - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
pub type SclkHfDivR = crate::FieldReader<SclkHfDiv>;
impl SclkHfDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclkHfDiv {
        match self.bits {
            7 => SclkHfDiv::Div256,
            6 => SclkHfDiv::Div128,
            5 => SclkHfDiv::Div64,
            4 => SclkHfDiv::Div32,
            3 => SclkHfDiv::Div16,
            2 => SclkHfDiv::Div8,
            1 => SclkHfDiv::Div4,
            0 => SclkHfDiv::Div2,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == SclkHfDiv::Div256
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SclkHfDiv::Div128
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SclkHfDiv::Div64
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SclkHfDiv::Div32
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SclkHfDiv::Div16
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SclkHfDiv::Div8
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SclkHfDiv::Div4
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SclkHfDiv::Div2
    }
}
#[doc = "Field `SCLK_HF_DIV` writer - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
pub type SclkHfDivW<'a, REG> = crate::FieldWriter<'a, REG, 3, SclkHfDiv, crate::Safe>;
impl<'a, REG> SclkHfDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div256)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div128)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div64)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div32)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div16)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div8)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div4)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfDiv::Div2)
    }
}
#[doc = "12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrDwnSrc {
    #[doc = "1: Use SCLK_LF in Powerdown"]
    SclkLf = 1,
    #[doc = "0: No clock in Powerdown"]
    None = 0,
}
impl From<PwrDwnSrc> for u8 {
    #[inline(always)]
    fn from(variant: PwrDwnSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrDwnSrc {
    type Ux = u8;
}
impl crate::IsEnum for PwrDwnSrc {}
#[doc = "Field `PWR_DWN_SRC` reader - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
pub type PwrDwnSrcR = crate::FieldReader<PwrDwnSrc>;
impl PwrDwnSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwrDwnSrc> {
        match self.bits {
            1 => Some(PwrDwnSrc::SclkLf),
            0 => Some(PwrDwnSrc::None),
            _ => None,
        }
    }
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PwrDwnSrc::SclkLf
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PwrDwnSrc::None
    }
}
#[doc = "Field `PWR_DWN_SRC` writer - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
pub type PwrDwnSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrDwnSrc>;
impl<'a, REG> PwrDwnSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(PwrDwnSrc::SclkLf)
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PwrDwnSrc::None)
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline(always)]
    pub fn sclk_hf_div(&self) -> SclkHfDivR {
        SclkHfDivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PwrDwnSrcR {
        PwrDwnSrcR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<AuxclkSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_div(&mut self) -> SclkHfDivW<AuxclkSpec> {
        SclkHfDivW::new(self, 8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_dwn_src(&mut self) -> PwrDwnSrcW<AuxclkSpec> {
        PwrDwnSrcW::new(self, 11)
    }
}
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxclkSpec;
impl crate::RegisterSpec for AuxclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxclk::R`](R) reader structure"]
impl crate::Readable for AuxclkSpec {}
#[doc = "`write(|w| ..)` method takes [`auxclk::W`](W) writer structure"]
impl crate::Writable for AuxclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXCLK to value 0x01"]
impl crate::Resettable for AuxclkSpec {
    const RESET_VALUE: u32 = 0x01;
}
