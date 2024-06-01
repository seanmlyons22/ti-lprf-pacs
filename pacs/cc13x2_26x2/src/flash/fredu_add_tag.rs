#[doc = "Register `FREDU_ADD_TAG` reader"]
pub type R = crate::R<FreduAddTagSpec>;
#[doc = "Register `FREDU_ADD_TAG` writer"]
pub type W = crate::W<FreduAddTagSpec>;
#[doc = "Field `REDU_ADD_TAG` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ReduAddTagR = crate::FieldReader<u32>;
#[doc = "Field `REDU_ADD_TAG` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ReduAddTagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redu_add_tag(&self) -> ReduAddTagR {
        ReduAddTagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redu_add_tag(&mut self) -> ReduAddTagW<FreduAddTagSpec> {
        ReduAddTagW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fredu_add_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fredu_add_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreduAddTagSpec;
impl crate::RegisterSpec for FreduAddTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fredu_add_tag::R`](R) reader structure"]
impl crate::Readable for FreduAddTagSpec {}
#[doc = "`write(|w| ..)` method takes [`fredu_add_tag::W`](W) writer structure"]
impl crate::Writable for FreduAddTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREDU_ADD_TAG to value 0"]
impl crate::Resettable for FreduAddTagSpec {
    const RESET_VALUE: u32 = 0;
}
