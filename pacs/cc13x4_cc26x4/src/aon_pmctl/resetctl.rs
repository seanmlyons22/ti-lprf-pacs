#[doc = "Register `RESETCTL` reader"]
pub type R = crate::R<ResetctlSpec>;
#[doc = "Register `RESETCTL` writer"]
pub type W = crate::W<ResetctlSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetSrc {
    #[doc = "7: Software reset via PRCM warm reset request"]
    Warmreset = 7,
    #[doc = "6: Software reset via SYSRESET or hardware power management timeout detection. Note: The hardware power management timeout circuit is always enabled."]
    Sysreset = 6,
    #[doc = "5: SCLK_LF, SCLK_MF or SCLK_HF clock loss detect"]
    ClkLoss = 5,
    #[doc = "4: Brown out detect on VDDR"]
    VddrLoss = 4,
    #[doc = "2: Brown out detect on VDDS"]
    VddsLoss = 2,
    #[doc = "1: Reset pin"]
    PinReset = 1,
    #[doc = "0: Power on reset"]
    PwrOn = 0,
}
impl From<ResetSrc> for u8 {
    #[inline(always)]
    fn from(variant: ResetSrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ResetSrc {
    type Ux = u8;
}
impl crate::IsEnum for ResetSrc {}
#[doc = "Field `RESET_SRC` reader - 3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause."]
pub type ResetSrcR = crate::FieldReader<ResetSrc>;
impl ResetSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ResetSrc> {
        match self.bits {
            7 => Some(ResetSrc::Warmreset),
            6 => Some(ResetSrc::Sysreset),
            5 => Some(ResetSrc::ClkLoss),
            4 => Some(ResetSrc::VddrLoss),
            2 => Some(ResetSrc::VddsLoss),
            1 => Some(ResetSrc::PinReset),
            0 => Some(ResetSrc::PwrOn),
            _ => None,
        }
    }
    #[doc = "Software reset via PRCM warm reset request"]
    #[inline(always)]
    pub fn is_warmreset(&self) -> bool {
        *self == ResetSrc::Warmreset
    }
    #[doc = "Software reset via SYSRESET or hardware power management timeout detection. Note: The hardware power management timeout circuit is always enabled."]
    #[inline(always)]
    pub fn is_sysreset(&self) -> bool {
        *self == ResetSrc::Sysreset
    }
    #[doc = "SCLK_LF, SCLK_MF or SCLK_HF clock loss detect"]
    #[inline(always)]
    pub fn is_clk_loss(&self) -> bool {
        *self == ResetSrc::ClkLoss
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline(always)]
    pub fn is_vddr_loss(&self) -> bool {
        *self == ResetSrc::VddrLoss
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline(always)]
    pub fn is_vdds_loss(&self) -> bool {
        *self == ResetSrc::VddsLoss
    }
    #[doc = "Reset pin"]
    #[inline(always)]
    pub fn is_pin_reset(&self) -> bool {
        *self == ResetSrc::PinReset
    }
    #[doc = "Power on reset"]
    #[inline(always)]
    pub fn is_pwr_on(&self) -> bool {
        *self == ResetSrc::PwrOn
    }
}
#[doc = "Field `RESET_SRC` writer - 3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause."]
pub type ResetSrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, ResetSrc>;
impl<'a, REG> ResetSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software reset via PRCM warm reset request"]
    #[inline(always)]
    pub fn warmreset(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::Warmreset)
    }
    #[doc = "Software reset via SYSRESET or hardware power management timeout detection. Note: The hardware power management timeout circuit is always enabled."]
    #[inline(always)]
    pub fn sysreset(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::Sysreset)
    }
    #[doc = "SCLK_LF, SCLK_MF or SCLK_HF clock loss detect"]
    #[inline(always)]
    pub fn clk_loss(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::ClkLoss)
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline(always)]
    pub fn vddr_loss(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::VddrLoss)
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline(always)]
    pub fn vdds_loss(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::VddsLoss)
    }
    #[doc = "Reset pin"]
    #[inline(always)]
    pub fn pin_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::PinReset)
    }
    #[doc = "Power on reset"]
    #[inline(always)]
    pub fn pwr_on(self) -> &'a mut crate::W<REG> {
        self.variant(ResetSrc::PwrOn)
    }
}
#[doc = "Field `MCU_WARM_RESET` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type McuWarmResetR = crate::BitReader;
#[doc = "Field `MCU_WARM_RESET` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type McuWarmResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_LOSS_EN` reader - 5:5\\]
Controls reset generation in case SCLK_LF, SCLK_MF or SCLK_HF is lost when clock loss detection is enabled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]
0: Clock loss is ignored 1: Clock loss generates system reset Note: Clock loss reset generation must be disabled when changing clock source for SCLK_LF. Failure to do so may result in a spurious system reset. Clock loss reset generation is controlled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]"]
pub type ClkLossEnR = crate::BitReader;
#[doc = "Field `CLK_LOSS_EN` writer - 5:5\\]
Controls reset generation in case SCLK_LF, SCLK_MF or SCLK_HF is lost when clock loss detection is enabled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]
0: Clock loss is ignored 1: Clock loss generates system reset Note: Clock loss reset generation must be disabled when changing clock source for SCLK_LF. Failure to do so may result in a spurious system reset. Clock loss reset generation is controlled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]"]
pub type ClkLossEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 11:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 11:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BOOT_DET_0` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type BootDet0R = crate::BitReader;
#[doc = "Field `BOOT_DET_0` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type BootDet0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_DET_1` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type BootDet1R = crate::BitReader;
#[doc = "Field `BOOT_DET_1` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type BootDet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_WU_FROM_SD` reader - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
pub type GpioWuFromSdR = crate::BitReader;
#[doc = "Field `GPIO_WU_FROM_SD` writer - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
pub type GpioWuFromSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_FROM_SD` reader - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
pub type WuFromSdR = crate::BitReader;
#[doc = "Field `WU_FROM_SD` writer - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
pub type WuFromSdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_DET_0_SET` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BootDet0SetR = crate::BitReader;
#[doc = "Field `BOOT_DET_0_SET` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BootDet0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_DET_1_SET` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type BootDet1SetR = crate::BitReader;
#[doc = "Field `BOOT_DET_1_SET` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type BootDet1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader;
#[doc = "Field `RESERVED18` writer - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BOOT_DET_0_CLR` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type BootDet0ClrR = crate::BitReader;
#[doc = "Field `BOOT_DET_0_CLR` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type BootDet0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_DET_1_CLR` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type BootDet1ClrR = crate::BitReader;
#[doc = "Field `BOOT_DET_1_CLR` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type BootDet1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED26` reader - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYSRESET` reader - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC"]
pub type SysresetR = crate::BitReader;
#[doc = "Field `SYSRESET` writer - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC"]
pub type SysresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause."]
    #[inline(always)]
    pub fn reset_src(&self) -> ResetSrcR {
        ResetSrcR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mcu_warm_reset(&self) -> McuWarmResetR {
        McuWarmResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls reset generation in case SCLK_LF, SCLK_MF or SCLK_HF is lost when clock loss detection is enabled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]
0: Clock loss is ignored 1: Clock loss generates system reset Note: Clock loss reset generation must be disabled when changing clock source for SCLK_LF. Failure to do so may result in a spurious system reset. Clock loss reset generation is controlled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]"]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> ClkLossEnR {
        ClkLossEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0(&self) -> BootDet0R {
        BootDet0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1(&self) -> BootDet1R {
        BootDet1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    pub fn gpio_wu_from_sd(&self) -> GpioWuFromSdR {
        GpioWuFromSdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    pub fn wu_from_sd(&self) -> WuFromSdR {
        WuFromSdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_set(&self) -> BootDet0SetR {
        BootDet0SetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_set(&self) -> BootDet1SetR {
        BootDet1SetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_clr(&self) -> BootDet0ClrR {
        BootDet0ClrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_clr(&self) -> BootDet1ClrR {
        BootDet1ClrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC"]
    #[inline(always)]
    pub fn sysreset(&self) -> SysresetR {
        SysresetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<ResetctlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause."]
    #[inline(always)]
    #[must_use]
    pub fn reset_src(&mut self) -> ResetSrcW<ResetctlSpec> {
        ResetSrcW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_warm_reset(&mut self) -> McuWarmResetW<ResetctlSpec> {
        McuWarmResetW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls reset generation in case SCLK_LF, SCLK_MF or SCLK_HF is lost when clock loss detection is enabled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]
0: Clock loss is ignored 1: Clock loss generates system reset Note: Clock loss reset generation must be disabled when changing clock source for SCLK_LF. Failure to do so may result in a spurious system reset. Clock loss reset generation is controlled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clk_loss_en(&mut self) -> ClkLossEnW<ResetctlSpec> {
        ClkLossEnW::new(self, 5)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<ResetctlSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_0(&mut self) -> BootDet0W<ResetctlSpec> {
        BootDet0W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_1(&mut self) -> BootDet1W<ResetctlSpec> {
        BootDet1W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wu_from_sd(&mut self) -> GpioWuFromSdW<ResetctlSpec> {
        GpioWuFromSdW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wu_from_sd(&mut self) -> WuFromSdW<ResetctlSpec> {
        WuFromSdW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_0_set(&mut self) -> BootDet0SetW<ResetctlSpec> {
        BootDet0SetW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_1_set(&mut self) -> BootDet1SetW<ResetctlSpec> {
        BootDet1SetW::new(self, 17)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<ResetctlSpec> {
        Reserved18W::new(self, 18)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_0_clr(&mut self) -> BootDet0ClrW<ResetctlSpec> {
        BootDet0ClrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_1_clr(&mut self) -> BootDet1ClrW<ResetctlSpec> {
        BootDet1ClrW::new(self, 25)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<ResetctlSpec> {
        Reserved26W::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC"]
    #[inline(always)]
    #[must_use]
    pub fn sysreset(&mut self) -> SysresetW<ResetctlSpec> {
        SysresetW::new(self, 31)
    }
}
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetctlSpec;
impl crate::RegisterSpec for ResetctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetctl::R`](R) reader structure"]
impl crate::Readable for ResetctlSpec {}
#[doc = "`write(|w| ..)` method takes [`resetctl::W`](W) writer structure"]
impl crate::Writable for ResetctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETCTL to value 0x01c0"]
impl crate::Resettable for ResetctlSpec {
    const RESET_VALUE: u32 = 0x01c0;
}
