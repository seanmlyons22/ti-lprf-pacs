#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "1:0\\]
Current VIMS mode\n\nValue on reset: 0"]
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
Current VIMS mode"]
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
Current VIMS mode"]
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
#[doc = "Field `INV` reader - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE_CHANGING` reader - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
pub type ModeChangingR = crate::BitReader;
#[doc = "Field `MODE_CHANGING` writer - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
pub type ModeChangingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBUS_LB_DIS` reader - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type SysbusLbDisR = crate::BitReader;
#[doc = "Field `SYSBUS_LB_DIS` writer - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type SysbusLbDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDCODE_LB_DIS` reader - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type IdcodeLbDisR = crate::BitReader;
#[doc = "Field `IDCODE_LB_DIS` writer - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
pub type IdcodeLbDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Current VIMS mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
    #[inline(always)]
    pub fn mode_changing(&self) -> ModeChangingR {
        ModeChangingR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SysbusLbDisR {
        SysbusLbDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IdcodeLbDisR {
        IdcodeLbDisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Current VIMS mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<StatSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<StatSpec> {
        InvW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode_changing(&mut self) -> ModeChangingW<StatSpec> {
        ModeChangingW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    #[must_use]
    pub fn sysbus_lb_dis(&mut self) -> SysbusLbDisW<StatSpec> {
        SysbusLbDisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    #[must_use]
    pub fn idcode_lb_dis(&mut self) -> IdcodeLbDisW<StatSpec> {
        IdcodeLbDisW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<StatSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Status Displays current VIMS mode and line buffer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
