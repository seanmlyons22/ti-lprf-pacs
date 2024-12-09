#[doc = "Register `FOLDCNT` reader"]
pub type R = crate::R<FoldcntSpec>;
#[doc = "Register `FOLDCNT` writer"]
pub type W = crate::W<FoldcntSpec>;
#[doc = "Field `FOLDCNT` reader - 7:0\\]
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
pub type FoldcntR = crate::FieldReader;
#[doc = "Field `FOLDCNT` writer - 7:0\\]
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
pub type FoldcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
    #[inline(always)]
    pub fn foldcnt(&self) -> FoldcntR {
        FoldcntR::new((self.bits & 0xff) as u8)
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
Counts on each cycle when all of the following are true: - DWT_CTRL.FOLDEVTENA == 1 and DEMCR.TRCENA == 1. - At least two instructions are executed, see DWT_CPICNT. - Either SecureNoninvasiveDebugAllowed() == TRUE, or the PE is in Non-secure state and NoninvasiveDebugAllowed() == TRUE. The counter is incremented by the number of instructions executed, minus one"]
    #[inline(always)]
    #[must_use]
    pub fn foldcnt(&mut self) -> FoldcntW<FoldcntSpec> {
        FoldcntW::new(self, 0)
    }
}
#[doc = "Increments on the additional cycles required to execute all load or store instructions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`foldcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`foldcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FoldcntSpec;
impl crate::RegisterSpec for FoldcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`foldcnt::R`](R) reader structure"]
impl crate::Readable for FoldcntSpec {}
#[doc = "`write(|w| ..)` method takes [`foldcnt::W`](W) writer structure"]
impl crate::Writable for FoldcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FOLDCNT to value 0"]
impl crate::Resettable for FoldcntSpec {
    const RESET_VALUE: u32 = 0;
}
