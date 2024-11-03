#[doc = "Register `ETPP` reader"]
pub type R = crate::R<EtppSpec>;
#[doc = "Register `ETPP` writer"]
pub type W = crate::W<EtppSpec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<EtppSpec> {
        Reserved0W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etpp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etpp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtppSpec;
impl crate::RegisterSpec for EtppSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etpp::R`](R) reader structure"]
impl crate::Readable for EtppSpec {}
#[doc = "`write(|w| ..)` method takes [`etpp::W`](W) writer structure"]
impl crate::Writable for EtppSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETPP to value 0"]
impl crate::Resettable for EtppSpec {
    const RESET_VALUE: u32 = 0;
}
