#[doc = "Register `LDO_TRIM` reader"]
pub type R = crate::R<LdoTrimSpec>;
#[doc = "Register `LDO_TRIM` writer"]
pub type W = crate::W<LdoTrimSpec>;
#[doc = "Field `VTRIM_DELTA` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VtrimDeltaR = crate::FieldReader;
#[doc = "Field `VTRIM_DELTA` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VtrimDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED1` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ITRIM_UDIGLDO` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type ItrimUdigldoR = crate::FieldReader;
#[doc = "Field `ITRIM_UDIGLDO` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type ItrimUdigldoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITRIM_DIGLDO_LOAD` reader - 12:11\\]
Internal. Only to be used through TI provided API."]
pub type ItrimDigldoLoadR = crate::FieldReader;
#[doc = "Field `ITRIM_DIGLDO_LOAD` writer - 12:11\\]
Internal. Only to be used through TI provided API."]
pub type ItrimDigldoLoadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED2` reader - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 15:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GLDO_CURSRC` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type GldoCursrcR = crate::FieldReader;
#[doc = "Field `GLDO_CURSRC` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type GldoCursrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VDDR_TRIM_SLEEP` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimSleepR = crate::FieldReader;
#[doc = "Field `VDDR_TRIM_SLEEP` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub type VddrTrimSleepW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED4` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_delta(&self) -> VtrimDeltaR {
        VtrimDeltaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_udigldo(&self) -> ItrimUdigldoR {
        ItrimUdigldoR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_digldo_load(&self) -> ItrimDigldoLoadR {
        ItrimDigldoLoadR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldo_cursrc(&self) -> GldoCursrcR {
        GldoCursrcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep(&self) -> VddrTrimSleepR {
        VddrTrimSleepR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtrim_delta(&mut self) -> VtrimDeltaW<LdoTrimSpec> {
        VtrimDeltaW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<LdoTrimSpec> {
        Reserved1W::new(self, 3)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itrim_udigldo(&mut self) -> ItrimUdigldoW<LdoTrimSpec> {
        ItrimUdigldoW::new(self, 8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itrim_digldo_load(&mut self) -> ItrimDigldoLoadW<LdoTrimSpec> {
        ItrimDigldoLoadW::new(self, 11)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<LdoTrimSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gldo_cursrc(&mut self) -> GldoCursrcW<LdoTrimSpec> {
        GldoCursrcW::new(self, 16)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<LdoTrimSpec> {
        Reserved3W::new(self, 19)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_trim_sleep(&mut self) -> VddrTrimSleepW<LdoTrimSpec> {
        VddrTrimSleepW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<LdoTrimSpec> {
        Reserved4W::new(self, 29)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdoTrimSpec;
impl crate::RegisterSpec for LdoTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo_trim::R`](R) reader structure"]
impl crate::Readable for LdoTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo_trim::W`](W) writer structure"]
impl crate::Writable for LdoTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDO_TRIM to value 0xe0f8_e0fb"]
impl crate::Resettable for LdoTrimSpec {
    const RESET_VALUE: u32 = 0xe0f8_e0fb;
}
