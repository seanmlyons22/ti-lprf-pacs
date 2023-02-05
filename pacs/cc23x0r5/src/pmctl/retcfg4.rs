#[doc = "Register `RETCFG4` reader"]
pub type R = crate::R<Retcfg4Spec>;
#[doc = "Register `RETCFG4` writer"]
pub type W = crate::W<Retcfg4Spec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Retcfg4Spec;
impl crate::RegisterSpec for Retcfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retcfg4::R`](R) reader structure"]
impl crate::Readable for Retcfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`retcfg4::W`](W) writer structure"]
impl crate::Writable for Retcfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETCFG4 to value 0"]
impl crate::Resettable for Retcfg4Spec {
    const RESET_VALUE: u32 = 0;
}
