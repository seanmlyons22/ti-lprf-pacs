#[doc = "Register `FUNC_ERR_ADD` reader"]
pub type R = crate::R<FuncErrAddSpec>;
#[doc = "Register `FUNC_ERR_ADD` writer"]
pub type W = crate::W<FuncErrAddSpec>;
#[doc = "Field `FUNC_ERR_ADD` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FuncErrAddR = crate::FieldReader<u32>;
#[doc = "Field `FUNC_ERR_ADD` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FuncErrAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn func_err_add(&self) -> FuncErrAddR {
        FuncErrAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn func_err_add(&mut self) -> FuncErrAddW<FuncErrAddSpec> {
        FuncErrAddW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_err_add::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_err_add::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncErrAddSpec;
impl crate::RegisterSpec for FuncErrAddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_err_add::R`](R) reader structure"]
impl crate::Readable for FuncErrAddSpec {}
#[doc = "`write(|w| ..)` method takes [`func_err_add::W`](W) writer structure"]
impl crate::Writable for FuncErrAddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC_ERR_ADD to value 0"]
impl crate::Resettable for FuncErrAddSpec {
    const RESET_VALUE: u32 = 0;
}
