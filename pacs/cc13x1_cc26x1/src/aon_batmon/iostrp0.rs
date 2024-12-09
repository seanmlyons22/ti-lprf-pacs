#[doc = "Register `IOSTRP0` reader"]
pub type R = crate::R<Iostrp0Spec>;
#[doc = "Register `IOSTRP0` writer"]
pub type W = crate::W<Iostrp0Spec>;
#[doc = "Field `CFG1` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type Cfg1R = crate::FieldReader;
#[doc = "Field `CFG1` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type Cfg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG2` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type Cfg2R = crate::FieldReader;
#[doc = "Field `CFG2` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type Cfg2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg1(&self) -> Cfg1R {
        Cfg1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg2(&self) -> Cfg2R {
        Cfg2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> Cfg1W<Iostrp0Spec> {
        Cfg1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> Cfg2W<Iostrp0Spec> {
        Cfg2W::new(self, 4)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iostrp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iostrp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iostrp0Spec;
impl crate::RegisterSpec for Iostrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iostrp0::R`](R) reader structure"]
impl crate::Readable for Iostrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`iostrp0::W`](W) writer structure"]
impl crate::Writable for Iostrp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOSTRP0 to value 0x28"]
impl crate::Resettable for Iostrp0Spec {
    const RESET_VALUE: u32 = 0x28;
}
