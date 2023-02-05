#[doc = "Register `ADC0` reader"]
pub struct R(crate::R<ADC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0` writer"]
pub struct W(crate::W<ADC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0_SPEC>;
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
impl From<crate::W<ADC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADC0_SPEC, bool, O>;
#[doc = "Field `RESET_N` reader - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
pub type RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `RESET_N` writer - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
pub type RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADC0_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADC0_SPEC, bool, O>;
#[doc = "Field `SMPL_CYCLE_EXP` reader - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
pub type SMPL_CYCLE_EXP_R = crate::FieldReader<u8, SMPL_CYCLE_EXP_A>;
#[doc = "6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMPL_CYCLE_EXP_A {
    #[doc = "15: 65536x 6 MHz clock periods = 10.9ms"]
    _10P9_MS = 15,
    #[doc = "14: 32768x 6 MHz clock periods = 5.46ms"]
    _5P46_MS = 14,
    #[doc = "13: 16384x 6 MHz clock periods = 2.73ms"]
    _2P73_MS = 13,
    #[doc = "12: 8192x 6 MHz clock periods = 1.37ms"]
    _1P37_MS = 12,
    #[doc = "11: 4096x 6 MHz clock periods = 682us"]
    _682_US = 11,
    #[doc = "10: 2048x 6 MHz clock periods = 341us"]
    _341_US = 10,
    #[doc = "9: 1024x 6 MHz clock periods = 170us"]
    _170_US = 9,
    #[doc = "8: 512x 6 MHz clock periods = 85.3us"]
    _85P3_US = 8,
    #[doc = "7: 256x 6 MHz clock periods = 42.6us"]
    _42P6_US = 7,
    #[doc = "6: 128x 6 MHz clock periods = 21.3us"]
    _21P3_US = 6,
    #[doc = "5: 64x 6 MHz clock periods = 10.6us"]
    _10P6_US = 5,
    #[doc = "4: 32x 6 MHz clock periods = 5.3us"]
    _5P3_US = 4,
    #[doc = "3: 16x 6 MHz clock periods = 2.7us"]
    _2P7_US = 3,
}
impl From<SMPL_CYCLE_EXP_A> for u8 {
    #[inline(always)]
    fn from(variant: SMPL_CYCLE_EXP_A) -> Self {
        variant as _
    }
}
impl SMPL_CYCLE_EXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMPL_CYCLE_EXP_A> {
        match self.bits {
            15 => Some(SMPL_CYCLE_EXP_A::_10P9_MS),
            14 => Some(SMPL_CYCLE_EXP_A::_5P46_MS),
            13 => Some(SMPL_CYCLE_EXP_A::_2P73_MS),
            12 => Some(SMPL_CYCLE_EXP_A::_1P37_MS),
            11 => Some(SMPL_CYCLE_EXP_A::_682_US),
            10 => Some(SMPL_CYCLE_EXP_A::_341_US),
            9 => Some(SMPL_CYCLE_EXP_A::_170_US),
            8 => Some(SMPL_CYCLE_EXP_A::_85P3_US),
            7 => Some(SMPL_CYCLE_EXP_A::_42P6_US),
            6 => Some(SMPL_CYCLE_EXP_A::_21P3_US),
            5 => Some(SMPL_CYCLE_EXP_A::_10P6_US),
            4 => Some(SMPL_CYCLE_EXP_A::_5P3_US),
            3 => Some(SMPL_CYCLE_EXP_A::_2P7_US),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10P9_MS`"]
    #[inline(always)]
    pub fn is_10p9_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_10P9_MS
    }
    #[doc = "Checks if the value of the field is `_5P46_MS`"]
    #[inline(always)]
    pub fn is_5p46_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_5P46_MS
    }
    #[doc = "Checks if the value of the field is `_2P73_MS`"]
    #[inline(always)]
    pub fn is_2p73_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_2P73_MS
    }
    #[doc = "Checks if the value of the field is `_1P37_MS`"]
    #[inline(always)]
    pub fn is_1p37_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_1P37_MS
    }
    #[doc = "Checks if the value of the field is `_682_US`"]
    #[inline(always)]
    pub fn is_682_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_682_US
    }
    #[doc = "Checks if the value of the field is `_341_US`"]
    #[inline(always)]
    pub fn is_341_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_341_US
    }
    #[doc = "Checks if the value of the field is `_170_US`"]
    #[inline(always)]
    pub fn is_170_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_170_US
    }
    #[doc = "Checks if the value of the field is `_85P3_US`"]
    #[inline(always)]
    pub fn is_85p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_85P3_US
    }
    #[doc = "Checks if the value of the field is `_42P6_US`"]
    #[inline(always)]
    pub fn is_42p6_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_42P6_US
    }
    #[doc = "Checks if the value of the field is `_21P3_US`"]
    #[inline(always)]
    pub fn is_21p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_21P3_US
    }
    #[doc = "Checks if the value of the field is `_10P6_US`"]
    #[inline(always)]
    pub fn is_10p6_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_10P6_US
    }
    #[doc = "Checks if the value of the field is `_5P3_US`"]
    #[inline(always)]
    pub fn is_5p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_5P3_US
    }
    #[doc = "Checks if the value of the field is `_2P7_US`"]
    #[inline(always)]
    pub fn is_2p7_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_2P7_US
    }
}
#[doc = "Field `SMPL_CYCLE_EXP` writer - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
pub type SMPL_CYCLE_EXP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ADC0_SPEC, u8, SMPL_CYCLE_EXP_A, 4, O>;
impl<'a, const O: u8> SMPL_CYCLE_EXP_W<'a, O> {
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    #[inline(always)]
    pub fn _10p9_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_10P9_MS)
    }
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    #[inline(always)]
    pub fn _5p46_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_5P46_MS)
    }
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    #[inline(always)]
    pub fn _2p73_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_2P73_MS)
    }
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    #[inline(always)]
    pub fn _1p37_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_1P37_MS)
    }
    #[doc = "4096x 6 MHz clock periods = 682us"]
    #[inline(always)]
    pub fn _682_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_682_US)
    }
    #[doc = "2048x 6 MHz clock periods = 341us"]
    #[inline(always)]
    pub fn _341_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_341_US)
    }
    #[doc = "1024x 6 MHz clock periods = 170us"]
    #[inline(always)]
    pub fn _170_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_170_US)
    }
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    #[inline(always)]
    pub fn _85p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_85P3_US)
    }
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    #[inline(always)]
    pub fn _42p6_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_42P6_US)
    }
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    #[inline(always)]
    pub fn _21p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_21P3_US)
    }
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    #[inline(always)]
    pub fn _10p6_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_10P6_US)
    }
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    #[inline(always)]
    pub fn _5p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_5P3_US)
    }
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    #[inline(always)]
    pub fn _2p7_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_2P7_US)
    }
}
#[doc = "Field `SMPL_MODE` reader - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
pub type SMPL_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SMPL_MODE` writer - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
pub type SMPL_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADC0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    pub fn smpl_cycle_exp(&self) -> SMPL_CYCLE_EXP_R {
        SMPL_CYCLE_EXP_R::new((self.bits >> 3) & 0x0f)
    }
    #[doc = "Bit 7 - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline(always)]
    pub fn smpl_mode(&self) -> SMPL_MODE_R {
        SMPL_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> RESET_N_W<1> {
        RESET_N_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    #[must_use]
    pub fn smpl_cycle_exp(&mut self) -> SMPL_CYCLE_EXP_W<3> {
        SMPL_CYCLE_EXP_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_mode(&mut self) -> SMPL_MODE_W<7> {
        SMPL_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0](index.html) module"]
pub struct ADC0_SPEC;
impl crate::RegisterSpec for ADC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc0::R](R) reader structure"]
impl crate::Readable for ADC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0::W](W) writer structure"]
impl crate::Writable for ADC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0 to value 0"]
impl crate::Resettable for ADC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
