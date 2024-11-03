#[doc = "Register `TIME8U` reader"]
pub type R = crate::R<Time8uSpec>;
#[doc = "Register `TIME8U` writer"]
pub type W = crate::W<Time8uSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Unsigned integer representing \\[34:3\\]slice of real time counter."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Unsigned integer representing \\[34:3\\]slice of real time counter."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing \\[34:3\\]slice of real time counter."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing \\[34:3\\]slice of real time counter."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Time8uSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "RTC Time value register. 32-bit unsigned integer representing \\[34:3\\]
time slice of the real time clock counter. The counter runs on LFCLK. This field has a resolution of 8us, and range of about 9.5 hours.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time8u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time8u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Time8uSpec;
impl crate::RegisterSpec for Time8uSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time8u::R`](R) reader structure"]
impl crate::Readable for Time8uSpec {}
#[doc = "`write(|w| ..)` method takes [`time8u::W`](W) writer structure"]
impl crate::Writable for Time8uSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME8U to value 0"]
impl crate::Resettable for Time8uSpec {
    const RESET_VALUE: u32 = 0;
}
