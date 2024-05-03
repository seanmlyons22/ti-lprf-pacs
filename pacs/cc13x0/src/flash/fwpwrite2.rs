#[doc = "Register `FWPWRITE2` reader"]
pub type R = crate::R<Fwpwrite2Spec>;
#[doc = "Register `FWPWRITE2` writer"]
pub type W = crate::W<Fwpwrite2Spec>;
#[doc = "Field `FWPWRITE2` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite2R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE2` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite2(&self) -> Fwpwrite2R {
        Fwpwrite2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite2(&mut self) -> Fwpwrite2W<Fwpwrite2Spec> {
        Fwpwrite2W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite2Spec;
impl crate::RegisterSpec for Fwpwrite2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite2::R`](R) reader structure"]
impl crate::Readable for Fwpwrite2Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite2::W`](W) writer structure"]
impl crate::Writable for Fwpwrite2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE2 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
