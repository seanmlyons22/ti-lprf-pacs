#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_HF_SRC_SEL` reader - 0:0\\]
Source select for sclk_hf."]
pub type SCLK_HF_SRC_SEL_R = crate::BitReader<SCLK_HF_SRC_SEL_A>;
#[doc = "0:0\\]
Source select for sclk_hf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLK_HF_SRC_SEL_A {
    #[doc = "1: High frequency XOSC or HPOSC clk (use HPOSC when HPOSC_MODE_EN = 1"]
    XOSC = 1,
    #[doc = "0: High frequency RCOSC clock"]
    RCOSC = 0,
}
impl From<SCLK_HF_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_HF_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLK_HF_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_SRC_SEL_A {
        match self.bits {
            true => SCLK_HF_SRC_SEL_A::XOSC,
            false => SCLK_HF_SRC_SEL_A::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SCLK_HF_SRC_SEL_A::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        *self == SCLK_HF_SRC_SEL_A::RCOSC
    }
}
#[doc = "Field `SCLK_HF_SRC_SEL` writer - 0:0\\]
Source select for sclk_hf."]
pub type SCLK_HF_SRC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTL0_SPEC, SCLK_HF_SRC_SEL_A, O>;
impl<'a, const O: u8> SCLK_HF_SRC_SEL_W<'a, O> {
    #[doc = "High frequency XOSC or HPOSC clk (use HPOSC when HPOSC_MODE_EN = 1"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SEL_A::XOSC)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SEL_A::RCOSC)
    }
}
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `SCLK_LF_SRC_SEL` reader - 3:2\\]
Source select for sclk_lf"]
pub type SCLK_LF_SRC_SEL_R = crate::FieldReader<u8, SCLK_LF_SRC_SEL_A>;
#[doc = "3:2\\]
Source select for sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLK_LF_SRC_SEL_A {
    #[doc = "3: Low frequency XOSC"]
    XOSCLF = 3,
    #[doc = "2: Low frequency RCOSC"]
    RCOSCLF = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC or HPOSC clk (use HPOSC when HPOSC_MODE_EN = 1)"]
    XOSCHFDLF = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF = 0,
}
impl From<SCLK_LF_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl SCLK_LF_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_SRC_SEL_A {
        match self.bits {
            3 => SCLK_LF_SRC_SEL_A::XOSCLF,
            2 => SCLK_LF_SRC_SEL_A::RCOSCLF,
            1 => SCLK_LF_SRC_SEL_A::XOSCHFDLF,
            0 => SCLK_LF_SRC_SEL_A::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::RCOSCHFDLF
    }
}
#[doc = "Field `SCLK_LF_SRC_SEL` writer - 3:2\\]
Source select for sclk_lf"]
pub type SCLK_LF_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTL0_SPEC, u8, SCLK_LF_SRC_SEL_A, 2, O>;
impl<'a, const O: u8> SCLK_LF_SRC_SEL_W<'a, O> {
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::XOSCLF)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::RCOSCLF)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC or HPOSC clk (use HPOSC when HPOSC_MODE_EN = 1)"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::XOSCHFDLF)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::RCOSCHFDLF)
    }
}
#[doc = "Field `ACLK_REF_SRC_SEL` reader - 6:4\\]
Source select for aclk_ref 000: RCOSC_HF derived (31.25kHz) 001: XOSC_HF derived (31.25kHz) 010: RCOSC_LF (32kHz) 011: XOSC_LF (32.768kHz) 100: RCOSC_MF (2MHz) 101-111: Not used"]
pub type ACLK_REF_SRC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACLK_REF_SRC_SEL` writer - 6:4\\]
Source select for aclk_ref 000: RCOSC_HF derived (31.25kHz) 001: XOSC_HF derived (31.25kHz) 010: RCOSC_LF (32kHz) 011: XOSC_LF (32.768kHz) 100: RCOSC_MF (2MHz) 101-111: Not used"]
pub type ACLK_REF_SRC_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `ACLK_TDC_SRC_SEL` reader - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
pub type ACLK_TDC_SRC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACLK_TDC_SRC_SEL` writer - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
pub type ACLK_TDC_SRC_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLK_LOSS_EN` reader - 9:9\\]
Enable clock loss detection and hence the indicators to the system controller. Checks both SCLK_HF, SCLK_MF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type CLK_LOSS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_LOSS_EN` writer - 9:9\\]
Enable clock loss detection and hence the indicators to the system controller. Checks both SCLK_HF, SCLK_MF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub type CLK_LOSS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `XOSC_LF_DIG_BYPASS` reader - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
pub type XOSC_LF_DIG_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_LF_DIG_BYPASS` writer - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
pub type XOSC_LF_DIG_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_POWER_MODE` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_POWER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_POWER_MODE` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_POWER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `RCOSC_LF_TRIMMED` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_LF_TRIMMED_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_LF_TRIMMED` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_LF_TRIMMED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `RESERVED13` reader - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED13` writer - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `HPOSC_MODE_EN` reader - 14:14\\]
0: HPOSC mode is not enabled. The 48 MHz crystal is required for radio operation. 1: Enables HPOSC mode. The internal HPOSC can be used as HF system clock and for radio operation."]
pub type HPOSC_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `HPOSC_MODE_EN` writer - 14:14\\]
0: HPOSC mode is not enabled. The 48 MHz crystal is required for radio operation. 1: Enables HPOSC mode. The internal HPOSC can be used as HF system clock and for radio operation."]
pub type HPOSC_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `RESERVED15` reader - 23:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED15` writer - 23:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u16, u16, 9, O>;
#[doc = "Field `CLK_DCDC_SRC_SEL` reader - 24:24\\]
Select DCDC clock source. 0: CLK_DCDC is 48 MHz clock from RCOSC or XOSC / HPOSC 1: CLK_DCDC is always 48 MHz clock from RCOSC"]
pub type CLK_DCDC_SRC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLK_DCDC_SRC_SEL` writer - 24:24\\]
Select DCDC clock source. 0: CLK_DCDC is 48 MHz clock from RCOSC or XOSC / HPOSC 1: CLK_DCDC is always 48 MHz clock from RCOSC"]
pub type CLK_DCDC_SRC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `DOUBLER_RESET_DURATION` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type DOUBLER_RESET_DURATION_R = crate::BitReader<bool>;
#[doc = "Field `DOUBLER_RESET_DURATION` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type DOUBLER_RESET_DURATION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `DOUBLER_START_DURATION` reader - 27:26\\]
Internal. Only to be used through TI provided API."]
pub type DOUBLER_START_DURATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUBLER_START_DURATION` writer - 27:26\\]
Internal. Only to be used through TI provided API."]
pub type DOUBLER_START_DURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BYPASS_RCOSC_LF_CLK_QUAL` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type BYPASS_RCOSC_LF_CLK_QUAL_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS_RCOSC_LF_CLK_QUAL` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type BYPASS_RCOSC_LF_CLK_QUAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `BYPASS_XOSC_LF_CLK_QUAL` reader - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type BYPASS_XOSC_LF_CLK_QUAL_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS_XOSC_LF_CLK_QUAL` writer - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type BYPASS_XOSC_LF_CLK_QUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `RESERVED30` reader - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED30` writer - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `XTAL_IS_24M` reader - 31:31\\]
Set based on the accurate high frequency XTAL."]
pub type XTAL_IS_24M_R = crate::BitReader<XTAL_IS_24M_A>;
#[doc = "31:31\\]
Set based on the accurate high frequency XTAL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTAL_IS_24M_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _24M = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _48M = 0,
}
impl From<XTAL_IS_24M_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_IS_24M_A) -> Self {
        variant as u8 != 0
    }
}
impl XTAL_IS_24M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_IS_24M_A {
        match self.bits {
            true => XTAL_IS_24M_A::_24M,
            false => XTAL_IS_24M_A::_48M,
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == XTAL_IS_24M_A::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        *self == XTAL_IS_24M_A::_48M
    }
}
#[doc = "Field `XTAL_IS_24M` writer - 31:31\\]
Set based on the accurate high frequency XTAL."]
pub type XTAL_IS_24M_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, XTAL_IS_24M_A, O>;
impl<'a, const O: u8> XTAL_IS_24M_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XTAL_IS_24M_A::_24M)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XTAL_IS_24M_A::_48M)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf."]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&self) -> SCLK_HF_SRC_SEL_R {
        SCLK_HF_SRC_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&self) -> SCLK_LF_SRC_SEL_R {
        SCLK_LF_SRC_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Source select for aclk_ref 000: RCOSC_HF derived (31.25kHz) 001: XOSC_HF derived (31.25kHz) 010: RCOSC_LF (32kHz) 011: XOSC_LF (32.768kHz) 100: RCOSC_MF (2MHz) 101-111: Not used"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&self) -> ACLK_REF_SRC_SEL_R {
        ACLK_REF_SRC_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&self) -> ACLK_TDC_SRC_SEL_R {
        ACLK_TDC_SRC_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to the system controller. Checks both SCLK_HF, SCLK_MF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&self) -> XOSC_LF_DIG_BYPASS_R {
        XOSC_LF_DIG_BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&self) -> XOSC_HF_POWER_MODE_R {
        XOSC_HF_POWER_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&self) -> RCOSC_LF_TRIMMED_R {
        RCOSC_LF_TRIMMED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: HPOSC mode is not enabled. The 48 MHz crystal is required for radio operation. 1: Enables HPOSC mode. The internal HPOSC can be used as HF system clock and for radio operation."]
    #[inline(always)]
    pub fn hposc_mode_en(&self) -> HPOSC_MODE_EN_R {
        HPOSC_MODE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:23 - 23:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Select DCDC clock source. 0: CLK_DCDC is 48 MHz clock from RCOSC or XOSC / HPOSC 1: CLK_DCDC is always 48 MHz clock from RCOSC"]
    #[inline(always)]
    pub fn clk_dcdc_src_sel(&self) -> CLK_DCDC_SRC_SEL_R {
        CLK_DCDC_SRC_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_reset_duration(&self) -> DOUBLER_RESET_DURATION_R {
        DOUBLER_RESET_DURATION_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_start_duration(&self) -> DOUBLER_START_DURATION_R {
        DOUBLER_START_DURATION_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&self) -> BYPASS_RCOSC_LF_CLK_QUAL_R {
        BYPASS_RCOSC_LF_CLK_QUAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&self) -> BYPASS_XOSC_LF_CLK_QUAL_R {
        BYPASS_XOSC_LF_CLK_QUAL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub fn xtal_is_24m(&self) -> XTAL_IS_24M_R {
        XTAL_IS_24M_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_src_sel(&mut self) -> SCLK_HF_SRC_SEL_W<0> {
        SCLK_HF_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_src_sel(&mut self) -> SCLK_LF_SRC_SEL_W<2> {
        SCLK_LF_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Source select for aclk_ref 000: RCOSC_HF derived (31.25kHz) 001: XOSC_HF derived (31.25kHz) 010: RCOSC_LF (32kHz) 011: XOSC_LF (32.768kHz) 100: RCOSC_MF (2MHz) 101-111: Not used"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_ref_src_sel(&mut self) -> ACLK_REF_SRC_SEL_W<4> {
        ACLK_REF_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_tdc_src_sel(&mut self) -> ACLK_TDC_SRC_SEL_W<7> {
        ACLK_TDC_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to the system controller. Checks both SCLK_HF, SCLK_MF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    #[must_use]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W<9> {
        CLK_LOSS_EN_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_lf_dig_bypass(&mut self) -> XOSC_LF_DIG_BYPASS_W<10> {
        XOSC_LF_DIG_BYPASS_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_power_mode(&mut self) -> XOSC_HF_POWER_MODE_W<11> {
        XOSC_HF_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_lf_trimmed(&mut self) -> RCOSC_LF_TRIMMED_W<12> {
        RCOSC_LF_TRIMMED_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
0: HPOSC mode is not enabled. The 48 MHz crystal is required for radio operation. 1: Enables HPOSC mode. The internal HPOSC can be used as HF system clock and for radio operation."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_mode_en(&mut self) -> HPOSC_MODE_EN_W<14> {
        HPOSC_MODE_EN_W::new(self)
    }
    #[doc = "Bits 15:23 - 23:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Select DCDC clock source. 0: CLK_DCDC is 48 MHz clock from RCOSC or XOSC / HPOSC 1: CLK_DCDC is always 48 MHz clock from RCOSC"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dcdc_src_sel(&mut self) -> CLK_DCDC_SRC_SEL_W<24> {
        CLK_DCDC_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn doubler_reset_duration(&mut self) -> DOUBLER_RESET_DURATION_W<25> {
        DOUBLER_RESET_DURATION_W::new(self)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn doubler_start_duration(&mut self) -> DOUBLER_START_DURATION_W<26> {
        DOUBLER_START_DURATION_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_rcosc_lf_clk_qual(&mut self) -> BYPASS_RCOSC_LF_CLK_QUAL_W<28> {
        BYPASS_RCOSC_LF_CLK_QUAL_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_xosc_lf_clk_qual(&mut self) -> BYPASS_XOSC_LF_CLK_QUAL_W<29> {
        BYPASS_XOSC_LF_CLK_QUAL_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> RESERVED30_W<30> {
        RESERVED30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn xtal_is_24m(&mut self) -> XTAL_IS_24M_W<31> {
        XTAL_IS_24M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0 Controls clock source selects\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
