#[doc = "Register `CLAIMTAG` reader"]
pub type R = crate::R<ClaimtagSpec>;
#[doc = "Register `CLAIMTAG` writer"]
pub type W = crate::W<ClaimtagSpec>;
#[doc = "Field `CLAIMTAG` reader - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
pub type ClaimtagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[inline(always)]
    pub fn claimtag(&self) -> ClaimtagR {
        ClaimtagR::new(self.bits)
    }
}
impl W {}
#[doc = "Current Claim Tag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimtag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimtag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimtagSpec;
impl crate::RegisterSpec for ClaimtagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimtag::R`](R) reader structure"]
impl crate::Readable for ClaimtagSpec {}
#[doc = "`write(|w| ..)` method takes [`claimtag::W`](W) writer structure"]
impl crate::Writable for ClaimtagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLAIMTAG to value 0"]
impl crate::Resettable for ClaimtagSpec {
    const RESET_VALUE: u32 = 0;
}
