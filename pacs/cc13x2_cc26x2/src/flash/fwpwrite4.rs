#[doc = "Register `FWPWRITE4` reader"]
pub type R = crate::R<Fwpwrite4Spec>;
#[doc = "Register `FWPWRITE4` writer"]
pub type W = crate::W<Fwpwrite4Spec>;
#[doc = "Field `FWPWRITE4` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite4R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE4` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite4(&self) -> Fwpwrite4R {
        Fwpwrite4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite4(&mut self) -> Fwpwrite4W<Fwpwrite4Spec> {
        Fwpwrite4W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite4Spec;
impl crate::RegisterSpec for Fwpwrite4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite4::R`](R) reader structure"]
impl crate::Readable for Fwpwrite4Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite4::W`](W) writer structure"]
impl crate::Writable for Fwpwrite4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE4 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
