#[doc = "Register `CLAIMCLR` reader"]
pub type R = crate::R<ClaimclrSpec>;
#[doc = "Register `CLAIMCLR` writer"]
pub type W = crate::W<ClaimclrSpec>;
#[doc = "Field `CLAIMCLR` writer - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
pub type ClaimclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[inline(always)]
    #[must_use]
    pub fn claimclr(&mut self) -> ClaimclrW<ClaimclrSpec> {
        ClaimclrW::new(self, 0)
    }
}
#[doc = "Claim Tag Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimclrSpec;
impl crate::RegisterSpec for ClaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimclr::R`](R) reader structure"]
impl crate::Readable for ClaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`claimclr::W`](W) writer structure"]
impl crate::Writable for ClaimclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLAIMCLR to value 0"]
impl crate::Resettable for ClaimclrSpec {
    const RESET_VALUE: u32 = 0;
}
