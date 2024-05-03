#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `BP_TRIMCFG_EN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BpTrimcfgEnR = crate::BitReader;
#[doc = "Field `BP_TRIMCFG_EN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BpTrimcfgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_READACCESS` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DisReadaccessR = crate::BitReader;
#[doc = "Field `DIS_READACCESS` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DisReadaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_EFUSECLK` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DisEfuseclkR = crate::BitReader;
#[doc = "Field `DIS_EFUSECLK` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type DisEfuseclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENGR_TRIM_STICKY_EN` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EngrTrimStickyEnR = crate::BitReader;
#[doc = "Field `ENGR_TRIM_STICKY_EN` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EngrTrimStickyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCFG_STICKY_EN` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type FcfgStickyEnR = crate::BitReader;
#[doc = "Field `FCFG_STICKY_EN` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type FcfgStickyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFG_STICKY_EN` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type CcfgStickyEnR = crate::BitReader;
#[doc = "Field `CCFG_STICKY_EN` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type CcfgStickyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STICKY_EN` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type MainStickyEnR = crate::BitReader;
#[doc = "Field `MAIN_STICKY_EN` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type MainStickyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 29:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 29:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `DIS_FWTEST` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type DisFwtestR = crate::BitReader;
#[doc = "Field `DIS_FWTEST` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type DisFwtestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_trimcfg_en(&self) -> BpTrimcfgEnR {
        BpTrimcfgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_readaccess(&self) -> DisReadaccessR {
        DisReadaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_efuseclk(&self) -> DisEfuseclkR {
        DisEfuseclkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn engr_trim_sticky_en(&self) -> EngrTrimStickyEnR {
        EngrTrimStickyEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fcfg_sticky_en(&self) -> FcfgStickyEnR {
        FcfgStickyEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ccfg_sticky_en(&self) -> CcfgStickyEnR {
        CcfgStickyEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_sticky_en(&self) -> MainStickyEnR {
        MainStickyEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:29 - 29:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x0003_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_fwtest(&self) -> DisFwtestR {
        DisFwtestR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bp_trimcfg_en(&mut self) -> BpTrimcfgEnW<CfgSpec> {
        BpTrimcfgEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CfgSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_readaccess(&mut self) -> DisReadaccessW<CfgSpec> {
        DisReadaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_efuseclk(&mut self) -> DisEfuseclkW<CfgSpec> {
        DisEfuseclkW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<CfgSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn engr_trim_sticky_en(&mut self) -> EngrTrimStickyEnW<CfgSpec> {
        EngrTrimStickyEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fcfg_sticky_en(&mut self) -> FcfgStickyEnW<CfgSpec> {
        FcfgStickyEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ccfg_sticky_en(&mut self) -> CcfgStickyEnW<CfgSpec> {
        CcfgStickyEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn main_sticky_en(&mut self) -> MainStickyEnW<CfgSpec> {
        MainStickyEnW::new(self, 11)
    }
    #[doc = "Bits 12:29 - 29:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<CfgSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_fwtest(&mut self) -> DisFwtestW<CfgSpec> {
        DisFwtestW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<CfgSpec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
