#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `DIS_STANDBY` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyR = crate::BitReader;
#[doc = "Field `DIS_STANDBY` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SWINTF` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type EnableSwintfR = crate::BitReader;
#[doc = "Field `ENABLE_SWINTF` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type EnableSwintfW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `STANDBY_PW_SEL` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelR = crate::FieldReader;
#[doc = "Field `STANDBY_PW_SEL` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBY_MODE_SEL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelR = crate::BitReader;
#[doc = "Field `STANDBY_MODE_SEL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_STANDBY` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type ForceStandbyR = crate::BitReader;
#[doc = "Field `FORCE_STANDBY` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type ForceStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED21` reader - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type Reserved21R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED21` writer - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby(&self) -> DisStandbyR {
        DisStandbyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn enable_swintf(&self) -> EnableSwintfR {
        EnableSwintfR::new(((self.bits >> 3) & 1) != 0)
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
    pub fn standby_pw_sel(&self) -> StandbyPwSelR {
        StandbyPwSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel(&self) -> StandbyModeSelR {
        StandbyModeSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_standby(&self) -> ForceStandbyR {
        ForceStandbyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby(&mut self) -> DisStandbyW<CfgSpec> {
        DisStandbyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CfgSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn enable_swintf(&mut self) -> EnableSwintfW<CfgSpec> {
        EnableSwintfW::new(self, 3)
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
    pub fn standby_pw_sel(&mut self) -> StandbyPwSelW<CfgSpec> {
        StandbyPwSelW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel(&mut self) -> StandbyModeSelW<CfgSpec> {
        StandbyModeSelW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn force_standby(&mut self) -> ForceStandbyW<CfgSpec> {
        ForceStandbyW::new(self, 9)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<CfgSpec> {
        Reserved21W::new(self, 10)
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
