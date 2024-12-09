#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Register `VERSION` writer"]
pub type W = crate::W<VersionSpec>;
#[doc = "Field `VERSION` reader - 31:0\\]
Timer Revision."]
pub type VersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Timer Revision."]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(self.bits)
    }
}
impl W {}
#[doc = "Peripheral Version This register provides information regarding the GPT version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VersionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VERSION to value 0x0400"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0x0400;
}
