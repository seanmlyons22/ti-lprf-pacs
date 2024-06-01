#[doc = "Register `ADC0` reader"]
pub type R = crate::R<Adc0Spec>;
#[doc = "Register `ADC0` writer"]
pub type W = crate::W<Adc0Spec>;
#[doc = "Field `EN` reader - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_N` reader - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
pub type ResetNR = crate::BitReader;
#[doc = "Field `RESET_N` writer - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
pub type ResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SmplCycleExp {
    #[doc = "15: 65536x 6 MHz clock periods = 10.9ms"]
    _10p9Ms = 15,
    #[doc = "14: 32768x 6 MHz clock periods = 5.46ms"]
    _5p46Ms = 14,
    #[doc = "13: 16384x 6 MHz clock periods = 2.73ms"]
    _2p73Ms = 13,
    #[doc = "12: 8192x 6 MHz clock periods = 1.37ms"]
    _1p37Ms = 12,
    #[doc = "11: 4096x 6 MHz clock periods = 682us"]
    _682Us = 11,
    #[doc = "10: 2048x 6 MHz clock periods = 341us"]
    _341Us = 10,
    #[doc = "9: 1024x 6 MHz clock periods = 170us"]
    _170Us = 9,
    #[doc = "8: 512x 6 MHz clock periods = 85.3us"]
    _85p3Us = 8,
    #[doc = "7: 256x 6 MHz clock periods = 42.6us"]
    _42p6Us = 7,
    #[doc = "6: 128x 6 MHz clock periods = 21.3us"]
    _21p3Us = 6,
    #[doc = "5: 64x 6 MHz clock periods = 10.6us"]
    _10p6Us = 5,
    #[doc = "4: 32x 6 MHz clock periods = 5.3us"]
    _5p3Us = 4,
    #[doc = "3: 16x 6 MHz clock periods = 2.7us"]
    _2p7Us = 3,
}
impl From<SmplCycleExp> for u8 {
    #[inline(always)]
    fn from(variant: SmplCycleExp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SmplCycleExp {
    type Ux = u8;
}
impl crate::IsEnum for SmplCycleExp {}
#[doc = "Field `SMPL_CYCLE_EXP` reader - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
pub type SmplCycleExpR = crate::FieldReader<SmplCycleExp>;
impl SmplCycleExpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SmplCycleExp> {
        match self.bits {
            15 => Some(SmplCycleExp::_10p9Ms),
            14 => Some(SmplCycleExp::_5p46Ms),
            13 => Some(SmplCycleExp::_2p73Ms),
            12 => Some(SmplCycleExp::_1p37Ms),
            11 => Some(SmplCycleExp::_682Us),
            10 => Some(SmplCycleExp::_341Us),
            9 => Some(SmplCycleExp::_170Us),
            8 => Some(SmplCycleExp::_85p3Us),
            7 => Some(SmplCycleExp::_42p6Us),
            6 => Some(SmplCycleExp::_21p3Us),
            5 => Some(SmplCycleExp::_10p6Us),
            4 => Some(SmplCycleExp::_5p3Us),
            3 => Some(SmplCycleExp::_2p7Us),
            _ => None,
        }
    }
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    #[inline(always)]
    pub fn is_10p9_ms(&self) -> bool {
        *self == SmplCycleExp::_10p9Ms
    }
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    #[inline(always)]
    pub fn is_5p46_ms(&self) -> bool {
        *self == SmplCycleExp::_5p46Ms
    }
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    #[inline(always)]
    pub fn is_2p73_ms(&self) -> bool {
        *self == SmplCycleExp::_2p73Ms
    }
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    #[inline(always)]
    pub fn is_1p37_ms(&self) -> bool {
        *self == SmplCycleExp::_1p37Ms
    }
    #[doc = "4096x 6 MHz clock periods = 682us"]
    #[inline(always)]
    pub fn is_682_us(&self) -> bool {
        *self == SmplCycleExp::_682Us
    }
    #[doc = "2048x 6 MHz clock periods = 341us"]
    #[inline(always)]
    pub fn is_341_us(&self) -> bool {
        *self == SmplCycleExp::_341Us
    }
    #[doc = "1024x 6 MHz clock periods = 170us"]
    #[inline(always)]
    pub fn is_170_us(&self) -> bool {
        *self == SmplCycleExp::_170Us
    }
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    #[inline(always)]
    pub fn is_85p3_us(&self) -> bool {
        *self == SmplCycleExp::_85p3Us
    }
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    #[inline(always)]
    pub fn is_42p6_us(&self) -> bool {
        *self == SmplCycleExp::_42p6Us
    }
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    #[inline(always)]
    pub fn is_21p3_us(&self) -> bool {
        *self == SmplCycleExp::_21p3Us
    }
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    #[inline(always)]
    pub fn is_10p6_us(&self) -> bool {
        *self == SmplCycleExp::_10p6Us
    }
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    #[inline(always)]
    pub fn is_5p3_us(&self) -> bool {
        *self == SmplCycleExp::_5p3Us
    }
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    #[inline(always)]
    pub fn is_2p7_us(&self) -> bool {
        *self == SmplCycleExp::_2p7Us
    }
}
#[doc = "Field `SMPL_CYCLE_EXP` writer - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
pub type SmplCycleExpW<'a, REG> = crate::FieldWriter<'a, REG, 4, SmplCycleExp>;
impl<'a, REG> SmplCycleExpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    #[inline(always)]
    pub fn _10p9_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_10p9Ms)
    }
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    #[inline(always)]
    pub fn _5p46_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_5p46Ms)
    }
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    #[inline(always)]
    pub fn _2p73_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_2p73Ms)
    }
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    #[inline(always)]
    pub fn _1p37_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_1p37Ms)
    }
    #[doc = "4096x 6 MHz clock periods = 682us"]
    #[inline(always)]
    pub fn _682_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_682Us)
    }
    #[doc = "2048x 6 MHz clock periods = 341us"]
    #[inline(always)]
    pub fn _341_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_341Us)
    }
    #[doc = "1024x 6 MHz clock periods = 170us"]
    #[inline(always)]
    pub fn _170_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_170Us)
    }
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    #[inline(always)]
    pub fn _85p3_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_85p3Us)
    }
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    #[inline(always)]
    pub fn _42p6_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_42p6Us)
    }
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    #[inline(always)]
    pub fn _21p3_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_21p3Us)
    }
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    #[inline(always)]
    pub fn _10p6_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_10p6Us)
    }
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    #[inline(always)]
    pub fn _5p3_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_5p3Us)
    }
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    #[inline(always)]
    pub fn _2p7_us(self) -> &'a mut crate::W<REG> {
        self.variant(SmplCycleExp::_2p7Us)
    }
}
#[doc = "Field `SMPL_MODE` reader - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
pub type SmplModeR = crate::BitReader;
#[doc = "Field `SMPL_MODE` writer - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
pub type SmplModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline(always)]
    pub fn reset_n(&self) -> ResetNR {
        ResetNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    pub fn smpl_cycle_exp(&self) -> SmplCycleExpR {
        SmplCycleExpR::new((self.bits >> 3) & 0x0f)
    }
    #[doc = "Bit 7 - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline(always)]
    pub fn smpl_mode(&self) -> SmplModeR {
        SmplModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Adc0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> ResetNW<Adc0Spec> {
        ResetNW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Adc0Spec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    #[must_use]
    pub fn smpl_cycle_exp(&mut self) -> SmplCycleExpW<Adc0Spec> {
        SmplCycleExpW::new(self, 3)
    }
    #[doc = "Bit 7 - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_mode(&mut self) -> SmplModeW<Adc0Spec> {
        SmplModeW::new(self, 7)
    }
}
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Spec;
impl crate::RegisterSpec for Adc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc0::R`](R) reader structure"]
impl crate::Readable for Adc0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc0::W`](W) writer structure"]
impl crate::Writable for Adc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC0 to value 0"]
impl crate::Resettable for Adc0Spec {
    const RESET_VALUE: u8 = 0;
}
