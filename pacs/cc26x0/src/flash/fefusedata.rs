#[doc = "Register `FEFUSEDATA` reader"]
pub type R = crate::R<FefusedataSpec>;
#[doc = "Register `FEFUSEDATA` writer"]
pub type W = crate::W<FefusedataSpec>;
#[doc = "Field `FEFUSEDATA` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FefusedataR = crate::FieldReader<u32>;
#[doc = "Field `FEFUSEDATA` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FefusedataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fefusedata(&self) -> FefusedataR {
        FefusedataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fefusedata(&mut self) -> FefusedataW<FefusedataSpec> {
        FefusedataW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fefusedata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fefusedata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FefusedataSpec;
impl crate::RegisterSpec for FefusedataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fefusedata::R`](R) reader structure"]
impl crate::Readable for FefusedataSpec {}
#[doc = "`write(|w| ..)` method takes [`fefusedata::W`](W) writer structure"]
impl crate::Writable for FefusedataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEFUSEDATA to value 0"]
impl crate::Resettable for FefusedataSpec {
    const RESET_VALUE: u32 = 0;
}
