#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "3: VIMS Off mode"]
    Off = 3,
    #[doc = "1: VIMS Cache mode"]
    Cache = 1,
    #[doc = "0: VIMS GPRAM mode"]
    Gpram = 0,
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
#[doc = "Field `MODE` reader - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            3 => Some(Mode::Off),
            1 => Some(Mode::Cache),
            0 => Some(Mode::Gpram),
            _ => None,
        }
    }
    #[doc = "VIMS Off mode"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
    #[doc = "VIMS Cache mode"]
    #[inline(always)]
    pub fn is_cache(&self) -> bool {
        *self == Mode::Cache
    }
    #[doc = "VIMS GPRAM mode"]
    #[inline(always)]
    pub fn is_gpram(&self) -> bool {
        *self == Mode::Gpram
    }
}
#[doc = "Field `MODE` writer - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VIMS Off mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
    #[doc = "VIMS Cache mode"]
    #[inline(always)]
    pub fn cache(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Cache)
    }
    #[doc = "VIMS GPRAM mode"]
    #[inline(always)]
    pub fn gpram(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Gpram)
    }
}
#[doc = "Field `PREF_EN` reader - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_CFG` reader - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
pub type ArbCfgR = crate::BitReader;
#[doc = "Field `ARB_CFG` writer - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
pub type ArbCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBUS_LB_DIS` reader - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
pub type SysbusLbDisR = crate::BitReader;
#[doc = "Field `SYSBUS_LB_DIS` writer - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
pub type SysbusLbDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDCODE_LB_DIS` reader - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
pub type IdcodeLbDisR = crate::BitReader;
#[doc = "Field `IDCODE_LB_DIS` writer - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
pub type IdcodeLbDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `DYN_CG_EN` reader - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
pub type DynCgEnR = crate::BitReader;
#[doc = "Field `DYN_CG_EN` writer - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
pub type DynCgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATS_EN` reader - 30:30\\]
Set this bit to enable statistic counters."]
pub type StatsEnR = crate::BitReader;
#[doc = "Field `STATS_EN` writer - 30:30\\]
Set this bit to enable statistic counters."]
pub type StatsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATS_CLR` reader - 31:31\\]
Set this bit to clear statistic counters."]
pub type StatsClrR = crate::BitReader;
#[doc = "Field `STATS_CLR` writer - 31:31\\]
Set this bit to clear statistic counters."]
pub type StatsClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ArbCfgR {
        ArbCfgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SysbusLbDisR {
        SysbusLbDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IdcodeLbDisR {
        IdcodeLbDisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:28 - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x007f_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    pub fn dyn_cg_en(&self) -> DynCgEnR {
        DynCgEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Set this bit to enable statistic counters."]
    #[inline(always)]
    pub fn stats_en(&self) -> StatsEnR {
        StatsEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set this bit to clear statistic counters."]
    #[inline(always)]
    pub fn stats_clr(&self) -> StatsClrR {
        StatsClrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<CtlSpec> {
        PrefEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn arb_cfg(&mut self) -> ArbCfgW<CtlSpec> {
        ArbCfgW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sysbus_lb_dis(&mut self) -> SysbusLbDisW<CtlSpec> {
        SysbusLbDisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn idcode_lb_dis(&mut self) -> IdcodeLbDisW<CtlSpec> {
        IdcodeLbDisW::new(self, 5)
    }
    #[doc = "Bits 6:28 - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<CtlSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 29 - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_cg_en(&mut self) -> DynCgEnW<CtlSpec> {
        DynCgEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Set this bit to enable statistic counters."]
    #[inline(always)]
    #[must_use]
    pub fn stats_en(&mut self) -> StatsEnW<CtlSpec> {
        StatsEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Set this bit to clear statistic counters."]
    #[inline(always)]
    #[must_use]
    pub fn stats_clr(&mut self) -> StatsClrW<CtlSpec> {
        StatsClrW::new(self, 31)
    }
}
#[doc = "Control Configure VIMS mode and line buffer settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
