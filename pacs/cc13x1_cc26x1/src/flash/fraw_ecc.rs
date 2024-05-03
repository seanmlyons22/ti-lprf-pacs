#[doc = "Register `FRAW_ECC` reader"]
pub type R = crate::R<FrawEccSpec>;
#[doc = "Register `FRAW_ECC` writer"]
pub type W = crate::W<FrawEccSpec>;
#[doc = "Field `RAW_ECC` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type RawEccR = crate::FieldReader<u32>;
#[doc = "Field `RAW_ECC` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type RawEccW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn raw_ecc(&self) -> RawEccR {
        RawEccR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn raw_ecc(&mut self) -> RawEccW<FrawEccSpec> {
        RawEccW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fraw_ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fraw_ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrawEccSpec;
impl crate::RegisterSpec for FrawEccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fraw_ecc::R`](R) reader structure"]
impl crate::Readable for FrawEccSpec {}
#[doc = "`write(|w| ..)` method takes [`fraw_ecc::W`](W) writer structure"]
impl crate::Writable for FrawEccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAW_ECC to value 0"]
impl crate::Resettable for FrawEccSpec {
    const RESET_VALUE: u32 = 0;
}
