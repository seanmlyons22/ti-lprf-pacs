#[doc = "Register `SLEEPCTL` reader"]
pub type R = crate::R<SleepctlSpec>;
#[doc = "Register `SLEEPCTL` writer"]
pub type W = crate::W<SleepctlSpec>;
#[doc = "Field `IO_PAD_SLEEP_DIS` reader - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set). 0: I/O pad sleep mode is enabled, meaning all outputs and pad configurations are latched. Inputs are transparent if pad is configured as input before IO_PAD_SLEEP_DIS is set to 1 1: I/O pad sleep mode is disabled Application software must reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN to avoid glitches on pins."]
pub type IoPadSleepDisR = crate::BitReader;
#[doc = "Field `IO_PAD_SLEEP_DIS` writer - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set). 0: I/O pad sleep mode is enabled, meaning all outputs and pad configurations are latched. Inputs are transparent if pad is configured as input before IO_PAD_SLEEP_DIS is set to 1 1: I/O pad sleep mode is disabled Application software must reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN to avoid glitches on pins."]
pub type IoPadSleepDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set). 0: I/O pad sleep mode is enabled, meaning all outputs and pad configurations are latched. Inputs are transparent if pad is configured as input before IO_PAD_SLEEP_DIS is set to 1 1: I/O pad sleep mode is disabled Application software must reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN to avoid glitches on pins."]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&self) -> IoPadSleepDisR {
        IoPadSleepDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set). 0: I/O pad sleep mode is enabled, meaning all outputs and pad configurations are latched. Inputs are transparent if pad is configured as input before IO_PAD_SLEEP_DIS is set to 1 1: I/O pad sleep mode is disabled Application software must reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN to avoid glitches on pins."]
    #[inline(always)]
    #[must_use]
    pub fn io_pad_sleep_dis(&mut self) -> IoPadSleepDisW<SleepctlSpec> {
        IoPadSleepDisW::new(self, 0)
    }
}
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepctlSpec;
impl crate::RegisterSpec for SleepctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepctl::R`](R) reader structure"]
impl crate::Readable for SleepctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sleepctl::W`](W) writer structure"]
impl crate::Writable for SleepctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEPCTL to value 0"]
impl crate::Resettable for SleepctlSpec {
    const RESET_VALUE: u32 = 0;
}
