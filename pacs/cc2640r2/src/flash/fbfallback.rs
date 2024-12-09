#[doc = "Register `FBFALLBACK` reader"]
pub type R = crate::R<FbfallbackSpec>;
#[doc = "Register `FBFALLBACK` writer"]
pub type W = crate::W<FbfallbackSpec>;
#[doc = "Field `BANKPWR0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr0R = crate::FieldReader;
#[doc = "Field `BANKPWR0` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR1` reader - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr1R = crate::FieldReader;
#[doc = "Field `BANKPWR1` writer - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR2` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr2R = crate::FieldReader;
#[doc = "Field `BANKPWR2` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR3` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr3R = crate::FieldReader;
#[doc = "Field `BANKPWR3` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR4` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr4R = crate::FieldReader;
#[doc = "Field `BANKPWR4` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR5` reader - 11:10\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr5R = crate::FieldReader;
#[doc = "Field `BANKPWR5` writer - 11:10\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR6` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr6R = crate::FieldReader;
#[doc = "Field `BANKPWR6` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKPWR7` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr7R = crate::FieldReader;
#[doc = "Field `BANKPWR7` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub type Bankpwr7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PWRSAV` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type RegPwrsavR = crate::FieldReader;
#[doc = "Field `REG_PWRSAV` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type RegPwrsavW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20R = crate::FieldReader;
#[doc = "Field `FSM_PWRSAV` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type FsmPwrsavR = crate::FieldReader;
#[doc = "Field `FSM_PWRSAV` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type FsmPwrsavW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr0(&self) -> Bankpwr0R {
        Bankpwr0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr1(&self) -> Bankpwr1R {
        Bankpwr1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr2(&self) -> Bankpwr2R {
        Bankpwr2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr3(&self) -> Bankpwr3R {
        Bankpwr3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr4(&self) -> Bankpwr4R {
        Bankpwr4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr5(&self) -> Bankpwr5R {
        Bankpwr5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr6(&self) -> Bankpwr6R {
        Bankpwr6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr7(&self) -> Bankpwr7R {
        Bankpwr7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg_pwrsav(&self) -> RegPwrsavR {
        RegPwrsavR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pwrsav(&self) -> FsmPwrsavR {
        FsmPwrsavR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr0(&mut self) -> Bankpwr0W<FbfallbackSpec> {
        Bankpwr0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr1(&mut self) -> Bankpwr1W<FbfallbackSpec> {
        Bankpwr1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr2(&mut self) -> Bankpwr2W<FbfallbackSpec> {
        Bankpwr2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr3(&mut self) -> Bankpwr3W<FbfallbackSpec> {
        Bankpwr3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr4(&mut self) -> Bankpwr4W<FbfallbackSpec> {
        Bankpwr4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr5(&mut self) -> Bankpwr5W<FbfallbackSpec> {
        Bankpwr5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr6(&mut self) -> Bankpwr6W<FbfallbackSpec> {
        Bankpwr6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bankpwr7(&mut self) -> Bankpwr7W<FbfallbackSpec> {
        Bankpwr7W::new(self, 14)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reg_pwrsav(&mut self) -> RegPwrsavW<FbfallbackSpec> {
        RegPwrsavW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_pwrsav(&mut self) -> FsmPwrsavW<FbfallbackSpec> {
        FsmPwrsavW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbfallback::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbfallback::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbfallbackSpec;
impl crate::RegisterSpec for FbfallbackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbfallback::R`](R) reader structure"]
impl crate::Readable for FbfallbackSpec {}
#[doc = "`write(|w| ..)` method takes [`fbfallback::W`](W) writer structure"]
impl crate::Writable for FbfallbackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBFALLBACK to value 0x0505_ffff"]
impl crate::Resettable for FbfallbackSpec {
    const RESET_VALUE: u32 = 0x0505_ffff;
}
