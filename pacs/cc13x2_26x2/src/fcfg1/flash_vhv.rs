#[doc = "Register `FLASH_VHV` reader"]
pub type R = crate::R<FlashVhvSpec>;
#[doc = "Register `FLASH_VHV` writer"]
pub type W = crate::W<FlashVhvSpec>;
#[doc = "Field `VHV_E` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvER = crate::FieldReader;
#[doc = "Field `VHV_E` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type VhvEW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED0` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM13_E` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type Trim13ER = crate::FieldReader;
#[doc = "Field `TRIM13_E` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type Trim13EW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED1` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VHV_P` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvPR = crate::FieldReader;
#[doc = "Field `VHV_P` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VhvPW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED2` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM13_P` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PR = crate::FieldReader;
#[doc = "Field `TRIM13_P` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Trim13PW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED3` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e(&self) -> VhvER {
        VhvER::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&self) -> Trim13ER {
        Trim13ER::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_p(&self) -> VhvPR {
        VhvPR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_p(&self) -> Trim13PR {
        Trim13PR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_e(&mut self) -> VhvEW<FlashVhvSpec> {
        VhvEW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<FlashVhvSpec> {
        Reserved0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_e(&mut self) -> Trim13EW<FlashVhvSpec> {
        Trim13EW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<FlashVhvSpec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhv_p(&mut self) -> VhvPW<FlashVhvSpec> {
        VhvPW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FlashVhvSpec> {
        Reserved2W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim13_p(&mut self) -> Trim13PW<FlashVhvSpec> {
        Trim13PW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<FlashVhvSpec> {
        Reserved3W::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_vhv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_vhv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashVhvSpec;
impl crate::RegisterSpec for FlashVhvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_vhv::R`](R) reader structure"]
impl crate::Readable for FlashVhvSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_vhv::W`](W) writer structure"]
impl crate::Writable for FlashVhvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_VHV to value 0x04"]
impl crate::Resettable for FlashVhvSpec {
    const RESET_VALUE: u32 = 0x04;
}
