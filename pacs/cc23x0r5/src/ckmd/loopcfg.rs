#[doc = "Register `LOOPCFG` reader"]
pub type R = crate::R<LoopcfgSpec>;
#[doc = "Register `LOOPCFG` writer"]
pub type W = crate::W<LoopcfgSpec>;
#[doc = "Field `KI` reader - 2:0\\]
Integral loop coefficient"]
pub type KiR = crate::FieldReader;
#[doc = "Field `KI` writer - 2:0\\]
Integral loop coefficient"]
pub type KiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KP` reader - 5:3\\]
Proportional loop coefficient"]
pub type KpR = crate::FieldReader;
#[doc = "Field `KP` writer - 5:3\\]
Proportional loop coefficient"]
pub type KpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OOR_LIMIT` reader - 9:6\\]
Out-of-range threshold"]
pub type OorLimitR = crate::FieldReader;
#[doc = "Field `OOR_LIMIT` writer - 9:6\\]
Out-of-range threshold"]
pub type OorLimitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SETTLED_TARGET` reader - 14:10\\]
Number of updates before HFOSC is considered \"settled\""]
pub type SettledTargetR = crate::FieldReader;
#[doc = "Field `SETTLED_TARGET` writer - 14:10\\]
Number of updates before HFOSC is considered \"settled\""]
pub type SettledTargetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `KI_BOOST` reader - 17:15\\]
Integral loop coefficient during BOOST"]
pub type KiBoostR = crate::FieldReader;
#[doc = "Field `KI_BOOST` writer - 17:15\\]
Integral loop coefficient during BOOST"]
pub type KiBoostW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KP_BOOST` reader - 20:18\\]
Proportional loop coefficient during BOOST"]
pub type KpBoostR = crate::FieldReader;
#[doc = "Field `KP_BOOST` writer - 20:18\\]
Proportional loop coefficient during BOOST"]
pub type KpBoostW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BOOST_TARGET` reader - 25:21\\]
Number of error-updates using BOOST values, before using KI/KP"]
pub type BoostTargetR = crate::FieldReader;
#[doc = "Field `BOOST_TARGET` writer - 25:21\\]
Number of error-updates using BOOST values, before using KI/KP"]
pub type BoostTargetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FINETRIM_INIT` reader - 31:26\\]
Initial value for the resistor fine trim"]
pub type FinetrimInitR = crate::FieldReader;
#[doc = "Field `FINETRIM_INIT` writer - 31:26\\]
Initial value for the resistor fine trim"]
pub type FinetrimInitW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Integral loop coefficient"]
    #[inline(always)]
    pub fn ki(&self) -> KiR {
        KiR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Proportional loop coefficient"]
    #[inline(always)]
    pub fn kp(&self) -> KpR {
        KpR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Out-of-range threshold"]
    #[inline(always)]
    pub fn oor_limit(&self) -> OorLimitR {
        OorLimitR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Number of updates before HFOSC is considered \"settled\""]
    #[inline(always)]
    pub fn settled_target(&self) -> SettledTargetR {
        SettledTargetR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
Integral loop coefficient during BOOST"]
    #[inline(always)]
    pub fn ki_boost(&self) -> KiBoostR {
        KiBoostR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Proportional loop coefficient during BOOST"]
    #[inline(always)]
    pub fn kp_boost(&self) -> KpBoostR {
        KpBoostR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Number of error-updates using BOOST values, before using KI/KP"]
    #[inline(always)]
    pub fn boost_target(&self) -> BoostTargetR {
        BoostTargetR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Initial value for the resistor fine trim"]
    #[inline(always)]
    pub fn finetrim_init(&self) -> FinetrimInitR {
        FinetrimInitR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Integral loop coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ki(&mut self) -> KiW<LoopcfgSpec> {
        KiW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Proportional loop coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn kp(&mut self) -> KpW<LoopcfgSpec> {
        KpW::new(self, 3)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Out-of-range threshold"]
    #[inline(always)]
    #[must_use]
    pub fn oor_limit(&mut self) -> OorLimitW<LoopcfgSpec> {
        OorLimitW::new(self, 6)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Number of updates before HFOSC is considered \"settled\""]
    #[inline(always)]
    #[must_use]
    pub fn settled_target(&mut self) -> SettledTargetW<LoopcfgSpec> {
        SettledTargetW::new(self, 10)
    }
    #[doc = "Bits 15:17 - 17:15\\]
Integral loop coefficient during BOOST"]
    #[inline(always)]
    #[must_use]
    pub fn ki_boost(&mut self) -> KiBoostW<LoopcfgSpec> {
        KiBoostW::new(self, 15)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Proportional loop coefficient during BOOST"]
    #[inline(always)]
    #[must_use]
    pub fn kp_boost(&mut self) -> KpBoostW<LoopcfgSpec> {
        KpBoostW::new(self, 18)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Number of error-updates using BOOST values, before using KI/KP"]
    #[inline(always)]
    #[must_use]
    pub fn boost_target(&mut self) -> BoostTargetW<LoopcfgSpec> {
        BoostTargetW::new(self, 21)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Initial value for the resistor fine trim"]
    #[inline(always)]
    #[must_use]
    pub fn finetrim_init(&mut self) -> FinetrimInitW<LoopcfgSpec> {
        FinetrimInitW::new(self, 26)
    }
}
#[doc = "Configuration Register for the Tracking Loop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loopcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loopcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoopcfgSpec;
impl crate::RegisterSpec for LoopcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loopcfg::R`](R) reader structure"]
impl crate::Readable for LoopcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`loopcfg::W`](W) writer structure"]
impl crate::Writable for LoopcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOPCFG to value 0x605e_33b3"]
impl crate::Resettable for LoopcfgSpec {
    const RESET_VALUE: u32 = 0x605e_33b3;
}
