#[doc = "Register `TMUTE4` reader"]
pub type R = crate::R<Tmute4Spec>;
#[doc = "Register `TMUTE4` writer"]
pub type W = crate::W<Tmute4Spec>;
#[doc = "Field `HENSEL` reader - 2:0\\]
DCDC: Control PMOS switch strength"]
pub type HenselR = crate::FieldReader;
#[doc = "Field `HENSEL` writer - 2:0\\]
DCDC: Control PMOS switch strength"]
pub type HenselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LENSEL` reader - 5:3\\]
DCDC: Control NMOS switch strength"]
pub type LenselR = crate::FieldReader;
#[doc = "Field `LENSEL` writer - 5:3\\]
DCDC: Control NMOS switch strength"]
pub type LenselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTIME` reader - 7:6\\]
DCDC: Dead time trim"]
pub type DtimeR = crate::FieldReader;
#[doc = "Field `DTIME` writer - 7:6\\]
DCDC: Dead time trim"]
pub type DtimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IPEAK` reader - 10:8\\]
DCDC: Set inductor peak current"]
pub type IpeakR = crate::FieldReader;
#[doc = "Field `IPEAK` writer - 10:8\\]
DCDC: Set inductor peak current"]
pub type IpeakW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCDCLOAD` reader - 12:11\\]
DCDC: Set DCDC load current threshold"]
pub type DcdcloadR = crate::FieldReader;
#[doc = "Field `DCDCLOAD` writer - 12:11\\]
DCDC: Set DCDC load current threshold"]
pub type DcdcloadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MIN` reader - 15:13\\]
IOC: Minimum IO drive strength"]
pub type MinR = crate::FieldReader;
#[doc = "Field `MIN` writer - 15:13\\]
IOC: Minimum IO drive strength"]
pub type MinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MED` reader - 18:16\\]
IOC: Medium IO drive strength"]
pub type MedR = crate::FieldReader;
#[doc = "Field `MED` writer - 18:16\\]
IOC: Medium IO drive strength"]
pub type MedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAX` reader - 21:19\\]
IOC: Maximum IO drive strength"]
pub type MaxR = crate::FieldReader;
#[doc = "Field `MAX` writer - 21:19\\]
IOC: Maximum IO drive strength"]
pub type MaxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IOSTRCFG1` reader - 25:22\\]
BATMON: IO drive strength conversion parameter for the first stage"]
pub type Iostrcfg1R = crate::FieldReader;
#[doc = "Field `IOSTRCFG1` writer - 25:22\\]
BATMON: IO drive strength conversion parameter for the first stage"]
pub type Iostrcfg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IOSTRCFG2` reader - 27:26\\]
BATMON: IO drive strength conversion parameter for the second stage"]
pub type Iostrcfg2R = crate::FieldReader;
#[doc = "Field `IOSTRCFG2` writer - 27:26\\]
BATMON: IO drive strength conversion parameter for the second stage"]
pub type Iostrcfg2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RECHCOMPREFLVL` reader - 31:28\\]
PMUREG: Recharge comparator reference level. Valid values are 0x2 to 0xA. Recommended default value is 0x9. Setting 0x2 corresponds to VDDR = 1.35V and 0xA corresponds to VDDR = 1.47V with 15mV linear increase for every step."]
pub type RechcompreflvlR = crate::FieldReader;
#[doc = "Field `RECHCOMPREFLVL` writer - 31:28\\]
PMUREG: Recharge comparator reference level. Valid values are 0x2 to 0xA. Recommended default value is 0x9. Setting 0x2 corresponds to VDDR = 1.35V and 0xA corresponds to VDDR = 1.47V with 15mV linear increase for every step."]
pub type RechcompreflvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DCDC: Control PMOS switch strength"]
    #[inline(always)]
    pub fn hensel(&self) -> HenselR {
        HenselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
DCDC: Control NMOS switch strength"]
    #[inline(always)]
    pub fn lensel(&self) -> LenselR {
        LenselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
DCDC: Dead time trim"]
    #[inline(always)]
    pub fn dtime(&self) -> DtimeR {
        DtimeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DCDC: Set inductor peak current"]
    #[inline(always)]
    pub fn ipeak(&self) -> IpeakR {
        IpeakR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
DCDC: Set DCDC load current threshold"]
    #[inline(always)]
    pub fn dcdcload(&self) -> DcdcloadR {
        DcdcloadR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
IOC: Minimum IO drive strength"]
    #[inline(always)]
    pub fn min(&self) -> MinR {
        MinR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
IOC: Medium IO drive strength"]
    #[inline(always)]
    pub fn med(&self) -> MedR {
        MedR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - 21:19\\]
IOC: Maximum IO drive strength"]
    #[inline(always)]
    pub fn max(&self) -> MaxR {
        MaxR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - 25:22\\]
BATMON: IO drive strength conversion parameter for the first stage"]
    #[inline(always)]
    pub fn iostrcfg1(&self) -> Iostrcfg1R {
        Iostrcfg1R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
BATMON: IO drive strength conversion parameter for the second stage"]
    #[inline(always)]
    pub fn iostrcfg2(&self) -> Iostrcfg2R {
        Iostrcfg2R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
PMUREG: Recharge comparator reference level. Valid values are 0x2 to 0xA. Recommended default value is 0x9. Setting 0x2 corresponds to VDDR = 1.35V and 0xA corresponds to VDDR = 1.47V with 15mV linear increase for every step."]
    #[inline(always)]
    pub fn rechcompreflvl(&self) -> RechcompreflvlR {
        RechcompreflvlR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DCDC: Control PMOS switch strength"]
    #[inline(always)]
    #[must_use]
    pub fn hensel(&mut self) -> HenselW<Tmute4Spec> {
        HenselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
DCDC: Control NMOS switch strength"]
    #[inline(always)]
    #[must_use]
    pub fn lensel(&mut self) -> LenselW<Tmute4Spec> {
        LenselW::new(self, 3)
    }
    #[doc = "Bits 6:7 - 7:6\\]
DCDC: Dead time trim"]
    #[inline(always)]
    #[must_use]
    pub fn dtime(&mut self) -> DtimeW<Tmute4Spec> {
        DtimeW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DCDC: Set inductor peak current"]
    #[inline(always)]
    #[must_use]
    pub fn ipeak(&mut self) -> IpeakW<Tmute4Spec> {
        IpeakW::new(self, 8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
DCDC: Set DCDC load current threshold"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcload(&mut self) -> DcdcloadW<Tmute4Spec> {
        DcdcloadW::new(self, 11)
    }
    #[doc = "Bits 13:15 - 15:13\\]
IOC: Minimum IO drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MinW<Tmute4Spec> {
        MinW::new(self, 13)
    }
    #[doc = "Bits 16:18 - 18:16\\]
IOC: Medium IO drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn med(&mut self) -> MedW<Tmute4Spec> {
        MedW::new(self, 16)
    }
    #[doc = "Bits 19:21 - 21:19\\]
IOC: Maximum IO drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MaxW<Tmute4Spec> {
        MaxW::new(self, 19)
    }
    #[doc = "Bits 22:25 - 25:22\\]
BATMON: IO drive strength conversion parameter for the first stage"]
    #[inline(always)]
    #[must_use]
    pub fn iostrcfg1(&mut self) -> Iostrcfg1W<Tmute4Spec> {
        Iostrcfg1W::new(self, 22)
    }
    #[doc = "Bits 26:27 - 27:26\\]
BATMON: IO drive strength conversion parameter for the second stage"]
    #[inline(always)]
    #[must_use]
    pub fn iostrcfg2(&mut self) -> Iostrcfg2W<Tmute4Spec> {
        Iostrcfg2W::new(self, 26)
    }
    #[doc = "Bits 28:31 - 31:28\\]
PMUREG: Recharge comparator reference level. Valid values are 0x2 to 0xA. Recommended default value is 0x9. Setting 0x2 corresponds to VDDR = 1.35V and 0xA corresponds to VDDR = 1.47V with 15mV linear increase for every step."]
    #[inline(always)]
    #[must_use]
    pub fn rechcompreflvl(&mut self) -> RechcompreflvlW<Tmute4Spec> {
        RechcompreflvlW::new(self, 28)
    }
}
#[doc = "TMUTE4 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmute4Spec;
impl crate::RegisterSpec for Tmute4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmute4::R`](R) reader structure"]
impl crate::Readable for Tmute4Spec {}
#[doc = "`write(|w| ..)` method takes [`tmute4::W`](W) writer structure"]
impl crate::Writable for Tmute4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMUTE4 to value 0xb02e_603f"]
impl crate::Resettable for Tmute4Spec {
    const RESET_VALUE: u32 = 0xb02e_603f;
}
