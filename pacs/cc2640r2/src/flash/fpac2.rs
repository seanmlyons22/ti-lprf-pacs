#[doc = "Register `FPAC2` reader"]
pub type R = crate::R<Fpac2Spec>;
#[doc = "Register `FPAC2` writer"]
pub type W = crate::W<Fpac2Spec>;
#[doc = "Field `PAGP` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type PagpR = crate::FieldReader<u16>;
#[doc = "Field `PAGP` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type PagpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pagp(&self) -> PagpR {
        PagpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pagp(&mut self) -> PagpW<Fpac2Spec> {
        PagpW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpac2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpac2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpac2Spec;
impl crate::RegisterSpec for Fpac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpac2::R`](R) reader structure"]
impl crate::Readable for Fpac2Spec {}
#[doc = "`write(|w| ..)` method takes [`fpac2::W`](W) writer structure"]
impl crate::Writable for Fpac2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPAC2 to value 0"]
impl crate::Resettable for Fpac2Spec {
    const RESET_VALUE: u32 = 0;
}
