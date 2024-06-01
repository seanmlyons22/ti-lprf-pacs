#[doc = "Register `DATALOWER` reader"]
pub type R = crate::R<DatalowerSpec>;
#[doc = "Register `DATALOWER` writer"]
pub type W = crate::W<DatalowerSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DatalowerSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datalower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datalower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatalowerSpec;
impl crate::RegisterSpec for DatalowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datalower::R`](R) reader structure"]
impl crate::Readable for DatalowerSpec {}
#[doc = "`write(|w| ..)` method takes [`datalower::W`](W) writer structure"]
impl crate::Writable for DatalowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATALOWER to value 0"]
impl crate::Resettable for DatalowerSpec {
    const RESET_VALUE: u32 = 0;
}
