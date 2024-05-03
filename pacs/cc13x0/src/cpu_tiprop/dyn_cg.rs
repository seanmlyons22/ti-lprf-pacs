#[doc = "Register `DYN_CG` reader"]
pub type R = crate::R<DynCgSpec>;
#[doc = "Register `DYN_CG` writer"]
pub type W = crate::W<DynCgSpec>;
#[doc = "Field `DYN_CG` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type DynCgR = crate::FieldReader;
#[doc = "Field `DYN_CG` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type DynCgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dyn_cg(&self) -> DynCgR {
        DynCgR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_cg(&mut self) -> DynCgW<DynCgSpec> {
        DynCgW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dyn_cg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dyn_cg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynCgSpec;
impl crate::RegisterSpec for DynCgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dyn_cg::R`](R) reader structure"]
impl crate::Readable for DynCgSpec {}
#[doc = "`write(|w| ..)` method takes [`dyn_cg::W`](W) writer structure"]
impl crate::Writable for DynCgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DYN_CG to value 0"]
impl crate::Resettable for DynCgSpec {
    const RESET_VALUE: u32 = 0;
}
