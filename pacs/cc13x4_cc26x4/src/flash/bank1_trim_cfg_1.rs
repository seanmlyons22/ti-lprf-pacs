#[doc = "Register `BANK1_TRIM_CFG_1` reader"]
pub type R = crate::R<Bank1TrimCfg1Spec>;
#[doc = "Register `BANK1_TRIM_CFG_1` writer"]
pub type W = crate::W<Bank1TrimCfg1Spec>;
#[doc = "Field `REDSWENW0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw0R = crate::BitReader;
#[doc = "Field `REDSWENW0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDSWENW1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw1R = crate::BitReader;
#[doc = "Field `REDSWENW1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDSWENW2` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw2R = crate::BitReader;
#[doc = "Field `REDSWENW2` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDSWENW3` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw3R = crate::BitReader;
#[doc = "Field `REDSWENW3` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type Redswenw3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDSWSELW0` reader - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw0R = crate::FieldReader;
#[doc = "Field `REDSWSELW0` writer - 9:4\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REDSWSELW1` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw1R = crate::FieldReader;
#[doc = "Field `REDSWSELW1` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REDSWSELW2` reader - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw2R = crate::FieldReader;
#[doc = "Field `REDSWSELW2` writer - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REDSWSELW3` reader - 27:22\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw3R = crate::FieldReader;
#[doc = "Field `REDSWSELW3` writer - 27:22\\]
Internal. Only to be used through TI provided API."]
pub type Redswselw3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw0(&self) -> Redswenw0R {
        Redswenw0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw1(&self) -> Redswenw1R {
        Redswenw1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw2(&self) -> Redswenw2R {
        Redswenw2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswenw3(&self) -> Redswenw3R {
        Redswenw3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw0(&self) -> Redswselw0R {
        Redswselw0R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw1(&self) -> Redswselw1R {
        Redswselw1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw2(&self) -> Redswselw2R {
        Redswselw2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - 27:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn redswselw3(&self) -> Redswselw3R {
        Redswselw3R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw0(&mut self) -> Redswenw0W<Bank1TrimCfg1Spec> {
        Redswenw0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw1(&mut self) -> Redswenw1W<Bank1TrimCfg1Spec> {
        Redswenw1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw2(&mut self) -> Redswenw2W<Bank1TrimCfg1Spec> {
        Redswenw2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswenw3(&mut self) -> Redswenw3W<Bank1TrimCfg1Spec> {
        Redswenw3W::new(self, 3)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw0(&mut self) -> Redswselw0W<Bank1TrimCfg1Spec> {
        Redswselw0W::new(self, 4)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw1(&mut self) -> Redswselw1W<Bank1TrimCfg1Spec> {
        Redswselw1W::new(self, 10)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw2(&mut self) -> Redswselw2W<Bank1TrimCfg1Spec> {
        Redswselw2W::new(self, 16)
    }
    #[doc = "Bits 22:27 - 27:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn redswselw3(&mut self) -> Redswselw3W<Bank1TrimCfg1Spec> {
        Redswselw3W::new(self, 22)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Bank1TrimCfg1Spec> {
        Reserved6W::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1_trim_cfg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1_trim_cfg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bank1TrimCfg1Spec;
impl crate::RegisterSpec for Bank1TrimCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank1_trim_cfg_1::R`](R) reader structure"]
impl crate::Readable for Bank1TrimCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`bank1_trim_cfg_1::W`](W) writer structure"]
impl crate::Writable for Bank1TrimCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK1_TRIM_CFG_1 to value 0"]
impl crate::Resettable for Bank1TrimCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
