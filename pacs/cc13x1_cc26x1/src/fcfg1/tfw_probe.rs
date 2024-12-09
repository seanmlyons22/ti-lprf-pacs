#[doc = "Register `TFW_PROBE` reader"]
pub type R = crate::R<TfwProbeSpec>;
#[doc = "Register `TFW_PROBE` writer"]
pub type W = crate::W<TfwProbeSpec>;
#[doc = "Field `REV` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type RevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfw_probe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfw_probe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfwProbeSpec;
impl crate::RegisterSpec for TfwProbeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfw_probe::R`](R) reader structure"]
impl crate::Readable for TfwProbeSpec {}
#[doc = "`write(|w| ..)` method takes [`tfw_probe::W`](W) writer structure"]
impl crate::Writable for TfwProbeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFW_PROBE to value 0"]
impl crate::Resettable for TfwProbeSpec {
    const RESET_VALUE: u32 = 0;
}
