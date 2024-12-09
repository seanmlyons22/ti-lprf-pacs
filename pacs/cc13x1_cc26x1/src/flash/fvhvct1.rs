#[doc = "Register `FVHVCT1` reader"]
pub type R = crate::R<Fvhvct1Spec>;
#[doc = "Register `FVHVCT1` writer"]
pub type W = crate::W<Fvhvct1Spec>;
#[doc = "Field `VHVCT_PV` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPvR = crate::FieldReader;
#[doc = "Field `VHVCT_PV` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM13_PV` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PvR = crate::FieldReader;
#[doc = "Field `TRIM13_PV` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `VHVCT_E` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvctER = crate::FieldReader;
#[doc = "Field `VHVCT_E` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvctEW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM13_E` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Trim13ER = crate::FieldReader;
#[doc = "Field `TRIM13_E` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Trim13EW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&self) -> VhvctPvR {
        VhvctPvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&self) -> Trim13PvR {
        Trim13PvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_e(&self) -> VhvctER {
        VhvctER::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&self) -> Trim13ER {
        Trim13ER::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_pv(&mut self) -> VhvctPvW<Fvhvct1Spec> {
        VhvctPvW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_pv(&mut self) -> Trim13PvW<Fvhvct1Spec> {
        Trim13PvW::new(self, 4)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_e(&mut self) -> VhvctEW<Fvhvct1Spec> {
        VhvctEW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_e(&mut self) -> Trim13EW<Fvhvct1Spec> {
        Trim13EW::new(self, 20)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvhvct1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvhvct1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fvhvct1Spec;
impl crate::RegisterSpec for Fvhvct1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvhvct1::R`](R) reader structure"]
impl crate::Readable for Fvhvct1Spec {}
#[doc = "`write(|w| ..)` method takes [`fvhvct1::W`](W) writer structure"]
impl crate::Writable for Fvhvct1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVHVCT1 to value 0x0084_0088"]
impl crate::Resettable for Fvhvct1Spec {
    const RESET_VALUE: u32 = 0x0084_0088;
}
