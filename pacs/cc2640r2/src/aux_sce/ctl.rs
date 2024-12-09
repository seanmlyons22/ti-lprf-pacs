#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CLK_EN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLE_STEP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SingleStepR = crate::BitReader;
#[doc = "Field `SINGLE_STEP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SingleStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_WU_HIGH` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type ForceWuHighR = crate::BitReader;
#[doc = "Field `FORCE_WU_HIGH` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type ForceWuHighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_WU_LOW` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type ForceWuLowR = crate::BitReader;
#[doc = "Field `FORCE_WU_LOW` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type ForceWuLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_FREEZE_EN` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type DbgFreezeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESET_VECTOR` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type ResetVectorR = crate::FieldReader;
#[doc = "Field `RESET_VECTOR` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type ResetVectorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `FORCE_EV_HIGH` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type ForceEvHighR = crate::FieldReader;
#[doc = "Field `FORCE_EV_HIGH` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type ForceEvHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FORCE_EV_LOW` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type ForceEvLowR = crate::FieldReader;
#[doc = "Field `FORCE_EV_LOW` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type ForceEvLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn single_step(&self) -> SingleStepR {
        SingleStepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_wu_high(&self) -> ForceWuHighR {
        ForceWuHighR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_wu_low(&self) -> ForceWuLowR {
        ForceWuLowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reset_vector(&self) -> ResetVectorR {
        ResetVectorR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_ev_high(&self) -> ForceEvHighR {
        ForceEvHighR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_ev_low(&self) -> ForceEvLowR {
        ForceEvLowR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<CtlSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn suspend(&mut self) -> SuspendW<CtlSpec> {
        SuspendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn single_step(&mut self) -> SingleStepW<CtlSpec> {
        SingleStepW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RestartW<CtlSpec> {
        RestartW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_wu_high(&mut self) -> ForceWuHighW<CtlSpec> {
        ForceWuHighW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_wu_low(&mut self) -> ForceWuLowW<CtlSpec> {
        ForceWuLowW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DbgFreezeEnW<CtlSpec> {
        DbgFreezeEnW::new(self, 6)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector(&mut self) -> ResetVectorW<CtlSpec> {
        ResetVectorW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_ev_high(&mut self) -> ForceEvHighW<CtlSpec> {
        ForceEvHighW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_ev_low(&mut self) -> ForceEvLowW<CtlSpec> {
        ForceEvLowW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
