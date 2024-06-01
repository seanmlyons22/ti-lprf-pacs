#[doc = "Register `TEMPP0` reader"]
pub type R = crate::R<Tempp0Spec>;
#[doc = "Register `TEMPP0` writer"]
pub type W = crate::W<Tempp0Spec>;
#[doc = "Field `CFG` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CfgR = crate::FieldReader;
#[doc = "Field `CFG` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<Tempp0Spec> {
        CfgW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Tempp0Spec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tempp0Spec;
impl crate::RegisterSpec for Tempp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempp0::R`](R) reader structure"]
impl crate::Readable for Tempp0Spec {}
#[doc = "`write(|w| ..)` method takes [`tempp0::W`](W) writer structure"]
impl crate::Writable for Tempp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEMPP0 to value 0"]
impl crate::Resettable for Tempp0Spec {
    const RESET_VALUE: u32 = 0;
}
