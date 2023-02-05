#[doc = "Register `RESETCTL` reader"]
pub struct R(crate::R<RESETCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETCTL` writer"]
pub struct W(crate::W<RESETCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RESETCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `RESET_SRC` reader - 3:1\\]
Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
pub type RESET_SRC_R = crate::FieldReader<u8, RESET_SRC_A>;
#[doc = "3:1\\]
Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESET_SRC_A {
    #[doc = "7: Software reset via PRCM warm reset request"]
    WARMRESET = 7,
    #[doc = "6: Software reset via SYSRESET register"]
    SYSRESET = 6,
    #[doc = "5: Clock loss detect"]
    CLK_LOSS = 5,
    #[doc = "4: Brown out detect on VDDR"]
    VDDR_LOSS = 4,
    #[doc = "3: Brown out detect on VDD"]
    VDD_LOSS = 3,
    #[doc = "2: Brown out detect on VDDS"]
    VDDS_LOSS = 2,
    #[doc = "1: Reset pin"]
    PIN_RESET = 1,
    #[doc = "0: Power on reset"]
    PWR_ON = 0,
}
impl From<RESET_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RESET_SRC_A) -> Self {
        variant as _
    }
}
impl RESET_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_SRC_A {
        match self.bits {
            7 => RESET_SRC_A::WARMRESET,
            6 => RESET_SRC_A::SYSRESET,
            5 => RESET_SRC_A::CLK_LOSS,
            4 => RESET_SRC_A::VDDR_LOSS,
            3 => RESET_SRC_A::VDD_LOSS,
            2 => RESET_SRC_A::VDDS_LOSS,
            1 => RESET_SRC_A::PIN_RESET,
            0 => RESET_SRC_A::PWR_ON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WARMRESET`"]
    #[inline(always)]
    pub fn is_warmreset(&self) -> bool {
        *self == RESET_SRC_A::WARMRESET
    }
    #[doc = "Checks if the value of the field is `SYSRESET`"]
    #[inline(always)]
    pub fn is_sysreset(&self) -> bool {
        *self == RESET_SRC_A::SYSRESET
    }
    #[doc = "Checks if the value of the field is `CLK_LOSS`"]
    #[inline(always)]
    pub fn is_clk_loss(&self) -> bool {
        *self == RESET_SRC_A::CLK_LOSS
    }
    #[doc = "Checks if the value of the field is `VDDR_LOSS`"]
    #[inline(always)]
    pub fn is_vddr_loss(&self) -> bool {
        *self == RESET_SRC_A::VDDR_LOSS
    }
    #[doc = "Checks if the value of the field is `VDD_LOSS`"]
    #[inline(always)]
    pub fn is_vdd_loss(&self) -> bool {
        *self == RESET_SRC_A::VDD_LOSS
    }
    #[doc = "Checks if the value of the field is `VDDS_LOSS`"]
    #[inline(always)]
    pub fn is_vdds_loss(&self) -> bool {
        *self == RESET_SRC_A::VDDS_LOSS
    }
    #[doc = "Checks if the value of the field is `PIN_RESET`"]
    #[inline(always)]
    pub fn is_pin_reset(&self) -> bool {
        *self == RESET_SRC_A::PIN_RESET
    }
    #[doc = "Checks if the value of the field is `PWR_ON`"]
    #[inline(always)]
    pub fn is_pwr_on(&self) -> bool {
        *self == RESET_SRC_A::PWR_ON
    }
}
#[doc = "Field `RESET_SRC` writer - 3:1\\]
Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
pub type RESET_SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RESETCTL_SPEC, u8, RESET_SRC_A, 3, O>;
impl<'a, const O: u8> RESET_SRC_W<'a, O> {
    #[doc = "Software reset via PRCM warm reset request"]
    #[inline(always)]
    pub fn warmreset(self) -> &'a mut W {
        self.variant(RESET_SRC_A::WARMRESET)
    }
    #[doc = "Software reset via SYSRESET register"]
    #[inline(always)]
    pub fn sysreset(self) -> &'a mut W {
        self.variant(RESET_SRC_A::SYSRESET)
    }
    #[doc = "Clock loss detect"]
    #[inline(always)]
    pub fn clk_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::CLK_LOSS)
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline(always)]
    pub fn vddr_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::VDDR_LOSS)
    }
    #[doc = "Brown out detect on VDD"]
    #[inline(always)]
    pub fn vdd_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::VDD_LOSS)
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline(always)]
    pub fn vdds_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::VDDS_LOSS)
    }
    #[doc = "Reset pin"]
    #[inline(always)]
    pub fn pin_reset(self) -> &'a mut W {
        self.variant(RESET_SRC_A::PIN_RESET)
    }
    #[doc = "Power on reset"]
    #[inline(always)]
    pub fn pwr_on(self) -> &'a mut W {
        self.variant(RESET_SRC_A::PWR_ON)
    }
}
#[doc = "Field `CLK_LOSS_EN` reader - 4:4\\]
Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset"]
pub type CLK_LOSS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_LOSS_EN` writer - 4:4\\]
Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset"]
pub type CLK_LOSS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `VDD_LOSS_EN` reader - 5:5\\]
Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
pub type VDD_LOSS_EN_R = crate::BitReader<bool>;
#[doc = "Field `VDD_LOSS_EN` writer - 5:5\\]
Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
pub type VDD_LOSS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `VDDR_LOSS_EN` reader - 6:6\\]
Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
pub type VDDR_LOSS_EN_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_LOSS_EN` writer - 6:6\\]
Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
pub type VDDR_LOSS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `VDDS_LOSS_EN` reader - 7:7\\]
Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
pub type VDDS_LOSS_EN_R = crate::BitReader<bool>;
#[doc = "Field `VDDS_LOSS_EN` writer - 7:7\\]
Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
pub type VDDS_LOSS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 8:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED8` writer - 8:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `VDD_LOSS_EN_OVR` reader - 9:9\\]
Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked"]
pub type VDD_LOSS_EN_OVR_R = crate::BitReader<bool>;
#[doc = "Field `VDD_LOSS_EN_OVR` writer - 9:9\\]
Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked"]
pub type VDD_LOSS_EN_OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `VDDR_LOSS_EN_OVR` reader - 10:10\\]
Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked"]
pub type VDDR_LOSS_EN_OVR_R = crate::BitReader<bool>;
#[doc = "Field `VDDR_LOSS_EN_OVR` writer - 10:10\\]
Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked"]
pub type VDDR_LOSS_EN_OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `VDDS_LOSS_EN_OVR` reader - 11:11\\]
Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked"]
pub type VDDS_LOSS_EN_OVR_R = crate::BitReader<bool>;
#[doc = "Field `VDDS_LOSS_EN_OVR` writer - 11:11\\]
Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked"]
pub type VDDS_LOSS_EN_OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `BOOT_DET_0` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_0_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DET_0` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `BOOT_DET_1` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_1_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DET_1` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `GPIO_WU_FROM_SD` reader - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
pub type GPIO_WU_FROM_SD_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_WU_FROM_SD` writer - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
pub type GPIO_WU_FROM_SD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `WU_FROM_SD` reader - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
pub type WU_FROM_SD_R = crate::BitReader<bool>;
#[doc = "Field `WU_FROM_SD` writer - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
pub type WU_FROM_SD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `BOOT_DET_0_SET` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_0_SET_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DET_0_SET` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_0_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `BOOT_DET_1_SET` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_1_SET_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DET_1_SET` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_1_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED18` writer - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETCTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `BOOT_DET_0_CLR` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_0_CLR_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DET_0_CLR` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_0_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `BOOT_DET_1_CLR` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_1_CLR_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_DET_1_CLR` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type BOOT_DET_1_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED26` reader - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED26` writer - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETCTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `SYSRESET` reader - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
pub type SYSRESET_R = crate::BitReader<bool>;
#[doc = "Field `SYSRESET` writer - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
pub type SYSRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
    #[inline(always)]
    pub fn reset_src(&self) -> RESET_SRC_R {
        RESET_SRC_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset"]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
    #[inline(always)]
    pub fn vdd_loss_en(&self) -> VDD_LOSS_EN_R {
        VDD_LOSS_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
    #[inline(always)]
    pub fn vddr_loss_en(&self) -> VDDR_LOSS_EN_R {
        VDDR_LOSS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
    #[inline(always)]
    pub fn vdds_loss_en(&self) -> VDDS_LOSS_EN_R {
        VDDS_LOSS_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked"]
    #[inline(always)]
    pub fn vdd_loss_en_ovr(&self) -> VDD_LOSS_EN_OVR_R {
        VDD_LOSS_EN_OVR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked"]
    #[inline(always)]
    pub fn vddr_loss_en_ovr(&self) -> VDDR_LOSS_EN_OVR_R {
        VDDR_LOSS_EN_OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked"]
    #[inline(always)]
    pub fn vdds_loss_en_ovr(&self) -> VDDS_LOSS_EN_OVR_R {
        VDDS_LOSS_EN_OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0(&self) -> BOOT_DET_0_R {
        BOOT_DET_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1(&self) -> BOOT_DET_1_R {
        BOOT_DET_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline(always)]
    pub fn gpio_wu_from_sd(&self) -> GPIO_WU_FROM_SD_R {
        GPIO_WU_FROM_SD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline(always)]
    pub fn wu_from_sd(&self) -> WU_FROM_SD_R {
        WU_FROM_SD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_set(&self) -> BOOT_DET_0_SET_R {
        BOOT_DET_0_SET_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_set(&self) -> BOOT_DET_1_SET_R {
        BOOT_DET_1_SET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_clr(&self) -> BOOT_DET_0_CLR_R {
        BOOT_DET_0_CLR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_clr(&self) -> BOOT_DET_1_CLR_R {
        BOOT_DET_1_CLR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
    #[inline(always)]
    pub fn sysreset(&self) -> SYSRESET_R {
        SYSRESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
    #[inline(always)]
    #[must_use]
    pub fn reset_src(&mut self) -> RESET_SRC_W<1> {
        RESET_SRC_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset"]
    #[inline(always)]
    #[must_use]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W<4> {
        CLK_LOSS_EN_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_loss_en(&mut self) -> VDD_LOSS_EN_W<5> {
        VDD_LOSS_EN_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
    #[inline(always)]
    #[must_use]
    pub fn vddr_loss_en(&mut self) -> VDDR_LOSS_EN_W<6> {
        VDDR_LOSS_EN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdds_loss_en(&mut self) -> VDDS_LOSS_EN_W<7> {
        VDDS_LOSS_EN_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_loss_en_ovr(&mut self) -> VDD_LOSS_EN_OVR_W<9> {
        VDD_LOSS_EN_OVR_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked"]
    #[inline(always)]
    #[must_use]
    pub fn vddr_loss_en_ovr(&mut self) -> VDDR_LOSS_EN_OVR_W<10> {
        VDDR_LOSS_EN_OVR_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked"]
    #[inline(always)]
    #[must_use]
    pub fn vdds_loss_en_ovr(&mut self) -> VDDS_LOSS_EN_OVR_W<11> {
        VDDS_LOSS_EN_OVR_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_0(&mut self) -> BOOT_DET_0_W<12> {
        BOOT_DET_0_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_1(&mut self) -> BOOT_DET_1_W<13> {
        BOOT_DET_1_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wu_from_sd(&mut self) -> GPIO_WU_FROM_SD_W<14> {
        GPIO_WU_FROM_SD_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\]
for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_from_sd(&mut self) -> WU_FROM_SD_W<15> {
        WU_FROM_SD_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_0_set(&mut self) -> BOOT_DET_0_SET_W<16> {
        BOOT_DET_0_SET_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_1_set(&mut self) -> BOOT_DET_1_SET_W<17> {
        BOOT_DET_1_SET_W::new(self)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_0_clr(&mut self) -> BOOT_DET_0_CLR_W<24> {
        BOOT_DET_0_CLR_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boot_det_1_clr(&mut self) -> BOOT_DET_1_CLR_W<25> {
        BOOT_DET_1_CLR_W::new(self)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> RESERVED26_W<26> {
        RESERVED26_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
    #[inline(always)]
    #[must_use]
    pub fn sysreset(&mut self) -> SYSRESET_W<31> {
        SYSRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctl](index.html) module"]
pub struct RESETCTL_SPEC;
impl crate::RegisterSpec for RESETCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetctl::R](R) reader structure"]
impl crate::Readable for RESETCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetctl::W](W) writer structure"]
impl crate::Writable for RESETCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETCTL to value 0xe0"]
impl crate::Resettable for RESETCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xe0;
}
