#[doc = "Register `FWPWRITE0` reader"]
pub type R = crate::R<Fwpwrite0Spec>;
#[doc = "Register `FWPWRITE0` writer"]
pub type W = crate::W<Fwpwrite0Spec>;
#[doc = "Field `FWPWRITE0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite0R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite0(&self) -> Fwpwrite0R {
        Fwpwrite0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite0(&mut self) -> Fwpwrite0W<Fwpwrite0Spec> {
        Fwpwrite0W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite0Spec;
impl crate::RegisterSpec for Fwpwrite0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite0::R`](R) reader structure"]
impl crate::Readable for Fwpwrite0Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite0::W`](W) writer structure"]
impl crate::Writable for Fwpwrite0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE0 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
