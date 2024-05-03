#[doc = "Register `BANK1_TRIM_CFG_2` reader"]
pub type R = crate::R<Bank1TrimCfg2Spec>;
#[doc = "Register `BANK1_TRIM_CFG_2` writer"]
pub type W = crate::W<Bank1TrimCfg2Spec>;
#[doc = "Field `RESERVED32` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved32R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED32` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved32(&self) -> Reserved32R {
        Reserved32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved32(&mut self) -> Reserved32W<Bank1TrimCfg2Spec> {
        Reserved32W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1_trim_cfg_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1_trim_cfg_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bank1TrimCfg2Spec;
impl crate::RegisterSpec for Bank1TrimCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank1_trim_cfg_2::R`](R) reader structure"]
impl crate::Readable for Bank1TrimCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`bank1_trim_cfg_2::W`](W) writer structure"]
impl crate::Writable for Bank1TrimCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK1_TRIM_CFG_2 to value 0"]
impl crate::Resettable for Bank1TrimCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
