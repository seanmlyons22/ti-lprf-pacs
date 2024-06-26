#[doc = "Register `SELFTESTSIGN` reader"]
pub type R = crate::R<SelftestsignSpec>;
#[doc = "Register `SELFTESTSIGN` writer"]
pub type W = crate::W<SelftestsignSpec>;
#[doc = "Field `SIGNATURE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SignatureR = crate::FieldReader<u32>;
#[doc = "Field `SIGNATURE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type SignatureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn signature(&self) -> SignatureR {
        SignatureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn signature(&mut self) -> SignatureW<SelftestsignSpec> {
        SignatureW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selftestsign::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selftestsign::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelftestsignSpec;
impl crate::RegisterSpec for SelftestsignSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`selftestsign::R`](R) reader structure"]
impl crate::Readable for SelftestsignSpec {}
#[doc = "`write(|w| ..)` method takes [`selftestsign::W`](W) writer structure"]
impl crate::Writable for SelftestsignSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SELFTESTSIGN to value 0"]
impl crate::Resettable for SelftestsignSpec {
    const RESET_VALUE: u32 = 0;
}
