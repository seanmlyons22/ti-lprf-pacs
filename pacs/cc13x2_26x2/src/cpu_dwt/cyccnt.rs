#[doc = "Register `CYCCNT` reader"]
pub type R = crate::R<CyccntSpec>;
#[doc = "Register `CYCCNT` writer"]
pub type W = crate::W<CyccntSpec>;
#[doc = "Field `CYCCNT` reader - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
pub type CyccntR = crate::FieldReader<u32>;
#[doc = "Field `CYCCNT` writer - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
pub type CyccntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub fn cyccnt(&self) -> CyccntR {
        CyccntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    #[must_use]
    pub fn cyccnt(&mut self) -> CyccntW<CyccntSpec> {
        CyccntW::new(self, 0)
    }
}
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cyccnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cyccnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
