#[doc = "Register `PREEVENT` reader"]
pub type R = crate::R<PreeventSpec>;
#[doc = "Register `PREEVENT` writer"]
pub type W = crate::W<PreeventSpec>;
#[doc = "Field `VAL` reader - 7:0\\]
Sets the HIGH time of the prescaler event output. Event goes high when the prescaler counter equals VAL. Event goes low when prescaler counter is 0. Note: - Can be used to precharge or turn an external component on for a short time before sampling, like in QDEC. - If there is a requirement to create such events that have very short periods compared to timer clock period, use two timers. One timer acts as prescaler and event generator for another timer."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - 7:0\\]
Sets the HIGH time of the prescaler event output. Event goes high when the prescaler counter equals VAL. Event goes low when prescaler counter is 0. Note: - Can be used to precharge or turn an external component on for a short time before sampling, like in QDEC. - If there is a requirement to create such events that have very short periods compared to timer clock period, use two timers. One timer acts as prescaler and event generator for another timer."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Sets the HIGH time of the prescaler event output. Event goes high when the prescaler counter equals VAL. Event goes low when prescaler counter is 0. Note: - Can be used to precharge or turn an external component on for a short time before sampling, like in QDEC. - If there is a requirement to create such events that have very short periods compared to timer clock period, use two timers. One timer acts as prescaler and event generator for another timer."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
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
Sets the HIGH time of the prescaler event output. Event goes high when the prescaler counter equals VAL. Event goes low when prescaler counter is 0. Note: - Can be used to precharge or turn an external component on for a short time before sampling, like in QDEC. - If there is a requirement to create such events that have very short periods compared to timer clock period, use two timers. One timer acts as prescaler and event generator for another timer."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PreeventSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<PreeventSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Prescaler Event This register is used to output a logic high signal before the zero crossing of the prescaler counter. The output is routed to the IOC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preevent::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preevent::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PreeventSpec;
impl crate::RegisterSpec for PreeventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preevent::R`](R) reader structure"]
impl crate::Readable for PreeventSpec {}
#[doc = "`write(|w| ..)` method takes [`preevent::W`](W) writer structure"]
impl crate::Writable for PreeventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREEVENT to value 0"]
impl crate::Resettable for PreeventSpec {
    const RESET_VALUE: u32 = 0;
}
