#[doc = "Register `FWPWRITE5` reader"]
pub type R = crate::R<Fwpwrite5Spec>;
#[doc = "Register `FWPWRITE5` writer"]
pub type W = crate::W<Fwpwrite5Spec>;
#[doc = "Field `FWPWRITE5` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite5R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE5` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite5(&self) -> Fwpwrite5R {
        Fwpwrite5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite5(&mut self) -> Fwpwrite5W<Fwpwrite5Spec> {
        Fwpwrite5W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite5Spec;
impl crate::RegisterSpec for Fwpwrite5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite5::R`](R) reader structure"]
impl crate::Readable for Fwpwrite5Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite5::W`](W) writer structure"]
impl crate::Writable for Fwpwrite5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE5 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
