#[doc = "Register `FPRIM_ADD_TAG` reader"]
pub type R = crate::R<FprimAddTagSpec>;
#[doc = "Register `FPRIM_ADD_TAG` writer"]
pub type W = crate::W<FprimAddTagSpec>;
#[doc = "Field `PRIM_ADD_TAG` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type PrimAddTagR = crate::FieldReader<u32>;
#[doc = "Field `PRIM_ADD_TAG` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type PrimAddTagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn prim_add_tag(&self) -> PrimAddTagR {
        PrimAddTagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn prim_add_tag(&mut self) -> PrimAddTagW<FprimAddTagSpec> {
        PrimAddTagW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fprim_add_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fprim_add_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FprimAddTagSpec;
impl crate::RegisterSpec for FprimAddTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fprim_add_tag::R`](R) reader structure"]
impl crate::Readable for FprimAddTagSpec {}
#[doc = "`write(|w| ..)` method takes [`fprim_add_tag::W`](W) writer structure"]
impl crate::Writable for FprimAddTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPRIM_ADD_TAG to value 0"]
impl crate::Resettable for FprimAddTagSpec {
    const RESET_VALUE: u32 = 0;
}
