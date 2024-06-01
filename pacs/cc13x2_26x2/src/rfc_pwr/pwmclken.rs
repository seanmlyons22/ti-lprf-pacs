#[doc = "Register `PWMCLKEN` reader"]
pub type R = crate::R<PwmclkenSpec>;
#[doc = "Register `PWMCLKEN` writer"]
pub type W = crate::W<PwmclkenSpec>;
#[doc = "Field `RFC` reader - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
pub type RfcR = crate::BitReader;
#[doc = "Field `RFC` writer - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
pub type RfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPE` reader - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
pub type CpeR = crate::BitReader;
#[doc = "Field `CPE` writer - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
pub type CpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPERAM` reader - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
pub type CperamR = crate::BitReader;
#[doc = "Field `CPERAM` writer - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
pub type CperamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDM` reader - 3:3\\]
Enable clock to the Modem (MDM) module."]
pub type MdmR = crate::BitReader;
#[doc = "Field `MDM` writer - 3:3\\]
Enable clock to the Modem (MDM) module."]
pub type MdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMRAM` reader - 4:4\\]
Enable clock to the Modem RAM module."]
pub type MdmramR = crate::BitReader;
#[doc = "Field `MDMRAM` writer - 4:4\\]
Enable clock to the Modem RAM module."]
pub type MdmramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFE` reader - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
pub type RfeR = crate::BitReader;
#[doc = "Field `RFE` writer - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFERAM` reader - 6:6\\]
Enable clock to the RF Engine RAM module."]
pub type RferamR = crate::BitReader;
#[doc = "Field `RFERAM` writer - 6:6\\]
Enable clock to the RF Engine RAM module."]
pub type RferamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAT` reader - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
pub type RatR = crate::BitReader;
#[doc = "Field `RAT` writer - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
pub type RatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHA` reader - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
pub type PhaR = crate::BitReader;
#[doc = "Field `PHA` writer - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
pub type PhaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSCA` reader - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
pub type FscaR = crate::BitReader;
#[doc = "Field `FSCA` writer - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
pub type FscaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFCTRC` reader - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
pub type RfctrcR = crate::BitReader;
#[doc = "Field `RFCTRC` writer - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
pub type RfctrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub fn rfc(&self) -> RfcR {
        RfcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cpe(&self) -> CpeR {
        CpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cperam(&self) -> CperamR {
        CperamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub fn mdm(&self) -> MdmR {
        MdmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub fn mdmram(&self) -> MdmramR {
        MdmramR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub fn rferam(&self) -> RferamR {
        RferamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub fn rat(&self) -> RatR {
        RatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub fn pha(&self) -> PhaR {
        PhaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub fn fsca(&self) -> FscaR {
        FscaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub fn rfctrc(&self) -> RfctrcR {
        RfctrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    #[must_use]
    pub fn rfc(&mut self) -> RfcW<PwmclkenSpec> {
        RfcW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    #[must_use]
    pub fn cpe(&mut self) -> CpeW<PwmclkenSpec> {
        CpeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    #[must_use]
    pub fn cperam(&mut self) -> CperamW<PwmclkenSpec> {
        CperamW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    #[must_use]
    pub fn mdm(&mut self) -> MdmW<PwmclkenSpec> {
        MdmW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    #[must_use]
    pub fn mdmram(&mut self) -> MdmramW<PwmclkenSpec> {
        MdmramW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<PwmclkenSpec> {
        RfeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    #[must_use]
    pub fn rferam(&mut self) -> RferamW<PwmclkenSpec> {
        RferamW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    #[must_use]
    pub fn rat(&mut self) -> RatW<PwmclkenSpec> {
        RatW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    #[must_use]
    pub fn pha(&mut self) -> PhaW<PwmclkenSpec> {
        PhaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    #[must_use]
    pub fn fsca(&mut self) -> FscaW<PwmclkenSpec> {
        FscaW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    #[must_use]
    pub fn rfctrc(&mut self) -> RfctrcW<PwmclkenSpec> {
        RfctrcW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<PwmclkenSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "RF Core Power Management and Clock Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmclken::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmclken::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmclkenSpec;
impl crate::RegisterSpec for PwmclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmclken::R`](R) reader structure"]
impl crate::Readable for PwmclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmclken::W`](W) writer structure"]
impl crate::Writable for PwmclkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMCLKEN to value 0x01"]
impl crate::Resettable for PwmclkenSpec {
    const RESET_VALUE: u32 = 0x01;
}
