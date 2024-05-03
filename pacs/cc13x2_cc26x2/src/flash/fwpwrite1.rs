#[doc = "Register `FWPWRITE1` reader"]
pub type R = crate::R<Fwpwrite1Spec>;
#[doc = "Register `FWPWRITE1` writer"]
pub type W = crate::W<Fwpwrite1Spec>;
#[doc = "Field `FWPWRITE1` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite1R = crate::FieldReader<u32>;
#[doc = "Field `FWPWRITE1` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Fwpwrite1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite1(&self) -> Fwpwrite1R {
        Fwpwrite1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fwpwrite1(&mut self) -> Fwpwrite1W<Fwpwrite1Spec> {
        Fwpwrite1W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fwpwrite1Spec;
impl crate::RegisterSpec for Fwpwrite1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite1::R`](R) reader structure"]
impl crate::Readable for Fwpwrite1Spec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite1::W`](W) writer structure"]
impl crate::Writable for Fwpwrite1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE1 to value 0xffff_ffff"]
impl crate::Resettable for Fwpwrite1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
