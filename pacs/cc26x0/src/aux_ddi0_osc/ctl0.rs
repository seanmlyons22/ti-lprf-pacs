#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SclkHfSrcSel {
    #[doc = "1: High frequency XOSC clk"]
    Xosc = 1,
    #[doc = "0: High frequency RCOSC clock"]
    Rcosc = 0,
}
impl From<SclkHfSrcSel> for bool {
    #[inline(always)]
    fn from(variant: SclkHfSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLK_HF_SRC_SEL` reader - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
pub type SclkHfSrcSelR = crate::BitReader<SclkHfSrcSel>;
impl SclkHfSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclkHfSrcSel {
        match self.bits {
            true => SclkHfSrcSel::Xosc,
            false => SclkHfSrcSel::Rcosc,
        }
    }
    #[doc = "High frequency XOSC clk"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SclkHfSrcSel::Xosc
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        *self == SclkHfSrcSel::Rcosc
    }
}
#[doc = "Field `SCLK_HF_SRC_SEL` writer - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
pub type SclkHfSrcSelW<'a, REG> = crate::BitWriter<'a, REG, SclkHfSrcSel>;
impl<'a, REG> SclkHfSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency XOSC clk"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfSrcSel::Xosc)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfSrcSel::Rcosc)
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SclkMfSrcSel {
    #[doc = "1: Medium frequency clock derived from high frequency XOSC."]
    Xcoschfdmf = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Rcoschfdmf = 0,
}
impl From<SclkMfSrcSel> for bool {
    #[inline(always)]
    fn from(variant: SclkMfSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLK_MF_SRC_SEL` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type SclkMfSrcSelR = crate::BitReader<SclkMfSrcSel>;
impl SclkMfSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclkMfSrcSel {
        match self.bits {
            true => SclkMfSrcSel::Xcoschfdmf,
            false => SclkMfSrcSel::Rcoschfdmf,
        }
    }
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    #[inline(always)]
    pub fn is_xcoschfdmf(&self) -> bool {
        *self == SclkMfSrcSel::Xcoschfdmf
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_rcoschfdmf(&self) -> bool {
        *self == SclkMfSrcSel::Rcoschfdmf
    }
}
#[doc = "Field `SCLK_MF_SRC_SEL` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type SclkMfSrcSelW<'a, REG> = crate::BitWriter<'a, REG, SclkMfSrcSel>;
impl<'a, REG> SclkMfSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    #[inline(always)]
    pub fn xcoschfdmf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkMfSrcSel::Xcoschfdmf)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfdmf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkMfSrcSel::Rcoschfdmf)
    }
}
#[doc = "3:2\\]
Source select for sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SclkLfSrcSel {
    #[doc = "3: Low frequency XOSC"]
    Xosclf = 3,
    #[doc = "2: Low frequency RCOSC"]
    Rcosclf = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC"]
    Xoschfdlf = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    Rcoschfdlf = 0,
}
impl From<SclkLfSrcSel> for u8 {
    #[inline(always)]
    fn from(variant: SclkLfSrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SclkLfSrcSel {
    type Ux = u8;
}
impl crate::IsEnum for SclkLfSrcSel {}
#[doc = "Field `SCLK_LF_SRC_SEL` reader - 3:2\\]
Source select for sclk_lf"]
pub type SclkLfSrcSelR = crate::FieldReader<SclkLfSrcSel>;
impl SclkLfSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclkLfSrcSel {
        match self.bits {
            3 => SclkLfSrcSel::Xosclf,
            2 => SclkLfSrcSel::Rcosclf,
            1 => SclkLfSrcSel::Xoschfdlf,
            0 => SclkLfSrcSel::Rcoschfdlf,
            _ => unreachable!(),
        }
    }
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        *self == SclkLfSrcSel::Xosclf
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        *self == SclkLfSrcSel::Rcosclf
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SclkLfSrcSel::Xoschfdlf
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SclkLfSrcSel::Rcoschfdlf
    }
}
#[doc = "Field `SCLK_LF_SRC_SEL` writer - 3:2\\]
Source select for sclk_lf"]
pub type SclkLfSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, SclkLfSrcSel, crate::Safe>;
impl<'a, REG> SclkLfSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrcSel::Xosclf)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrcSel::Rcosclf)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrcSel::Xoschfdlf)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrcSel::Rcoschfdlf)
    }
}
#[doc = "Field `SPARE4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare4R = crate::BitReader;
#[doc = "Field `SPARE4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_REF_SRC_SEL` reader - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
pub type AclkRefSrcSelR = crate::FieldReader;
#[doc = "Field `ACLK_REF_SRC_SEL` writer - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
pub type AclkRefSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACLK_TDC_SRC_SEL` reader - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
pub type AclkTdcSrcSelR = crate::FieldReader;
#[doc = "Field `ACLK_TDC_SRC_SEL` writer - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
pub type AclkTdcSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_LOSS_EN` reader - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type ClkLossEnR = crate::BitReader;
#[doc = "Field `CLK_LOSS_EN` writer - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type ClkLossEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_LF_DIG_BYPASS` reader - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
pub type XoscLfDigBypassR = crate::BitReader;
#[doc = "Field `XOSC_LF_DIG_BYPASS` writer - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
pub type XoscLfDigBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_HF_POWER_MODE` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfPowerModeR = crate::BitReader;
#[doc = "Field `XOSC_HF_POWER_MODE` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfPowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSC_LF_TRIMMED` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RcoscLfTrimmedR = crate::BitReader;
#[doc = "Field `RCOSC_LF_TRIMMED` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RcoscLfTrimmedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED13` reader - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `RESERVED13` writer - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPOSC_MODE_EN` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type HposcModeEnR = crate::BitReader;
#[doc = "Field `HPOSC_MODE_EN` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type HposcModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALLOW_SCLK_HF_SWITCHING` reader - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
pub type AllowSclkHfSwitchingR = crate::BitReader;
#[doc = "Field `ALLOW_SCLK_HF_SWITCHING` writer - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
pub type AllowSclkHfSwitchingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `RESERVED17` writer - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FORCE_KICKSTART_EN` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type ForceKickstartEnR = crate::BitReader;
#[doc = "Field `FORCE_KICKSTART_EN` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type ForceKickstartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED23` reader - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::FieldReader;
#[doc = "Field `RESERVED23` writer - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUBLER_RESET_DURATION` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type DoublerResetDurationR = crate::BitReader;
#[doc = "Field `DOUBLER_RESET_DURATION` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type DoublerResetDurationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUBLER_START_DURATION` reader - 27:26\\]
Internal. Only to be used through TI provided API."]
pub type DoublerStartDurationR = crate::FieldReader;
#[doc = "Field `DOUBLER_START_DURATION` writer - 27:26\\]
Internal. Only to be used through TI provided API."]
pub type DoublerStartDurationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BYPASS_RCOSC_LF_CLK_QUAL` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type BypassRcoscLfClkQualR = crate::BitReader;
#[doc = "Field `BYPASS_RCOSC_LF_CLK_QUAL` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type BypassRcoscLfClkQualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_XOSC_LF_CLK_QUAL` reader - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type BypassXoscLfClkQualR = crate::BitReader;
#[doc = "Field `BYPASS_XOSC_LF_CLK_QUAL` writer - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type BypassXoscLfClkQualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED30` reader - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::BitReader;
#[doc = "Field `RESERVED30` writer - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "31:31\\]
Set based on the accurate high frequency XTAL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XtalIs24m {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _24m = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _48m = 0,
}
impl From<XtalIs24m> for bool {
    #[inline(always)]
    fn from(variant: XtalIs24m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTAL_IS_24M` reader - 31:31\\]
Set based on the accurate high frequency XTAL."]
pub type XtalIs24mR = crate::BitReader<XtalIs24m>;
impl XtalIs24mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XtalIs24m {
        match self.bits {
            true => XtalIs24m::_24m,
            false => XtalIs24m::_48m,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == XtalIs24m::_24m
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        *self == XtalIs24m::_48m
    }
}
#[doc = "Field `XTAL_IS_24M` writer - 31:31\\]
Set based on the accurate high frequency XTAL."]
pub type XtalIs24mW<'a, REG> = crate::BitWriter<'a, REG, XtalIs24m>;
impl<'a, REG> XtalIs24mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut crate::W<REG> {
        self.variant(XtalIs24m::_24m)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut crate::W<REG> {
        self.variant(XtalIs24m::_48m)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&self) -> SclkHfSrcSelR {
        SclkHfSrcSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sclk_mf_src_sel(&self) -> SclkMfSrcSelR {
        SclkMfSrcSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&self) -> SclkLfSrcSelR {
        SclkLfSrcSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare4(&self) -> Spare4R {
        Spare4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&self) -> AclkRefSrcSelR {
        AclkRefSrcSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&self) -> AclkTdcSrcSelR {
        AclkTdcSrcSelR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> ClkLossEnR {
        ClkLossEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&self) -> XoscLfDigBypassR {
        XoscLfDigBypassR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&self) -> XoscHfPowerModeR {
        XoscHfPowerModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&self) -> RcoscLfTrimmedR {
        RcoscLfTrimmedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_mode_en(&self) -> HposcModeEnR {
        HposcModeEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline(always)]
    pub fn allow_sclk_hf_switching(&self) -> AllowSclkHfSwitchingR {
        AllowSclkHfSwitchingR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_kickstart_en(&self) -> ForceKickstartEnR {
        ForceKickstartEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_reset_duration(&self) -> DoublerResetDurationR {
        DoublerResetDurationR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_start_duration(&self) -> DoublerStartDurationR {
        DoublerStartDurationR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&self) -> BypassRcoscLfClkQualR {
        BypassRcoscLfClkQualR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&self) -> BypassXoscLfClkQualR {
        BypassXoscLfClkQualR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub fn xtal_is_24m(&self) -> XtalIs24mR {
        XtalIs24mR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_src_sel(&mut self) -> SclkHfSrcSelW<Ctl0Spec> {
        SclkHfSrcSelW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_mf_src_sel(&mut self) -> SclkMfSrcSelW<Ctl0Spec> {
        SclkMfSrcSelW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_src_sel(&mut self) -> SclkLfSrcSelW<Ctl0Spec> {
        SclkLfSrcSelW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare4(&mut self) -> Spare4W<Ctl0Spec> {
        Spare4W::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_ref_src_sel(&mut self) -> AclkRefSrcSelW<Ctl0Spec> {
        AclkRefSrcSelW::new(self, 5)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_tdc_src_sel(&mut self) -> AclkTdcSrcSelW<Ctl0Spec> {
        AclkTdcSrcSelW::new(self, 7)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    #[must_use]
    pub fn clk_loss_en(&mut self) -> ClkLossEnW<Ctl0Spec> {
        ClkLossEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_lf_dig_bypass(&mut self) -> XoscLfDigBypassW<Ctl0Spec> {
        XoscLfDigBypassW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_power_mode(&mut self) -> XoscHfPowerModeW<Ctl0Spec> {
        XoscHfPowerModeW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_lf_trimmed(&mut self) -> RcoscLfTrimmedW<Ctl0Spec> {
        RcoscLfTrimmedW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<Ctl0Spec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_mode_en(&mut self) -> HposcModeEnW<Ctl0Spec> {
        HposcModeEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Ctl0Spec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline(always)]
    #[must_use]
    pub fn allow_sclk_hf_switching(&mut self) -> AllowSclkHfSwitchingW<Ctl0Spec> {
        AllowSclkHfSwitchingW::new(self, 16)
    }
    #[doc = "Bits 17:21 - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Ctl0Spec> {
        Reserved17W::new(self, 17)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_kickstart_en(&mut self) -> ForceKickstartEnW<Ctl0Spec> {
        ForceKickstartEnW::new(self, 22)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<Ctl0Spec> {
        Reserved23W::new(self, 23)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn doubler_reset_duration(&mut self) -> DoublerResetDurationW<Ctl0Spec> {
        DoublerResetDurationW::new(self, 25)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn doubler_start_duration(&mut self) -> DoublerStartDurationW<Ctl0Spec> {
        DoublerStartDurationW::new(self, 26)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_rcosc_lf_clk_qual(&mut self) -> BypassRcoscLfClkQualW<Ctl0Spec> {
        BypassRcoscLfClkQualW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_xosc_lf_clk_qual(&mut self) -> BypassXoscLfClkQualW<Ctl0Spec> {
        BypassXoscLfClkQualW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> Reserved30W<Ctl0Spec> {
        Reserved30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn xtal_is_24m(&mut self) -> XtalIs24mW<Ctl0Spec> {
        XtalIs24mW::new(self, 31)
    }
}
#[doc = "Control 0 Controls clock source selects\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
