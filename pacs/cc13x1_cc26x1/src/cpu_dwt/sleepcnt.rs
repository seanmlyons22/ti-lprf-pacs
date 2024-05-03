#[doc = "Register `SLEEPCNT` reader"]
pub type R = crate::R<SleepcntSpec>;
#[doc = "Register `SLEEPCNT` writer"]
pub type W = crate::W<SleepcntSpec>;
#[doc = "Field `SLEEPCNT` reader - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
pub type SleepcntR = crate::FieldReader;
#[doc = "Field `SLEEPCNT` writer - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
pub type SleepcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    pub fn sleepcnt(&self) -> SleepcntR {
        SleepcntR::new((self.bits & 0xff) as u8)
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
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    #[must_use]
    pub fn sleepcnt(&mut self) -> SleepcntW<SleepcntSpec> {
        SleepcntW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SleepcntSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepcntSpec;
impl crate::RegisterSpec for SleepcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepcnt::R`](R) reader structure"]
impl crate::Readable for SleepcntSpec {}
#[doc = "`write(|w| ..)` method takes [`sleepcnt::W`](W) writer structure"]
impl crate::Writable for SleepcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEPCNT to value 0"]
impl crate::Resettable for SleepcntSpec {
    const RESET_VALUE: u32 = 0;
}
