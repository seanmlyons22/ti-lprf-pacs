#[doc = "Register `PWMCLKEN` reader"]
pub struct R(crate::R<PWMCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMCLKEN` writer"]
pub struct W(crate::W<PWMCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCLKEN_SPEC>;
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
impl From<crate::W<PWMCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC` reader - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
pub type RFC_R = crate::BitReader<bool>;
#[doc = "Field `RFC` writer - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
pub type RFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `CPE` reader - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
pub type CPE_R = crate::BitReader<bool>;
#[doc = "Field `CPE` writer - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
pub type CPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `CPERAM` reader - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
pub type CPERAM_R = crate::BitReader<bool>;
#[doc = "Field `CPERAM` writer - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
pub type CPERAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `MDM` reader - 3:3\\]
Enable clock to the Modem (MDM) module."]
pub type MDM_R = crate::BitReader<bool>;
#[doc = "Field `MDM` writer - 3:3\\]
Enable clock to the Modem (MDM) module."]
pub type MDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `MDMRAM` reader - 4:4\\]
Enable clock to the Modem RAM module."]
pub type MDMRAM_R = crate::BitReader<bool>;
#[doc = "Field `MDMRAM` writer - 4:4\\]
Enable clock to the Modem RAM module."]
pub type MDMRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `RFE` reader - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
pub type RFE_R = crate::BitReader<bool>;
#[doc = "Field `RFE` writer - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
pub type RFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `RFERAM` reader - 6:6\\]
Enable clock to the RF Engine RAM module."]
pub type RFERAM_R = crate::BitReader<bool>;
#[doc = "Field `RFERAM` writer - 6:6\\]
Enable clock to the RF Engine RAM module."]
pub type RFERAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `RAT` reader - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
pub type RAT_R = crate::BitReader<bool>;
#[doc = "Field `RAT` writer - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
pub type RAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `PHA` reader - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
pub type PHA_R = crate::BitReader<bool>;
#[doc = "Field `PHA` writer - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
pub type PHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `FSCA` reader - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
pub type FSCA_R = crate::BitReader<bool>;
#[doc = "Field `FSCA` writer - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
pub type FSCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `RFCTRC` reader - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
pub type RFCTRC_R = crate::BitReader<bool>;
#[doc = "Field `RFCTRC` writer - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
pub type RFCTRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `IQRAM` reader - 11:11\\]
Enable clock to IQ RAM in coherent demodulator"]
pub type IQRAM_R = crate::BitReader<bool>;
#[doc = "Field `IQRAM` writer - 11:11\\]
Enable clock to IQ RAM in coherent demodulator"]
pub type IQRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `MOD` reader - 12:12\\]
Enable clock to the Modulator"]
pub type MOD_R = crate::BitReader<bool>;
#[doc = "Field `MOD` writer - 12:12\\]
Enable clock to the Modulator"]
pub type MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `DEMOD` reader - 13:13\\]
Enable clock to the Demodulator"]
pub type DEMOD_R = crate::BitReader<bool>;
#[doc = "Field `DEMOD` writer - 13:13\\]
Enable clock to the Demodulator"]
pub type DEMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCLKEN_SPEC, bool, O>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWMCLKEN_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cperam(&self) -> CPERAM_R {
        CPERAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub fn mdm(&self) -> MDM_R {
        MDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub fn mdmram(&self) -> MDMRAM_R {
        MDMRAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub fn rferam(&self) -> RFERAM_R {
        RFERAM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub fn rat(&self) -> RAT_R {
        RAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub fn fsca(&self) -> FSCA_R {
        FSCA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub fn rfctrc(&self) -> RFCTRC_R {
        RFCTRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable clock to IQ RAM in coherent demodulator"]
    #[inline(always)]
    pub fn iqram(&self) -> IQRAM_R {
        IQRAM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable clock to the Modulator"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable clock to the Demodulator"]
    #[inline(always)]
    pub fn demod(&self) -> DEMOD_R {
        DEMOD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    #[must_use]
    pub fn rfc(&mut self) -> RFC_W<0> {
        RFC_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    #[must_use]
    pub fn cpe(&mut self) -> CPE_W<1> {
        CPE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    #[must_use]
    pub fn cperam(&mut self) -> CPERAM_W<2> {
        CPERAM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    #[must_use]
    pub fn mdm(&mut self) -> MDM_W<3> {
        MDM_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    #[must_use]
    pub fn mdmram(&mut self) -> MDMRAM_W<4> {
        MDMRAM_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<5> {
        RFE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    #[must_use]
    pub fn rferam(&mut self) -> RFERAM_W<6> {
        RFERAM_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    #[must_use]
    pub fn rat(&mut self) -> RAT_W<7> {
        RAT_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    #[must_use]
    pub fn pha(&mut self) -> PHA_W<8> {
        PHA_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    #[must_use]
    pub fn fsca(&mut self) -> FSCA_W<9> {
        FSCA_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    #[must_use]
    pub fn rfctrc(&mut self) -> RFCTRC_W<10> {
        RFCTRC_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable clock to IQ RAM in coherent demodulator"]
    #[inline(always)]
    #[must_use]
    pub fn iqram(&mut self) -> IQRAM_W<11> {
        IQRAM_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable clock to the Modulator"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<12> {
        MOD_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable clock to the Demodulator"]
    #[inline(always)]
    #[must_use]
    pub fn demod(&mut self) -> DEMOD_W<13> {
        DEMOD_W::new(self)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF Core Power Management and Clock Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmclken](index.html) module"]
pub struct PWMCLKEN_SPEC;
impl crate::RegisterSpec for PWMCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmclken::R](R) reader structure"]
impl crate::Readable for PWMCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmclken::W](W) writer structure"]
impl crate::Writable for PWMCLKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWMCLKEN to value 0x01"]
impl crate::Resettable for PWMCLKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
