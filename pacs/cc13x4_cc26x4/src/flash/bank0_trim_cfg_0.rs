#[doc = "Register `BANK0_TRIM_CFG_0` reader"]
pub type R = crate::R<Bank0TrimCfg0Spec>;
#[doc = "Register `BANK0_TRIM_CFG_0` writer"]
pub type W = crate::W<Bank0TrimCfg0Spec>;
#[doc = "Field `BANK0_TRIM_CFG_0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Bank0TrimCfg0R = crate::FieldReader<u32>;
#[doc = "Field `BANK0_TRIM_CFG_0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type Bank0TrimCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank0_trim_cfg_0(&self) -> Bank0TrimCfg0R {
        Bank0TrimCfg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bank0_trim_cfg_0(&mut self) -> Bank0TrimCfg0W<Bank0TrimCfg0Spec> {
        Bank0TrimCfg0W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0_trim_cfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0_trim_cfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bank0TrimCfg0Spec;
impl crate::RegisterSpec for Bank0TrimCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank0_trim_cfg_0::R`](R) reader structure"]
impl crate::Readable for Bank0TrimCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`bank0_trim_cfg_0::W`](W) writer structure"]
impl crate::Writable for Bank0TrimCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK0_TRIM_CFG_0 to value 0"]
impl crate::Resettable for Bank0TrimCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
