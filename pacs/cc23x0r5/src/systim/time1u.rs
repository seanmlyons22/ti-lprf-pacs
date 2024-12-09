#[doc = "Register `TIME1U` reader"]
pub type R = crate::R<Time1uSpec>;
#[doc = "Register `TIME1U` writer"]
pub type W = crate::W<Time1uSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
32-bit counter value \\[33:2\\]. This will provide a resolution of 1us and a range of 1hr and 11m."]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
32-bit counter value \\[33:2\\]. This will provide a resolution of 1us and a range of 1hr and 11m."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {}
#[doc = "Systimer Counter Value. This 32-bit value reads out bits\\[33:2\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 1us with a range of about 1 h 11m.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time1u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time1u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Time1uSpec;
impl crate::RegisterSpec for Time1uSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time1u::R`](R) reader structure"]
impl crate::Readable for Time1uSpec {}
#[doc = "`write(|w| ..)` method takes [`time1u::W`](W) writer structure"]
impl crate::Writable for Time1uSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME1U to value 0"]
impl crate::Resettable for Time1uSpec {
    const RESET_VALUE: u32 = 0;
}
