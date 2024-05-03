#[doc = "Register `FWPWRITE6` reader"]
pub type R = crate::R<Fwpwrite6Spec>;
#[doc = "Register `FWPWRITE6` writer"]
pub type W = crate::W<Fwpwrite6Spec>;
#[doc = "Field `FWPWRITE6` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite6R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE6` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite6(&self) -> Fwpwrite6R {
        Fwpwrite6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite6(&mut self) -> Fwpwrite6W<Fwpwrite6Spec> {
        Fwpwrite6W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite6Spec;
impl crate::RegisterSpec for Fwpwrite6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite6::R`](R) reader structure"]
impl crate::Readable for Fwpwrite6Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite6::W`](W) writer structure"]
impl crate::Writable for Fwpwrite6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE6 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
