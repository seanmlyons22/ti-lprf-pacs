#[doc = "Register `FWPWRITE7` reader"]
pub type R = crate::R<Fwpwrite7Spec>;
#[doc = "Register `FWPWRITE7` writer"]
pub type W = crate::W<Fwpwrite7Spec>;
#[doc = "Field `FWPWRITE7` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite7R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE7` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite7(&self) -> Fwpwrite7R {
        Fwpwrite7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite7(&mut self) -> Fwpwrite7W<Fwpwrite7Spec> {
        Fwpwrite7W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite7Spec;
impl crate::RegisterSpec for Fwpwrite7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite7::R`](R) reader structure"]
impl crate::Readable for Fwpwrite7Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite7::W`](W) writer structure"]
impl crate::Writable for Fwpwrite7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE7 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite7Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
