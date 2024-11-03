#[doc = "Register `MUNLOCK` reader"]
pub type R = crate::R<MunlockSpec>;
#[doc = "Register `MUNLOCK` writer"]
pub type W = crate::W<MunlockSpec>;
#[doc = "Field `KEY` reader - 31:0\\]
Write the unlock key 0xC5AF_6927 to temporarily unlock registers in mutable section. The lock is set automatically if no write accesses, to the mutable section, are detected for consecutive 32 CLKULL (24MHz) clock cycles. Writing any value other that the unlock key will immediately lock the mutable register space for write access."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - 31:0\\]
Write the unlock key 0xC5AF_6927 to temporarily unlock registers in mutable section. The lock is set automatically if no write accesses, to the mutable section, are detected for consecutive 32 CLKULL (24MHz) clock cycles. Writing any value other that the unlock key will immediately lock the mutable register space for write access."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Write the unlock key 0xC5AF_6927 to temporarily unlock registers in mutable section. The lock is set automatically if no write accesses, to the mutable section, are detected for consecutive 32 CLKULL (24MHz) clock cycles. Writing any value other that the unlock key will immediately lock the mutable register space for write access."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Write the unlock key 0xC5AF_6927 to temporarily unlock registers in mutable section. The lock is set automatically if no write accesses, to the mutable section, are detected for consecutive 32 CLKULL (24MHz) clock cycles. Writing any value other that the unlock key will immediately lock the mutable register space for write access."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<MunlockSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Unlocks registers in mutable section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`munlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`munlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MunlockSpec;
impl crate::RegisterSpec for MunlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`munlock::R`](R) reader structure"]
impl crate::Readable for MunlockSpec {}
#[doc = "`write(|w| ..)` method takes [`munlock::W`](W) writer structure"]
impl crate::Writable for MunlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUNLOCK to value 0"]
impl crate::Resettable for MunlockSpec {
    const RESET_VALUE: u32 = 0;
}
