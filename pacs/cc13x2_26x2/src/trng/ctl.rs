#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_MODE` reader - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
pub type TestModeR = crate::BitReader;
#[doc = "Field `TEST_MODE` writer - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
pub type TestModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_LFSR_FB` reader - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
pub type NoLfsrFbR = crate::BitReader;
#[doc = "Field `NO_LFSR_FB` writer - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
pub type NoLfsrFbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TRNG_EN` reader - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
pub type TrngEnR = crate::BitReader;
#[doc = "Field `TRNG_EN` writer - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
pub type TrngEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `STARTUP_CYCLES` reader - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
pub type StartupCyclesR = crate::FieldReader<u16>;
#[doc = "Field `STARTUP_CYCLES` writer - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
pub type StartupCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline(always)]
    pub fn test_mode(&self) -> TestModeR {
        TestModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
    #[inline(always)]
    pub fn no_lfsr_fb(&self) -> NoLfsrFbR {
        NoLfsrFbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline(always)]
    pub fn trng_en(&self) -> TrngEnR {
        TrngEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline(always)]
    pub fn startup_cycles(&self) -> StartupCyclesR {
        StartupCyclesR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<CtlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline(always)]
    #[must_use]
    pub fn test_mode(&mut self) -> TestModeW<CtlSpec> {
        TestModeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
    #[inline(always)]
    #[must_use]
    pub fn no_lfsr_fb(&mut self) -> NoLfsrFbW<CtlSpec> {
        NoLfsrFbW::new(self, 2)
    }
    #[doc = "Bits 3:9 - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CtlSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline(always)]
    #[must_use]
    pub fn trng_en(&mut self) -> TrngEnW<CtlSpec> {
        TrngEnW::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<CtlSpec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn startup_cycles(&mut self) -> StartupCyclesW<CtlSpec> {
        StartupCyclesW::new(self, 16)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
