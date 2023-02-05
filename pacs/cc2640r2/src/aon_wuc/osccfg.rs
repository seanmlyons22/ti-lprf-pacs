#[doc = "Register `OSCCFG` reader"]
pub struct R(crate::R<OSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCFG` writer"]
pub struct W(crate::W<OSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCFG_SPEC>;
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
impl From<crate::W<OSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_E` reader - 2:0\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0"]
pub type PER_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_E` writer - 2:0\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0"]
pub type PER_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PER_M` reader - 7:3\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0"]
pub type PER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_M` writer - 7:3\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0"]
pub type PER_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0"]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0"]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0"]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PER_E_W<0> {
        PER_E_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0"]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PER_M_W<3> {
        PER_M_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccfg](index.html) module"]
pub struct OSCCFG_SPEC;
impl crate::RegisterSpec for OSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osccfg::R](R) reader structure"]
impl crate::Readable for OSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osccfg::W](W) writer structure"]
impl crate::Writable for OSCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCFG to value 0"]
impl crate::Resettable for OSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
