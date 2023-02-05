#[doc = "Register `AIFPWMVALUE` reader"]
pub struct R(crate::R<AIFPWMVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFPWMVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFPWMVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFPWMVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFPWMVALUE` writer"]
pub struct W(crate::W<AIFPWMVALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFPWMVALUE_SPEC>;
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
impl From<crate::W<AIFPWMVALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFPWMVALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULSE_WIDTH` reader - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
pub type PULSE_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PULSE_WIDTH` writer - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
pub type PULSE_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AIFPWMVALUE_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AIFPWMVALUE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
    #[inline(always)]
    pub fn pulse_width(&self) -> PULSE_WIDTH_R {
        PULSE_WIDTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_width(&mut self) -> PULSE_WIDTH_W<0> {
        PULSE_WIDTH_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio Interface PWM Debug Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifpwmvalue](index.html) module"]
pub struct AIFPWMVALUE_SPEC;
impl crate::RegisterSpec for AIFPWMVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aifpwmvalue::R](R) reader structure"]
impl crate::Readable for AIFPWMVALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aifpwmvalue::W](W) writer structure"]
impl crate::Writable for AIFPWMVALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIFPWMVALUE to value 0"]
impl crate::Resettable for AIFPWMVALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
