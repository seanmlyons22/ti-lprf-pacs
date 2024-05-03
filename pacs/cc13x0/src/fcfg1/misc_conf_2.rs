#[doc = "Register `MISC_CONF_2` reader"]
pub type R = crate::R<MiscConf2Spec>;
#[doc = "Register `MISC_CONF_2` writer"]
pub type W = crate::W<MiscConf2Spec>;
#[doc = "Field `HPOSC_COMP_P3` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP3R = crate::FieldReader;
#[doc = "Field `HPOSC_COMP_P3` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type HposcCompP3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p3(&self) -> HposcCompP3R {
        HposcCompP3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hposc_comp_p3(&mut self) -> HposcCompP3W<MiscConf2Spec> {
        HposcCompP3W::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscConf2Spec;
impl crate::RegisterSpec for MiscConf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_conf_2::R`](R) reader structure"]
impl crate::Readable for MiscConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`misc_conf_2::W`](W) writer structure"]
impl crate::Writable for MiscConf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_CONF_2 to value 0xffff_ff00"]
impl crate::Resettable for MiscConf2Spec {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
