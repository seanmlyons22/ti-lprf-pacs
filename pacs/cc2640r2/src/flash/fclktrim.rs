#[doc = "Register `FCLKTRIM` reader"]
pub type R = crate::R<FclktrimSpec>;
#[doc = "Register `FCLKTRIM` writer"]
pub type W = crate::W<FclktrimSpec>;
#[doc = "Field `TRIM_EN` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimEnR = crate::FieldReader<u32>;
#[doc = "Field `TRIM_EN` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type TrimEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_en(&self) -> TrimEnR {
        TrimEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_en(&mut self) -> TrimEnW<FclktrimSpec> {
        TrimEnW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fclktrim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fclktrim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FclktrimSpec;
impl crate::RegisterSpec for FclktrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fclktrim::R`](R) reader structure"]
impl crate::Readable for FclktrimSpec {}
#[doc = "`write(|w| ..)` method takes [`fclktrim::W`](W) writer structure"]
impl crate::Writable for FclktrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCLKTRIM to value 0"]
impl crate::Resettable for FclktrimSpec {
    const RESET_VALUE: u32 = 0;
}
