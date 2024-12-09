#[doc = "Register `CAP_TRIM` reader"]
pub type R = crate::R<CapTrimSpec>;
#[doc = "Register `CAP_TRIM` writer"]
pub type W = crate::W<CapTrimSpec>;
#[doc = "Field `FLUX_CAP_0P4_TRIM` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type FluxCap0p4TrimR = crate::FieldReader<u16>;
#[doc = "Field `FLUX_CAP_0P28_TRIM` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type FluxCap0p28TrimR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flux_cap_0p4_trim(&self) -> FluxCap0p4TrimR {
        FluxCap0p4TrimR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flux_cap_0p28_trim(&self) -> FluxCap0p28TrimR {
        FluxCap0p28TrimR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapTrimSpec;
impl crate::RegisterSpec for CapTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_trim::R`](R) reader structure"]
impl crate::Readable for CapTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`cap_trim::W`](W) writer structure"]
impl crate::Writable for CapTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_TRIM to value 0xffff_ffff"]
impl crate::Resettable for CapTrimSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
