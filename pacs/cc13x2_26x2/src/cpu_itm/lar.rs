#[doc = "Register `LAR` reader"]
pub type R = crate::R<LarSpec>;
#[doc = "Register `LAR` writer"]
pub type W = crate::W<LarSpec>;
#[doc = "Field `LOCK_ACCESS` writer - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
pub type LockAccessW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[inline(always)]
    #[must_use]
    pub fn lock_access(&mut self) -> LockAccessW<LarSpec> {
        LockAccessW::new(self, 0)
    }
}
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LarSpec;
impl crate::RegisterSpec for LarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lar::R`](R) reader structure"]
impl crate::Readable for LarSpec {}
#[doc = "`write(|w| ..)` method takes [`lar::W`](W) writer structure"]
impl crate::Writable for LarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LarSpec {
    const RESET_VALUE: u32 = 0;
}
