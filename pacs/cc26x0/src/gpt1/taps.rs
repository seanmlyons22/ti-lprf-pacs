#[doc = "Register `TAPS` reader"]
pub type R = crate::R<TapsSpec>;
#[doc = "Register `TAPS` writer"]
pub type W = crate::W<TapsSpec>;
#[doc = "Field `PSS` reader - 7:0\\]
GPT Timer A Pre-scaler"]
pub type PssR = crate::FieldReader;
#[doc = "Field `PSS` writer - 7:0\\]
GPT Timer A Pre-scaler"]
pub type PssW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer A Pre-scaler"]
    #[inline(always)]
    pub fn pss(&self) -> PssR {
        PssR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer A Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn pss(&mut self) -> PssW<TapsSpec> {
        PssW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<TapsSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TapsSpec;
impl crate::RegisterSpec for TapsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`taps::R`](R) reader structure"]
impl crate::Readable for TapsSpec {}
#[doc = "`write(|w| ..)` method takes [`taps::W`](W) writer structure"]
impl crate::Writable for TapsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAPS to value 0"]
impl crate::Resettable for TapsSpec {
    const RESET_VALUE: u32 = 0;
}
