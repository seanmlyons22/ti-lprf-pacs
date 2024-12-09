#[doc = "Register `FVHVCT3` reader"]
pub type R = crate::R<Fvhvct3Spec>;
#[doc = "Register `FVHVCT3` writer"]
pub type W = crate::W<Fvhvct3Spec>;
#[doc = "Field `VHVCT_READ` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvctReadR = crate::FieldReader;
#[doc = "Field `VHVCT_READ` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvctReadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `WCT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type WctR = crate::FieldReader;
#[doc = "Field `WCT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type WctW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_read(&self) -> VhvctReadR {
        VhvctReadR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wct(&self) -> WctR {
        WctR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_read(&mut self) -> VhvctReadW<Fvhvct3Spec> {
        VhvctReadW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wct(&mut self) -> WctW<Fvhvct3Spec> {
        WctW::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvhvct3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvhvct3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fvhvct3Spec;
impl crate::RegisterSpec for Fvhvct3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvhvct3::R`](R) reader structure"]
impl crate::Readable for Fvhvct3Spec {}
#[doc = "`write(|w| ..)` method takes [`fvhvct3::W`](W) writer structure"]
impl crate::Writable for Fvhvct3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVHVCT3 to value 0x000f_0000"]
impl crate::Resettable for Fvhvct3Spec {
    const RESET_VALUE: u32 = 0x000f_0000;
}
