#[doc = "Register `CLAIMSET` reader"]
pub type R = crate::R<ClaimsetSpec>;
#[doc = "Register `CLAIMSET` writer"]
pub type W = crate::W<ClaimsetSpec>;
#[doc = "Field `CLAIMSET` reader - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
pub type ClaimsetR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMSET` writer - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
pub type ClaimsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    pub fn claimset(&self) -> ClaimsetR {
        ClaimsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    #[must_use]
    pub fn claimset(&mut self) -> ClaimsetW<ClaimsetSpec> {
        ClaimsetW::new(self, 0)
    }
}
#[doc = "Claim Tag Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimsetSpec;
impl crate::RegisterSpec for ClaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimset::R`](R) reader structure"]
impl crate::Readable for ClaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`claimset::W`](W) writer structure"]
impl crate::Writable for ClaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLAIMSET to value 0x0f"]
impl crate::Resettable for ClaimsetSpec {
    const RESET_VALUE: u32 = 0x0f;
}
