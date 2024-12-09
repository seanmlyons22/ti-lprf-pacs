#[doc = "Register `RCOSC_HF_TEMPCOMP` reader"]
pub type R = crate::R<RcoscHfTempcompSpec>;
#[doc = "Register `RCOSC_HF_TEMPCOMP` writer"]
pub type W = crate::W<RcoscHfTempcompSpec>;
#[doc = "Field `CTRIMFRACT_SLOPE` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CtrimfractSlopeR = crate::FieldReader;
#[doc = "Field `CTRIMFRACT_QUAD` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type CtrimfractQuadR = crate::FieldReader;
#[doc = "Field `CTRIM` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type CtrimR = crate::FieldReader;
#[doc = "Field `FINE_RESISTOR` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FineResistorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_slope(&self) -> CtrimfractSlopeR {
        CtrimfractSlopeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_quad(&self) -> CtrimfractQuadR {
        CtrimfractQuadR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrim(&self) -> CtrimR {
        CtrimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fine_resistor(&self) -> FineResistorR {
        FineResistorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcosc_hf_tempcomp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcosc_hf_tempcomp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcoscHfTempcompSpec;
impl crate::RegisterSpec for RcoscHfTempcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcosc_hf_tempcomp::R`](R) reader structure"]
impl crate::Readable for RcoscHfTempcompSpec {}
#[doc = "`write(|w| ..)` method takes [`rcosc_hf_tempcomp::W`](W) writer structure"]
impl crate::Writable for RcoscHfTempcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCOSC_HF_TEMPCOMP to value 0x03"]
impl crate::Resettable for RcoscHfTempcompSpec {
    const RESET_VALUE: u32 = 0x03;
}
