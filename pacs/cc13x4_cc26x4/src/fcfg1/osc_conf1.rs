#[doc = "Register `OSC_CONF1` reader"]
pub type R = crate::R<OscConf1Spec>;
#[doc = "Register `OSC_CONF1` writer"]
pub type W = crate::W<OscConf1Spec>;
#[doc = "Field `RCOSC_MF_BIAS_ADJ` reader - 3:0\\]
Value is written to DDI_0_OSC:RCOSCMFCTL.RCOSC_MF_BIAS_ADJ by boot FW while in safezone."]
pub type RcoscMfBiasAdjR = crate::FieldReader;
#[doc = "Field `RESERVED4` reader - 25:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RCOSC_MF_SINGLE_TRIM_METHOD` reader - 26:26\\]
Defines trim method used: 0: Dual trim method 1: Single trim method"]
pub type RcoscMfSingleTrimMethodR = crate::BitReader;
#[doc = "Field `RCOSC_MF_TEMP_DEPEND_MODE` reader - 27:27\\]
Defines whether dual trim was needed: 0: Dual trims needed on this chip 1: Dual trims not needed on this chip"]
pub type RcoscMfTempDependModeR = crate::BitReader;
#[doc = "Field `RCOSC_MF_BIAS_HTEMP` reader - 31:28\\]
Defines the MF_BIAS trim code to use for high temp. Only valid if RCOSC_MF_SINGLE_TRIM_METHOD == 0"]
pub type RcoscMfBiasHtempR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Value is written to DDI_0_OSC:RCOSCMFCTL.RCOSC_MF_BIAS_ADJ by boot FW while in safezone."]
    #[inline(always)]
    pub fn rcosc_mf_bias_adj(&self) -> RcoscMfBiasAdjR {
        RcoscMfBiasAdjR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:25 - 25:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x003f_ffff)
    }
    #[doc = "Bit 26 - 26:26\\]
Defines trim method used: 0: Dual trim method 1: Single trim method"]
    #[inline(always)]
    pub fn rcosc_mf_single_trim_method(&self) -> RcoscMfSingleTrimMethodR {
        RcoscMfSingleTrimMethodR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Defines whether dual trim was needed: 0: Dual trims needed on this chip 1: Dual trims not needed on this chip"]
    #[inline(always)]
    pub fn rcosc_mf_temp_depend_mode(&self) -> RcoscMfTempDependModeR {
        RcoscMfTempDependModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Defines the MF_BIAS trim code to use for high temp. Only valid if RCOSC_MF_SINGLE_TRIM_METHOD == 0"]
    #[inline(always)]
    pub fn rcosc_mf_bias_htemp(&self) -> RcoscMfBiasHtempR {
        RcoscMfBiasHtempR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Oscillator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscConf1Spec;
impl crate::RegisterSpec for OscConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc_conf1::R`](R) reader structure"]
impl crate::Readable for OscConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`osc_conf1::W`](W) writer structure"]
impl crate::Writable for OscConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSC_CONF1 to value 0x03ff_0000"]
impl crate::Resettable for OscConf1Spec {
    const RESET_VALUE: u32 = 0x03ff_0000;
}
