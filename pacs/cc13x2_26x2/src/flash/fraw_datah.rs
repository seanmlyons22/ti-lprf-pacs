#[doc = "Register `FRAW_DATAH` reader"]
pub type R = crate::R<FrawDatahSpec>;
#[doc = "Register `FRAW_DATAH` writer"]
pub type W = crate::W<FrawDatahSpec>;
#[doc = "Field `FRAW_DATAH` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FrawDatahR = crate::FieldReader<u32>;
#[doc = "Field `FRAW_DATAH` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FrawDatahW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fraw_datah(&self) -> FrawDatahR {
        FrawDatahR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fraw_datah(&mut self) -> FrawDatahW<FrawDatahSpec> {
        FrawDatahW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fraw_datah::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fraw_datah::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrawDatahSpec;
impl crate::RegisterSpec for FrawDatahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fraw_datah::R`](R) reader structure"]
impl crate::Readable for FrawDatahSpec {}
#[doc = "`write(|w| ..)` method takes [`fraw_datah::W`](W) writer structure"]
impl crate::Writable for FrawDatahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAW_DATAH to value 0"]
impl crate::Resettable for FrawDatahSpec {
    const RESET_VALUE: u32 = 0;
}
