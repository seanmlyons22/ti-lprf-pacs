#[doc = "Register `SWPWRPROF` reader"]
pub type R = crate::R<SwpwrprofSpec>;
#[doc = "Register `SWPWRPROF` writer"]
pub type W = crate::W<SwpwrprofSpec>;
#[doc = "Field `STAT` reader - 2:0\\]
Software status bits that can be read by the power profiler."]
pub type StatR = crate::FieldReader;
#[doc = "Field `STAT` writer - 2:0\\]
Software status bits that can be read by the power profiler."]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Software status bits that can be read by the power profiler."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Software status bits that can be read by the power profiler."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<SwpwrprofSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SwpwrprofSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Software Power Profiler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swpwrprof::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpwrprof::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwpwrprofSpec;
impl crate::RegisterSpec for SwpwrprofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swpwrprof::R`](R) reader structure"]
impl crate::Readable for SwpwrprofSpec {}
#[doc = "`write(|w| ..)` method takes [`swpwrprof::W`](W) writer structure"]
impl crate::Writable for SwpwrprofSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPWRPROF to value 0"]
impl crate::Resettable for SwpwrprofSpec {
    const RESET_VALUE: u32 = 0;
}
