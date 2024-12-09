#[doc = "Register `TIME250N` reader"]
pub type R = crate::R<Time250nSpec>;
#[doc = "Register `TIME250N` writer"]
pub type W = crate::W<Time250nSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
32-bit counter value \\[31:0\\]. This will provide a 250ns resolution and a range of 17.9m."]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
32-bit counter value \\[31:0\\]. This will provide a 250ns resolution and a range of 17.9m."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {}
#[doc = "Systimer Counter Value. This 32-bit value reads out bits \\[31:0\\]
of the systimer counter. The counter is 34-bit and runs on CLKSVT/12. It maintains a resolution of 250ns with a range of about 17.9m.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time250n::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time250n::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Time250nSpec;
impl crate::RegisterSpec for Time250nSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time250n::R`](R) reader structure"]
impl crate::Readable for Time250nSpec {}
#[doc = "`write(|w| ..)` method takes [`time250n::W`](W) writer structure"]
impl crate::Writable for Time250nSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME250N to value 0"]
impl crate::Resettable for Time250nSpec {
    const RESET_VALUE: u32 = 0;
}
