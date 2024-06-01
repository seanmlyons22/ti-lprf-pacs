#[doc = "Register `FRAW_DATAL` reader"]
pub type R = crate::R<FrawDatalSpec>;
#[doc = "Register `FRAW_DATAL` writer"]
pub type W = crate::W<FrawDatalSpec>;
#[doc = "Field `FRAW_DATAL` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FrawDatalR = crate::FieldReader<u32>;
#[doc = "Field `FRAW_DATAL` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FrawDatalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fraw_datal(&self) -> FrawDatalR {
        FrawDatalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fraw_datal(&mut self) -> FrawDatalW<FrawDatalSpec> {
        FrawDatalW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fraw_datal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fraw_datal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrawDatalSpec;
impl crate::RegisterSpec for FrawDatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fraw_datal::R`](R) reader structure"]
impl crate::Readable for FrawDatalSpec {}
#[doc = "`write(|w| ..)` method takes [`fraw_datal::W`](W) writer structure"]
impl crate::Writable for FrawDatalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRAW_DATAL to value 0"]
impl crate::Resettable for FrawDatalSpec {
    const RESET_VALUE: u32 = 0;
}
