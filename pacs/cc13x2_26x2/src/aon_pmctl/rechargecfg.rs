#[doc = "Register `RECHARGECFG` reader"]
pub type R = crate::R<RechargecfgSpec>;
#[doc = "Register `RECHARGECFG` writer"]
pub type W = crate::W<RechargecfgSpec>;
#[doc = "Field `PER_E` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type PerER = crate::FieldReader;
#[doc = "Field `PER_E` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type PerEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PER_M` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type PerMR = crate::FieldReader;
#[doc = "Field `PER_M` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type PerMW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAX_PER_E` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type MaxPerER = crate::FieldReader;
#[doc = "Field `MAX_PER_E` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type MaxPerEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAX_PER_M` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type MaxPerMR = crate::FieldReader;
#[doc = "Field `MAX_PER_M` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub type MaxPerMW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `C1` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type C1R = crate::FieldReader;
#[doc = "Field `C1` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type C1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type C2R = crate::FieldReader;
#[doc = "Field `C2` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type C2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "3: External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    Comparator = 3,
    #[doc = "2: Adaptive timer"]
    Adaptive = 2,
    #[doc = "1: Static timer"]
    Static = 1,
    #[doc = "0: Recharge disabled"]
    Off = 0,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            3 => Mode::Comparator,
            2 => Mode::Adaptive,
            1 => Mode::Static,
            0 => Mode::Off,
            _ => unreachable!(),
        }
    }
    #[doc = "External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    #[inline(always)]
    pub fn is_comparator(&self) -> bool {
        *self == Mode::Comparator
    }
    #[doc = "Adaptive timer"]
    #[inline(always)]
    pub fn is_adaptive(&self) -> bool {
        *self == Mode::Adaptive
    }
    #[doc = "Static timer"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Mode::Static
    }
    #[doc = "Recharge disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
}
#[doc = "Field `MODE` writer - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    #[inline(always)]
    pub fn comparator(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Comparator)
    }
    #[doc = "Adaptive timer"]
    #[inline(always)]
    pub fn adaptive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Adaptive)
    }
    #[doc = "Static timer"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Static)
    }
    #[doc = "Recharge disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PerER {
        PerER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PerMR {
        PerMR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_e(&self) -> MaxPerER {
        MaxPerER::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_m(&self) -> MaxPerMR {
        MaxPerMR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c1(&self) -> C1R {
        C1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c2(&self) -> C2R {
        C2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PerEW<RechargecfgSpec> {
        PerEW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PerMW<RechargecfgSpec> {
        PerMW::new(self, 3)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_per_e(&mut self) -> MaxPerEW<RechargecfgSpec> {
        MaxPerEW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_per_m(&mut self) -> MaxPerMW<RechargecfgSpec> {
        MaxPerMW::new(self, 11)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1W<RechargecfgSpec> {
        C1W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2W<RechargecfgSpec> {
        C2W::new(self, 20)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<RechargecfgSpec> {
        ModeW::new(self, 30)
    }
}
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RechargecfgSpec;
impl crate::RegisterSpec for RechargecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rechargecfg::R`](R) reader structure"]
impl crate::Readable for RechargecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rechargecfg::W`](W) writer structure"]
impl crate::Writable for RechargecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECHARGECFG to value 0xc000_0000"]
impl crate::Resettable for RechargecfgSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
