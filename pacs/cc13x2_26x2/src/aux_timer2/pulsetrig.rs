#[doc = "Register `PULSETRIG` reader"]
pub type R = crate::R<PulsetrigSpec>;
#[doc = "Register `PULSETRIG` writer"]
pub type W = crate::W<PulsetrigSpec>;
#[doc = "Field `TRIG` reader - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
pub type TrigR = crate::BitReader;
#[doc = "Field `TRIG` writer - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Pulse trigger. Write 1 to generate a pulse to AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE. Pulse width equals the duty cycle of AUX_SYSIF:TIMER2CLKCTL.SRC."]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TrigW<PulsetrigSpec> {
        TrigW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<PulsetrigSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Pulse Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulsetrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulsetrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PulsetrigSpec;
impl crate::RegisterSpec for PulsetrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulsetrig::R`](R) reader structure"]
impl crate::Readable for PulsetrigSpec {}
#[doc = "`write(|w| ..)` method takes [`pulsetrig::W`](W) writer structure"]
impl crate::Writable for PulsetrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PULSETRIG to value 0"]
impl crate::Resettable for PulsetrigSpec {
    const RESET_VALUE: u32 = 0;
}
