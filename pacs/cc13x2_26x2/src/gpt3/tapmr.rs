#[doc = "Register `TAPMR` reader"]
pub type R = crate::R<TapmrSpec>;
#[doc = "Register `TAPMR` writer"]
pub type W = crate::W<TapmrSpec>;
#[doc = "Field `TAPSMR` reader - 7:0\\]
GPT Timer A Pre-scale Match. In 16 bit mode this field holds bits 23 to 16."]
pub type TapsmrR = crate::FieldReader;
#[doc = "Field `TAPSMR` writer - 7:0\\]
GPT Timer A Pre-scale Match. In 16 bit mode this field holds bits 23 to 16."]
pub type TapsmrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer A Pre-scale Match. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub fn tapsmr(&self) -> TapsmrR {
        TapsmrR::new((self.bits & 0xff) as u8)
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
GPT Timer A Pre-scale Match. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    #[must_use]
    pub fn tapsmr(&mut self) -> TapsmrW<TapmrSpec> {
        TapsmrW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<TapmrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TapmrSpec;
impl crate::RegisterSpec for TapmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapmr::R`](R) reader structure"]
impl crate::Readable for TapmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tapmr::W`](W) writer structure"]
impl crate::Writable for TapmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAPMR to value 0"]
impl crate::Resettable for TapmrSpec {
    const RESET_VALUE: u32 = 0;
}
