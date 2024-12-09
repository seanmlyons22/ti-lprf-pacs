#[doc = "Register `BATMONP0` reader"]
pub type R = crate::R<Batmonp0Spec>;
#[doc = "Register `BATMONP0` writer"]
pub type W = crate::W<Batmonp0Spec>;
#[doc = "Field `CFG` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type CfgR = crate::FieldReader;
#[doc = "Field `CFG` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<Batmonp0Spec> {
        CfgW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmonp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmonp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Batmonp0Spec;
impl crate::RegisterSpec for Batmonp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`batmonp0::R`](R) reader structure"]
impl crate::Readable for Batmonp0Spec {}
#[doc = "`write(|w| ..)` method takes [`batmonp0::W`](W) writer structure"]
impl crate::Writable for Batmonp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BATMONP0 to value 0"]
impl crate::Resettable for Batmonp0Spec {
    const RESET_VALUE: u32 = 0;
}
