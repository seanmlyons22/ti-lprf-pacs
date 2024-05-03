#[doc = "Register `CYCCNT` reader"]
pub type R = crate::R<CyccntSpec>;
#[doc = "Register `CYCCNT` writer"]
pub type W = crate::W<CyccntSpec>;
#[doc = "Field `CYCCNT` reader - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
pub type CyccntR = crate::FieldReader<u32>;
#[doc = "Field `CYCCNT` writer - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
pub type CyccntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
    #[inline(always)]
    pub fn cyccnt(&self) -> CyccntR {
        CyccntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Increments one on each processor clock cycle when DWT_CTRL.CYCCNTENA == 1 and DEMCR.TRCENA == 1. On overflow, CYCCNT wraps to zero"]
    #[inline(always)]
    #[must_use]
    pub fn cyccnt(&mut self) -> CyccntW<CyccntSpec> {
        CyccntW::new(self, 0)
    }
}
#[doc = "Shows or sets the value of the processor cycle counter, CYCCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cyccnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cyccnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CyccntSpec;
impl crate::RegisterSpec for CyccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cyccnt::R`](R) reader structure"]
impl crate::Readable for CyccntSpec {}
#[doc = "`write(|w| ..)` method takes [`cyccnt::W`](W) writer structure"]
impl crate::Writable for CyccntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CYCCNT to value 0"]
impl crate::Resettable for CyccntSpec {
    const RESET_VALUE: u32 = 0;
}
