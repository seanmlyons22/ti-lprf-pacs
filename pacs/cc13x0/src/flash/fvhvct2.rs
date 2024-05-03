#[doc = "Register `FVHVCT2` reader"]
pub type R = crate::R<Fvhvct2Spec>;
#[doc = "Register `FVHVCT2` writer"]
pub type W = crate::W<Fvhvct2Spec>;
#[doc = "Field `RESERVED0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VHVCT_P` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPR = crate::FieldReader;
#[doc = "Field `VHVCT_P` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvctPW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM13_P` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PR = crate::FieldReader;
#[doc = "Field `TRIM13_P` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_p(&self) -> VhvctPR {
        VhvctPR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_p(&self) -> Trim13PR {
        Trim13PR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Fvhvct2Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_p(&mut self) -> VhvctPW<Fvhvct2Spec> {
        VhvctPW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_p(&mut self) -> Trim13PW<Fvhvct2Spec> {
        Trim13PW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<Fvhvct2Spec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvhvct2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvhvct2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fvhvct2Spec;
impl crate::RegisterSpec for Fvhvct2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvhvct2::R`](R) reader structure"]
impl crate::Readable for Fvhvct2Spec {}
#[doc = "`write(|w| ..)` method takes [`fvhvct2::W`](W) writer structure"]
impl crate::Writable for Fvhvct2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVHVCT2 to value 0x00a2_0000"]
impl crate::Resettable for Fvhvct2Spec {
    const RESET_VALUE: u32 = 0x00a2_0000;
}
