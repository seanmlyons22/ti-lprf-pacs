#[doc = "Register `AIFPWMVALUE` reader"]
pub type R = crate::R<AifpwmvalueSpec>;
#[doc = "Register `AIFPWMVALUE` writer"]
pub type W = crate::W<AifpwmvalueSpec>;
#[doc = "Field `PULSE_WIDTH` reader - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
pub type PulseWidthR = crate::FieldReader<u16>;
#[doc = "Field `PULSE_WIDTH` writer - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
pub type PulseWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
    #[inline(always)]
    pub fn pulse_width(&self) -> PulseWidthR {
        PulseWidthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_width(&mut self) -> PulseWidthW<AifpwmvalueSpec> {
        PulseWidthW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<AifpwmvalueSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Audio Interface PWM Debug Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifpwmvalue::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifpwmvalue::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifpwmvalueSpec;
impl crate::RegisterSpec for AifpwmvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifpwmvalue::R`](R) reader structure"]
impl crate::Readable for AifpwmvalueSpec {}
#[doc = "`write(|w| ..)` method takes [`aifpwmvalue::W`](W) writer structure"]
impl crate::Writable for AifpwmvalueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFPWMVALUE to value 0"]
impl crate::Resettable for AifpwmvalueSpec {
    const RESET_VALUE: u32 = 0;
}
