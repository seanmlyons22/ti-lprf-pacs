#[doc = "Register `TIME524M` reader"]
pub type R = crate::R<Time524mSpec>;
#[doc = "Register `TIME524M` writer"]
pub type W = crate::W<Time524mSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Unsigned integer representing. \\[50:19\\]slice of real time counter."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Unsigned integer representing. \\[50:19\\]slice of real time counter."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing. \\[50:19\\]slice of real time counter."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing. \\[50:19\\]slice of real time counter."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Time524mSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "RTC time value register. 32-bit unsigned integer representing \\[50:19\\]
time slice of the real time clock counter. This field has a resolution of about 0.5s and a range of about 71.4 years.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time524m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time524m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Time524mSpec;
impl crate::RegisterSpec for Time524mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time524m::R`](R) reader structure"]
impl crate::Readable for Time524mSpec {}
#[doc = "`write(|w| ..)` method takes [`time524m::W`](W) writer structure"]
impl crate::Writable for Time524mSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME524M to value 0"]
impl crate::Resettable for Time524mSpec {
    const RESET_VALUE: u32 = 0;
}
