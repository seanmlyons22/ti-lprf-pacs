#[doc = "Register `STAT0` reader"]
pub type R = crate::R<Stat0Spec>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<Stat0Spec>;
#[doc = "Field `PENDINGSCLKHFSWITCHING` reader - 0:0\\]
Indicates when sclk_hf is ready to be switched"]
pub type PendingsclkhfswitchingR = crate::BitReader;
#[doc = "Field `PENDINGSCLKHFSWITCHING` writer - 0:0\\]
Indicates when sclk_hf is ready to be switched"]
pub type PendingsclkhfswitchingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_DATA` reader - 6:1\\]
adc_data"]
pub type AdcDataR = crate::FieldReader;
#[doc = "Field `ADC_DATA` writer - 6:1\\]
adc_data"]
pub type AdcDataW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADC_DATA_READY` reader - 7:7\\]
indicates when adc_data is ready."]
pub type AdcDataReadyR = crate::BitReader;
#[doc = "Field `ADC_DATA_READY` writer - 7:7\\]
indicates when adc_data is ready."]
pub type AdcDataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_THMET` reader - 8:8\\]
ADC_THMET"]
pub type AdcThmetR = crate::BitReader;
#[doc = "Field `ADC_THMET` writer - 8:8\\]
ADC_THMET"]
pub type AdcThmetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `RESERVED9` writer - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_HF_HP_BUF_EN` reader - 10:10\\]
XOSC_HF_HP_BUF_EN"]
pub type XoscHfHpBufEnR = crate::BitReader;
#[doc = "Field `XOSC_HF_HP_BUF_EN` writer - 10:10\\]
XOSC_HF_HP_BUF_EN"]
pub type XoscHfHpBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_HF_LP_BUF_EN` reader - 11:11\\]
XOSC_HF_LP_BUF_EN"]
pub type XoscHfLpBufEnR = crate::BitReader;
#[doc = "Field `XOSC_HF_LP_BUF_EN` writer - 11:11\\]
XOSC_HF_LP_BUF_EN"]
pub type XoscHfLpBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XB_48M_CLK_EN` reader - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
pub type Xb48mClkEnR = crate::BitReader;
#[doc = "Field `XB_48M_CLK_EN` writer - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
pub type Xb48mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED14` reader - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `RESERVED14` writer - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_HF_EN` reader - 15:15\\]
Indicates that XOSC_HF is enabled."]
pub type XoscHfEnR = crate::BitReader;
#[doc = "Field `XOSC_HF_EN` writer - 15:15\\]
Indicates that XOSC_HF is enabled."]
pub type XoscHfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_LF_LOSS` reader - 16:16\\]
Indicates sclk_lf is lost"]
pub type SclkLfLossR = crate::BitReader;
#[doc = "Field `SCLK_LF_LOSS` writer - 16:16\\]
Indicates sclk_lf is lost"]
pub type SclkLfLossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_HF_LOSS` reader - 17:17\\]
Indicates sclk_hf is lost"]
pub type SclkHfLossR = crate::BitReader;
#[doc = "Field `SCLK_HF_LOSS` writer - 17:17\\]
Indicates sclk_hf is lost"]
pub type SclkHfLossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DCDC_RDY_ACK` reader - 18:18\\]
CLK_DCDC_RDY_ACK"]
pub type ClkDcdcRdyAckR = crate::BitReader;
#[doc = "Field `CLK_DCDC_RDY_ACK` writer - 18:18\\]
CLK_DCDC_RDY_ACK"]
pub type ClkDcdcRdyAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DCDC_RDY` reader - 19:19\\]
CLK_DCDC_RDY"]
pub type ClkDcdcRdyR = crate::BitReader;
#[doc = "Field `CLK_DCDC_RDY` writer - 19:19\\]
CLK_DCDC_RDY"]
pub type ClkDcdcRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC_LF_EN` reader - 20:20\\]
XOSC_LF_EN"]
pub type XoscLfEnR = crate::BitReader;
#[doc = "Field `XOSC_LF_EN` writer - 20:20\\]
XOSC_LF_EN"]
pub type XoscLfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSC_LF_EN` reader - 21:21\\]
RCOSC_LF_EN"]
pub type RcoscLfEnR = crate::BitReader;
#[doc = "Field `RCOSC_LF_EN` writer - 21:21\\]
RCOSC_LF_EN"]
pub type RcoscLfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSC_HF_EN` reader - 22:22\\]
RCOSC_HF_EN"]
pub type RcoscHfEnR = crate::BitReader;
#[doc = "Field `RCOSC_HF_EN` writer - 22:22\\]
RCOSC_HF_EN"]
pub type RcoscHfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED23` reader - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::FieldReader;
#[doc = "Field `RESERVED23` writer - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "28:28\\]
Indicates source for the sclk_hf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SclkHfSrc {
    #[doc = "1: High frequency XOSC"]
    Xosc = 1,
    #[doc = "0: High frequency RCOSC clock"]
    Rcosc = 0,
}
impl From<SclkHfSrc> for bool {
    #[inline(always)]
    fn from(variant: SclkHfSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLK_HF_SRC` reader - 28:28\\]
Indicates source for the sclk_hf"]
pub type SclkHfSrcR = crate::BitReader<SclkHfSrc>;
impl SclkHfSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclkHfSrc {
        match self.bits {
            true => SclkHfSrc::Xosc,
            false => SclkHfSrc::Rcosc,
        }
    }
    #[doc = "High frequency XOSC"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SclkHfSrc::Xosc
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        *self == SclkHfSrc::Rcosc
    }
}
#[doc = "Field `SCLK_HF_SRC` writer - 28:28\\]
Indicates source for the sclk_hf"]
pub type SclkHfSrcW<'a, REG> = crate::BitWriter<'a, REG, SclkHfSrc>;
impl<'a, REG> SclkHfSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency XOSC"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfSrc::Xosc)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut crate::W<REG> {
        self.variant(SclkHfSrc::Rcosc)
    }
}
#[doc = "30:29\\]
Indicates source for the sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SclkLfSrc {
    #[doc = "3: Low frequency XOSC"]
    Xosclf = 3,
    #[doc = "2: Low frequency RCOSC"]
    Rcosclf = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC"]
    Xoschfdlf = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    Rcoschfdlf = 0,
}
impl From<SclkLfSrc> for u8 {
    #[inline(always)]
    fn from(variant: SclkLfSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SclkLfSrc {
    type Ux = u8;
}
impl crate::IsEnum for SclkLfSrc {}
#[doc = "Field `SCLK_LF_SRC` reader - 30:29\\]
Indicates source for the sclk_lf"]
pub type SclkLfSrcR = crate::FieldReader<SclkLfSrc>;
impl SclkLfSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclkLfSrc {
        match self.bits {
            3 => SclkLfSrc::Xosclf,
            2 => SclkLfSrc::Rcosclf,
            1 => SclkLfSrc::Xoschfdlf,
            0 => SclkLfSrc::Rcoschfdlf,
            _ => unreachable!(),
        }
    }
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        *self == SclkLfSrc::Xosclf
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        *self == SclkLfSrc::Rcosclf
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SclkLfSrc::Xoschfdlf
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SclkLfSrc::Rcoschfdlf
    }
}
#[doc = "Field `SCLK_LF_SRC` writer - 30:29\\]
Indicates source for the sclk_lf"]
pub type SclkLfSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, SclkLfSrc, crate::Safe>;
impl<'a, REG> SclkLfSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrc::Xosclf)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrc::Rcosclf)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrc::Xoschfdlf)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut crate::W<REG> {
        self.variant(SclkLfSrc::Rcoschfdlf)
    }
}
#[doc = "Field `SPARE31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare31R = crate::BitReader;
#[doc = "Field `SPARE31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates when sclk_hf is ready to be switched"]
    #[inline(always)]
    pub fn pendingsclkhfswitching(&self) -> PendingsclkhfswitchingR {
        PendingsclkhfswitchingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
adc_data"]
    #[inline(always)]
    pub fn adc_data(&self) -> AdcDataR {
        AdcDataR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
indicates when adc_data is ready."]
    #[inline(always)]
    pub fn adc_data_ready(&self) -> AdcDataReadyR {
        AdcDataReadyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC_THMET"]
    #[inline(always)]
    pub fn adc_thmet(&self) -> AdcThmetR {
        AdcThmetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_hp_buf_en(&self) -> XoscHfHpBufEnR {
        XoscHfHpBufEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_lp_buf_en(&self) -> XoscHfLpBufEnR {
        XoscHfLpBufEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    pub fn xb_48m_clk_en(&self) -> Xb48mClkEnR {
        Xb48mClkEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    pub fn xosc_hf_en(&self) -> XoscHfEnR {
        XoscHfEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates sclk_lf is lost"]
    #[inline(always)]
    pub fn sclk_lf_loss(&self) -> SclkLfLossR {
        SclkLfLossR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates sclk_hf is lost"]
    #[inline(always)]
    pub fn sclk_hf_loss(&self) -> SclkHfLossR {
        SclkHfLossR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    pub fn clk_dcdc_rdy_ack(&self) -> ClkDcdcRdyAckR {
        ClkDcdcRdyAckR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
CLK_DCDC_RDY"]
    #[inline(always)]
    pub fn clk_dcdc_rdy(&self) -> ClkDcdcRdyR {
        ClkDcdcRdyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
XOSC_LF_EN"]
    #[inline(always)]
    pub fn xosc_lf_en(&self) -> XoscLfEnR {
        XoscLfEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
RCOSC_LF_EN"]
    #[inline(always)]
    pub fn rcosc_lf_en(&self) -> RcoscLfEnR {
        RcoscLfEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
RCOSC_HF_EN"]
    #[inline(always)]
    pub fn rcosc_hf_en(&self) -> RcoscHfEnR {
        RcoscHfEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Indicates source for the sclk_hf"]
    #[inline(always)]
    pub fn sclk_hf_src(&self) -> SclkHfSrcR {
        SclkHfSrcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Indicates source for the sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src(&self) -> SclkLfSrcR {
        SclkLfSrcR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&self) -> Spare31R {
        Spare31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates when sclk_hf is ready to be switched"]
    #[inline(always)]
    #[must_use]
    pub fn pendingsclkhfswitching(&mut self) -> PendingsclkhfswitchingW<Stat0Spec> {
        PendingsclkhfswitchingW::new(self, 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
adc_data"]
    #[inline(always)]
    #[must_use]
    pub fn adc_data(&mut self) -> AdcDataW<Stat0Spec> {
        AdcDataW::new(self, 1)
    }
    #[doc = "Bit 7 - 7:7\\]
indicates when adc_data is ready."]
    #[inline(always)]
    #[must_use]
    pub fn adc_data_ready(&mut self) -> AdcDataReadyW<Stat0Spec> {
        AdcDataReadyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC_THMET"]
    #[inline(always)]
    #[must_use]
    pub fn adc_thmet(&mut self) -> AdcThmetW<Stat0Spec> {
        AdcThmetW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Stat0Spec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_hp_buf_en(&mut self) -> XoscHfHpBufEnW<Stat0Spec> {
        XoscHfHpBufEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_lp_buf_en(&mut self) -> XoscHfLpBufEnW<Stat0Spec> {
        XoscHfLpBufEnW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<Stat0Spec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    #[must_use]
    pub fn xb_48m_clk_en(&mut self) -> Xb48mClkEnW<Stat0Spec> {
        Xb48mClkEnW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<Stat0Spec> {
        Reserved14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_en(&mut self) -> XoscHfEnW<Stat0Spec> {
        XoscHfEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates sclk_lf is lost"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_loss(&mut self) -> SclkLfLossW<Stat0Spec> {
        SclkLfLossW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates sclk_hf is lost"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_loss(&mut self) -> SclkHfLossW<Stat0Spec> {
        SclkHfLossW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dcdc_rdy_ack(&mut self) -> ClkDcdcRdyAckW<Stat0Spec> {
        ClkDcdcRdyAckW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
CLK_DCDC_RDY"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dcdc_rdy(&mut self) -> ClkDcdcRdyW<Stat0Spec> {
        ClkDcdcRdyW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
XOSC_LF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_lf_en(&mut self) -> XoscLfEnW<Stat0Spec> {
        XoscLfEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
RCOSC_LF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_lf_en(&mut self) -> RcoscLfEnW<Stat0Spec> {
        RcoscLfEnW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
RCOSC_HF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_hf_en(&mut self) -> RcoscHfEnW<Stat0Spec> {
        RcoscHfEnW::new(self, 22)
    }
    #[doc = "Bits 23:27 - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<Stat0Spec> {
        Reserved23W::new(self, 23)
    }
    #[doc = "Bit 28 - 28:28\\]
Indicates source for the sclk_hf"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_hf_src(&mut self) -> SclkHfSrcW<Stat0Spec> {
        SclkHfSrcW::new(self, 28)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Indicates source for the sclk_lf"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_lf_src(&mut self) -> SclkLfSrcW<Stat0Spec> {
        SclkLfSrcW::new(self, 29)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare31(&mut self) -> Spare31W<Stat0Spec> {
        Spare31W::new(self, 31)
    }
}
#[doc = "Status 0 This register contains status signals from OSC_DIG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat0Spec;
impl crate::RegisterSpec for Stat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for Stat0Spec {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for Stat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for Stat0Spec {
    const RESET_VALUE: u32 = 0;
}
