#[doc = "Register `STAT1` reader"]
pub type R = crate::R<Stat1Spec>;
#[doc = "Register `STAT1` writer"]
pub type W = crate::W<Stat1Spec>;
#[doc = "Field `CLK_DCDC_GOOD` reader - 0:0\\]
CLK_DCDC_GOOD"]
pub type ClkDcdcGoodR = crate::BitReader;
#[doc = "Field `CLK_CHP_GOOD` reader - 1:1\\]
CLK_CHP_GOOD"]
pub type ClkChpGoodR = crate::BitReader;
#[doc = "Field `ACLK_REF_GOOD` reader - 2:2\\]
ACLK_REF_GOOD"]
pub type AclkRefGoodR = crate::BitReader;
#[doc = "Field `ACLK_TDC_GOOD` reader - 3:3\\]
ACLK_TDC_GOOD"]
pub type AclkTdcGoodR = crate::BitReader;
#[doc = "Field `ACLK_ADC_GOOD` reader - 4:4\\]
ACLK_ADC_GOOD"]
pub type AclkAdcGoodR = crate::BitReader;
#[doc = "Field `SCLK_LF_GOOD` reader - 5:5\\]
SCLK_LF_GOOD"]
pub type SclkLfGoodR = crate::BitReader;
#[doc = "Field `SCLK_MF_GOOD` reader - 6:6\\]
SCLK_MF_GOOD"]
pub type SclkMfGoodR = crate::BitReader;
#[doc = "Field `SCLK_HF_GOOD` reader - 7:7\\]
SCLK_HF_GOOD"]
pub type SclkHfGoodR = crate::BitReader;
#[doc = "Field `CLK_DCDC_EN` reader - 8:8\\]
CLK_DCDC_EN"]
pub type ClkDcdcEnR = crate::BitReader;
#[doc = "Field `CLK_CHP_EN` reader - 9:9\\]
CLK_CHP_EN"]
pub type ClkChpEnR = crate::BitReader;
#[doc = "Field `ACLK_REF_EN` reader - 10:10\\]
ACLK_REF_EN"]
pub type AclkRefEnR = crate::BitReader;
#[doc = "Field `ACLK_TDC_EN` reader - 11:11\\]
ACLK_TDC_EN"]
pub type AclkTdcEnR = crate::BitReader;
#[doc = "Field `ACLK_ADC_EN` reader - 12:12\\]
ACLK_ADC_EN"]
pub type AclkAdcEnR = crate::BitReader;
#[doc = "Field `SCLK_MF_EN` reader - 13:13\\]
SCLK_MF_EN"]
pub type SclkMfEnR = crate::BitReader;
#[doc = "Field `SCLK_HF_EN` reader - 14:14\\]
SCLK_HF_EN"]
pub type SclkHfEnR = crate::BitReader;
#[doc = "Field `FORCE_RCOSC_HF` reader - 15:15\\]
force_rcosc_hf"]
pub type ForceRcoscHfR = crate::BitReader;
#[doc = "Field `LPM_UPDATE_AMP` reader - 21:16\\]
OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
pub type LpmUpdateAmpR = crate::FieldReader;
#[doc = "Field `HPM_UPDATE_AMP` reader - 27:22\\]
OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
pub type HpmUpdateAmpR = crate::FieldReader;
#[doc = "31:28\\]
AMPCOMP FSM State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampstate {
    #[doc = "14: FAST_START_SETTLE"]
    FastStartSettle = 14,
    #[doc = "13: FAST_START"]
    FastStart = 13,
    #[doc = "12: DUMMY_TO_INIT_1"]
    DummyToInit1 = 12,
    #[doc = "11: IDAC_DECREMENT_WITH_MEASURE"]
    IdacDecWMeasure = 11,
    #[doc = "10: IBIAS_INCREMENT"]
    IbiasInc = 10,
    #[doc = "9: LPM_UPDATE"]
    LpmUpdate = 9,
    #[doc = "8: IBIAS_DECREMENT_WITH_MEASURE"]
    IbiasDecWMeasure = 8,
    #[doc = "7: IBIAS_CAP_UPDATE"]
    IbiasCapUpdate = 7,
    #[doc = "6: IDAC_INCREMENT"]
    IdacIncrement = 6,
    #[doc = "5: HPM_UPDATE"]
    HpmUpdate = 5,
    #[doc = "4: HPM_RAMP3"]
    HpmRamp3 = 4,
    #[doc = "3: HPM_RAMP2"]
    HpmRamp2 = 3,
    #[doc = "2: HPM_RAMP1"]
    HpmRamp1 = 2,
    #[doc = "1: INITIALIZATION"]
    Initialization = 1,
    #[doc = "0: RESET"]
    Reset = 0,
}
impl From<Rampstate> for u8 {
    #[inline(always)]
    fn from(variant: Rampstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rampstate {
    type Ux = u8;
}
impl crate::IsEnum for Rampstate {}
#[doc = "Field `RAMPSTATE` reader - 31:28\\]
AMPCOMP FSM State"]
pub type RampstateR = crate::FieldReader<Rampstate>;
impl RampstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rampstate> {
        match self.bits {
            14 => Some(Rampstate::FastStartSettle),
            13 => Some(Rampstate::FastStart),
            12 => Some(Rampstate::DummyToInit1),
            11 => Some(Rampstate::IdacDecWMeasure),
            10 => Some(Rampstate::IbiasInc),
            9 => Some(Rampstate::LpmUpdate),
            8 => Some(Rampstate::IbiasDecWMeasure),
            7 => Some(Rampstate::IbiasCapUpdate),
            6 => Some(Rampstate::IdacIncrement),
            5 => Some(Rampstate::HpmUpdate),
            4 => Some(Rampstate::HpmRamp3),
            3 => Some(Rampstate::HpmRamp2),
            2 => Some(Rampstate::HpmRamp1),
            1 => Some(Rampstate::Initialization),
            0 => Some(Rampstate::Reset),
            _ => None,
        }
    }
    #[doc = "FAST_START_SETTLE"]
    #[inline(always)]
    pub fn is_fast_start_settle(&self) -> bool {
        *self == Rampstate::FastStartSettle
    }
    #[doc = "FAST_START"]
    #[inline(always)]
    pub fn is_fast_start(&self) -> bool {
        *self == Rampstate::FastStart
    }
    #[doc = "DUMMY_TO_INIT_1"]
    #[inline(always)]
    pub fn is_dummy_to_init_1(&self) -> bool {
        *self == Rampstate::DummyToInit1
    }
    #[doc = "IDAC_DECREMENT_WITH_MEASURE"]
    #[inline(always)]
    pub fn is_idac_dec_w_measure(&self) -> bool {
        *self == Rampstate::IdacDecWMeasure
    }
    #[doc = "IBIAS_INCREMENT"]
    #[inline(always)]
    pub fn is_ibias_inc(&self) -> bool {
        *self == Rampstate::IbiasInc
    }
    #[doc = "LPM_UPDATE"]
    #[inline(always)]
    pub fn is_lpm_update(&self) -> bool {
        *self == Rampstate::LpmUpdate
    }
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE"]
    #[inline(always)]
    pub fn is_ibias_dec_w_measure(&self) -> bool {
        *self == Rampstate::IbiasDecWMeasure
    }
    #[doc = "IBIAS_CAP_UPDATE"]
    #[inline(always)]
    pub fn is_ibias_cap_update(&self) -> bool {
        *self == Rampstate::IbiasCapUpdate
    }
    #[doc = "IDAC_INCREMENT"]
    #[inline(always)]
    pub fn is_idac_increment(&self) -> bool {
        *self == Rampstate::IdacIncrement
    }
    #[doc = "HPM_UPDATE"]
    #[inline(always)]
    pub fn is_hpm_update(&self) -> bool {
        *self == Rampstate::HpmUpdate
    }
    #[doc = "HPM_RAMP3"]
    #[inline(always)]
    pub fn is_hpm_ramp3(&self) -> bool {
        *self == Rampstate::HpmRamp3
    }
    #[doc = "HPM_RAMP2"]
    #[inline(always)]
    pub fn is_hpm_ramp2(&self) -> bool {
        *self == Rampstate::HpmRamp2
    }
    #[doc = "HPM_RAMP1"]
    #[inline(always)]
    pub fn is_hpm_ramp1(&self) -> bool {
        *self == Rampstate::HpmRamp1
    }
    #[doc = "INITIALIZATION"]
    #[inline(always)]
    pub fn is_initialization(&self) -> bool {
        *self == Rampstate::Initialization
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Rampstate::Reset
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
CLK_DCDC_GOOD"]
    #[inline(always)]
    pub fn clk_dcdc_good(&self) -> ClkDcdcGoodR {
        ClkDcdcGoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLK_CHP_GOOD"]
    #[inline(always)]
    pub fn clk_chp_good(&self) -> ClkChpGoodR {
        ClkChpGoodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ACLK_REF_GOOD"]
    #[inline(always)]
    pub fn aclk_ref_good(&self) -> AclkRefGoodR {
        AclkRefGoodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ACLK_TDC_GOOD"]
    #[inline(always)]
    pub fn aclk_tdc_good(&self) -> AclkTdcGoodR {
        AclkTdcGoodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ACLK_ADC_GOOD"]
    #[inline(always)]
    pub fn aclk_adc_good(&self) -> AclkAdcGoodR {
        AclkAdcGoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SCLK_LF_GOOD"]
    #[inline(always)]
    pub fn sclk_lf_good(&self) -> SclkLfGoodR {
        SclkLfGoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_MF_GOOD"]
    #[inline(always)]
    pub fn sclk_mf_good(&self) -> SclkMfGoodR {
        SclkMfGoodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF_GOOD"]
    #[inline(always)]
    pub fn sclk_hf_good(&self) -> SclkHfGoodR {
        SclkHfGoodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CLK_DCDC_EN"]
    #[inline(always)]
    pub fn clk_dcdc_en(&self) -> ClkDcdcEnR {
        ClkDcdcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
CLK_CHP_EN"]
    #[inline(always)]
    pub fn clk_chp_en(&self) -> ClkChpEnR {
        ClkChpEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
ACLK_REF_EN"]
    #[inline(always)]
    pub fn aclk_ref_en(&self) -> AclkRefEnR {
        AclkRefEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
ACLK_TDC_EN"]
    #[inline(always)]
    pub fn aclk_tdc_en(&self) -> AclkTdcEnR {
        AclkTdcEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
ACLK_ADC_EN"]
    #[inline(always)]
    pub fn aclk_adc_en(&self) -> AclkAdcEnR {
        AclkAdcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
SCLK_MF_EN"]
    #[inline(always)]
    pub fn sclk_mf_en(&self) -> SclkMfEnR {
        SclkMfEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
SCLK_HF_EN"]
    #[inline(always)]
    pub fn sclk_hf_en(&self) -> SclkHfEnR {
        SclkHfEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
force_rcosc_hf"]
    #[inline(always)]
    pub fn force_rcosc_hf(&self) -> ForceRcoscHfR {
        ForceRcoscHfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub fn lpm_update_amp(&self) -> LpmUpdateAmpR {
        LpmUpdateAmpR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - 27:22\\]
OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub fn hpm_update_amp(&self) -> HpmUpdateAmpR {
        HpmUpdateAmpR::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
AMPCOMP FSM State"]
    #[inline(always)]
    pub fn rampstate(&self) -> RampstateR {
        RampstateR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Status 1 This register contains status signals from OSC_DIG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat1Spec;
impl crate::RegisterSpec for Stat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for Stat1Spec {}
#[doc = "`write(|w| ..)` method takes [`stat1::W`](W) writer structure"]
impl crate::Writable for Stat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for Stat1Spec {
    const RESET_VALUE: u32 = 0;
}
