#[doc = "Register `STAT0` reader"]
pub struct R(crate::R<STAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT0` writer"]
pub struct W(crate::W<STAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT0_SPEC>;
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
impl From<crate::W<STAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PENDINGSCLKHFSWITCHING` reader - 0:0\\]
Indicates when SCLK_HF clock source is ready to be switched"]
pub type PENDINGSCLKHFSWITCHING_R = crate::BitReader<bool>;
#[doc = "Field `PENDINGSCLKHFSWITCHING` writer - 0:0\\]
Indicates when SCLK_HF clock source is ready to be switched"]
pub type PENDINGSCLKHFSWITCHING_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `ADC_DATA` reader - 6:1\\]
adc_data"]
pub type ADC_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_DATA` writer - 6:1\\]
adc_data"]
pub type ADC_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT0_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADC_DATA_READY` reader - 7:7\\]
indicates when adc_data is ready."]
pub type ADC_DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DATA_READY` writer - 7:7\\]
indicates when adc_data is ready."]
pub type ADC_DATA_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `ADC_THMET` reader - 8:8\\]
ADC_THMET"]
pub type ADC_THMET_R = crate::BitReader<bool>;
#[doc = "Field `ADC_THMET` writer - 8:8\\]
ADC_THMET"]
pub type ADC_THMET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED9` writer - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_HP_BUF_EN` reader - 10:10\\]
XOSC_HF_HP_BUF_EN"]
pub type XOSC_HF_HP_BUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_HP_BUF_EN` writer - 10:10\\]
XOSC_HF_HP_BUF_EN"]
pub type XOSC_HF_HP_BUF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_LP_BUF_EN` reader - 11:11\\]
XOSC_HF_LP_BUF_EN"]
pub type XOSC_HF_LP_BUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_LP_BUF_EN` writer - 11:11\\]
XOSC_HF_LP_BUF_EN"]
pub type XOSC_HF_LP_BUF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `XB_48M_CLK_EN` reader - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
pub type XB_48M_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `XB_48M_CLK_EN` writer - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
pub type XB_48M_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `RESERVED14` reader - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED14` writer - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `XOSC_HF_EN` reader - 15:15\\]
Indicates that XOSC_HF is enabled."]
pub type XOSC_HF_EN_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_HF_EN` writer - 15:15\\]
Indicates that XOSC_HF is enabled."]
pub type XOSC_HF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `SCLK_LF_LOSS` reader - 16:16\\]
Indicates sclk_lf is lost"]
pub type SCLK_LF_LOSS_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_LF_LOSS` writer - 16:16\\]
Indicates sclk_lf is lost"]
pub type SCLK_LF_LOSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `SCLK_HF_LOSS` reader - 17:17\\]
Indicates sclk_hf is lost"]
pub type SCLK_HF_LOSS_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_HF_LOSS` writer - 17:17\\]
Indicates sclk_hf is lost"]
pub type SCLK_HF_LOSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `CLK_DCDC_RDY_ACK` reader - 18:18\\]
CLK_DCDC_RDY_ACK"]
pub type CLK_DCDC_RDY_ACK_R = crate::BitReader<bool>;
#[doc = "Field `CLK_DCDC_RDY_ACK` writer - 18:18\\]
CLK_DCDC_RDY_ACK"]
pub type CLK_DCDC_RDY_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `CLK_DCDC_RDY` reader - 19:19\\]
CLK_DCDC_RDY"]
pub type CLK_DCDC_RDY_R = crate::BitReader<bool>;
#[doc = "Field `CLK_DCDC_RDY` writer - 19:19\\]
CLK_DCDC_RDY"]
pub type CLK_DCDC_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `XOSC_LF_EN` reader - 20:20\\]
XOSC_LF_EN"]
pub type XOSC_LF_EN_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_LF_EN` writer - 20:20\\]
XOSC_LF_EN"]
pub type XOSC_LF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `RCOSC_LF_EN` reader - 21:21\\]
RCOSC_LF_EN"]
pub type RCOSC_LF_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_LF_EN` writer - 21:21\\]
RCOSC_LF_EN"]
pub type RCOSC_LF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `RCOSC_HF_EN` reader - 22:22\\]
RCOSC_HF_EN"]
pub type RCOSC_HF_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_HF_EN` writer - 22:22\\]
RCOSC_HF_EN"]
pub type RCOSC_HF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `RESERVED23` reader - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED23` writer - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT0_SPEC, u8, u8, 5, O>;
#[doc = "Field `SCLK_HF_SRC` reader - 28:28\\]
Indicates source for the sclk_hf"]
pub type SCLK_HF_SRC_R = crate::BitReader<SCLK_HF_SRC_A>;
#[doc = "28:28\\]
Indicates source for the sclk_hf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLK_HF_SRC_A {
    #[doc = "1: High frequency XOSC"]
    XOSC = 1,
    #[doc = "0: High frequency RCOSC clock"]
    RCOSC = 0,
}
impl From<SCLK_HF_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_HF_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLK_HF_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_SRC_A {
        match self.bits {
            true => SCLK_HF_SRC_A::XOSC,
            false => SCLK_HF_SRC_A::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SCLK_HF_SRC_A::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        *self == SCLK_HF_SRC_A::RCOSC
    }
}
#[doc = "Field `SCLK_HF_SRC` writer - 28:28\\]
Indicates source for the sclk_hf"]
pub type SCLK_HF_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, SCLK_HF_SRC_A, O>;
impl<'a, const O: u8> SCLK_HF_SRC_W<'a, O> {
    #[doc = "High frequency XOSC"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_A::XOSC)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_A::RCOSC)
    }
}
#[doc = "Field `SCLK_LF_SRC` reader - 30:29\\]
Indicates source for the sclk_lf"]
pub type SCLK_LF_SRC_R = crate::FieldReader<u8, SCLK_LF_SRC_A>;
#[doc = "30:29\\]
Indicates source for the sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLK_LF_SRC_A {
    #[doc = "3: Low frequency XOSC"]
    XOSCLF = 3,
    #[doc = "2: Low frequency RCOSC"]
    RCOSCLF = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF = 0,
}
impl From<SCLK_LF_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_SRC_A) -> Self {
        variant as _
    }
}
impl SCLK_LF_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_SRC_A {
        match self.bits {
            3 => SCLK_LF_SRC_A::XOSCLF,
            2 => SCLK_LF_SRC_A::RCOSCLF,
            1 => SCLK_LF_SRC_A::XOSCHFDLF,
            0 => SCLK_LF_SRC_A::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        *self == SCLK_LF_SRC_A::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        *self == SCLK_LF_SRC_A::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_A::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_A::RCOSCHFDLF
    }
}
#[doc = "Field `SCLK_LF_SRC` writer - 30:29\\]
Indicates source for the sclk_lf"]
pub type SCLK_LF_SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, STAT0_SPEC, u8, SCLK_LF_SRC_A, 2, O>;
impl<'a, const O: u8> SCLK_LF_SRC_W<'a, O> {
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::XOSCLF)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::RCOSCLF)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::XOSCHFDLF)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::RCOSCHFDLF)
    }
}
#[doc = "Field `SPARE31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE31_R = crate::BitReader<bool>;
#[doc = "Field `SPARE31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates when SCLK_HF clock source is ready to be switched"]
    #[inline(always)]
    pub fn pendingsclkhfswitching(&self) -> PENDINGSCLKHFSWITCHING_R {
        PENDINGSCLKHFSWITCHING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
adc_data"]
    #[inline(always)]
    pub fn adc_data(&self) -> ADC_DATA_R {
        ADC_DATA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
indicates when adc_data is ready."]
    #[inline(always)]
    pub fn adc_data_ready(&self) -> ADC_DATA_READY_R {
        ADC_DATA_READY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC_THMET"]
    #[inline(always)]
    pub fn adc_thmet(&self) -> ADC_THMET_R {
        ADC_THMET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_hp_buf_en(&self) -> XOSC_HF_HP_BUF_EN_R {
        XOSC_HF_HP_BUF_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_lp_buf_en(&self) -> XOSC_HF_LP_BUF_EN_R {
        XOSC_HF_LP_BUF_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    pub fn xb_48m_clk_en(&self) -> XB_48M_CLK_EN_R {
        XB_48M_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    pub fn xosc_hf_en(&self) -> XOSC_HF_EN_R {
        XOSC_HF_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates sclk_lf is lost"]
    #[inline(always)]
    pub fn sclk_lf_loss(&self) -> SCLK_LF_LOSS_R {
        SCLK_LF_LOSS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates sclk_hf is lost"]
    #[inline(always)]
    pub fn sclk_hf_loss(&self) -> SCLK_HF_LOSS_R {
        SCLK_HF_LOSS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    pub fn clk_dcdc_rdy_ack(&self) -> CLK_DCDC_RDY_ACK_R {
        CLK_DCDC_RDY_ACK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
CLK_DCDC_RDY"]
    #[inline(always)]
    pub fn clk_dcdc_rdy(&self) -> CLK_DCDC_RDY_R {
        CLK_DCDC_RDY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
XOSC_LF_EN"]
    #[inline(always)]
    pub fn xosc_lf_en(&self) -> XOSC_LF_EN_R {
        XOSC_LF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
RCOSC_LF_EN"]
    #[inline(always)]
    pub fn rcosc_lf_en(&self) -> RCOSC_LF_EN_R {
        RCOSC_LF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
RCOSC_HF_EN"]
    #[inline(always)]
    pub fn rcosc_hf_en(&self) -> RCOSC_HF_EN_R {
        RCOSC_HF_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Indicates source for the sclk_hf"]
    #[inline(always)]
    pub fn sclk_hf_src(&self) -> SCLK_HF_SRC_R {
        SCLK_HF_SRC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Indicates source for the sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src(&self) -> SCLK_LF_SRC_R {
        SCLK_LF_SRC_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&self) -> SPARE31_R {
        SPARE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates when SCLK_HF clock source is ready to be switched"]
    #[inline(always)]
    #[must_use]
    pub fn pendingsclkhfswitching(&mut self) -> PENDINGSCLKHFSWITCHING_W<0> {
        PENDINGSCLKHFSWITCHING_W::new(self)
    }
    #[doc = "Bits 1:6 - 6:1\\]
adc_data"]
    #[inline(always)]
    #[must_use]
    pub fn adc_data(&mut self) -> ADC_DATA_W<1> {
        ADC_DATA_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
indicates when adc_data is ready."]
    #[inline(always)]
    #[must_use]
    pub fn adc_data_ready(&mut self) -> ADC_DATA_READY_W<7> {
        ADC_DATA_READY_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC_THMET"]
    #[inline(always)]
    #[must_use]
    pub fn adc_thmet(&mut self) -> ADC_THMET_W<8> {
        ADC_THMET_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_hp_buf_en(&mut self) -> XOSC_HF_HP_BUF_EN_W<10> {
        XOSC_HF_HP_BUF_EN_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_lp_buf_en(&mut self) -> XOSC_HF_LP_BUF_EN_W<11> {
        XOSC_HF_LP_BUF_EN_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    #[must_use]
    pub fn xb_48m_clk_en(&mut self) -> XB_48M_CLK_EN_W<13> {
        XB_48M_CLK_EN_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_en(&mut self) -> XOSC_HF_EN_W<15> {
        XOSC_HF_EN_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates sclk_lf is lost"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_loss(&mut self) -> SCLK_LF_LOSS_W<16> {
        SCLK_LF_LOSS_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates sclk_hf is lost"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_loss(&mut self) -> SCLK_HF_LOSS_W<17> {
        SCLK_HF_LOSS_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dcdc_rdy_ack(&mut self) -> CLK_DCDC_RDY_ACK_W<18> {
        CLK_DCDC_RDY_ACK_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
CLK_DCDC_RDY"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dcdc_rdy(&mut self) -> CLK_DCDC_RDY_W<19> {
        CLK_DCDC_RDY_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
XOSC_LF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_lf_en(&mut self) -> XOSC_LF_EN_W<20> {
        XOSC_LF_EN_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
RCOSC_LF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_lf_en(&mut self) -> RCOSC_LF_EN_W<21> {
        RCOSC_LF_EN_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
RCOSC_HF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_hf_en(&mut self) -> RCOSC_HF_EN_W<22> {
        RCOSC_HF_EN_W::new(self)
    }
    #[doc = "Bits 23:27 - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> RESERVED23_W<23> {
        RESERVED23_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Indicates source for the sclk_hf"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_src(&mut self) -> SCLK_HF_SRC_W<28> {
        SCLK_HF_SRC_W::new(self)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Indicates source for the sclk_lf"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_src(&mut self) -> SCLK_LF_SRC_W<29> {
        SCLK_LF_SRC_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare31(&mut self) -> SPARE31_W<31> {
        SPARE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status 0 This register contains status signals from OSC_DIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](index.html) module"]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat0::R](R) reader structure"]
impl crate::Readable for STAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat0::W](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
