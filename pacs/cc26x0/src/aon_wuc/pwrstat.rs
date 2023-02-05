#[doc = "Register `PWRSTAT` reader"]
pub struct R(crate::R<PWRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRSTAT` writer"]
pub struct W(crate::W<PWRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRSTAT_SPEC>;
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
impl From<crate::W<PWRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `AUX_RESET_DONE` reader - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub type AUX_RESET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_RESET_DONE` writer - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub type AUX_RESET_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `AUX_BUS_CONNECTED` reader - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
pub type AUX_BUS_CONNECTED_R = crate::BitReader<bool>;
#[doc = "Field `AUX_BUS_CONNECTED` writer - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
pub type AUX_BUS_CONNECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `MCU_PD_ON` reader - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
pub type MCU_PD_ON_R = crate::BitReader<bool>;
#[doc = "Field `MCU_PD_ON` writer - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
pub type MCU_PD_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `AUX_PD_ON` reader - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
pub type AUX_PD_ON_R = crate::BitReader<bool>;
#[doc = "Field `AUX_PD_ON` writer - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
pub type AUX_PD_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `JTAG_PD_ON` reader - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub type JTAG_PD_ON_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_PD_ON` writer - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub type JTAG_PD_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED7` writer - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRSTAT_SPEC, u8, u8, 2, O>;
#[doc = "Field `AUX_PWR_DWN` reader - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
pub type AUX_PWR_DWN_R = crate::BitReader<bool>;
#[doc = "Field `AUX_PWR_DWN` writer - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
pub type AUX_PWR_DWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRSTAT_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    pub fn aux_reset_done(&self) -> AUX_RESET_DONE_R {
        AUX_RESET_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
    #[inline(always)]
    pub fn aux_bus_connected(&self) -> AUX_BUS_CONNECTED_R {
        AUX_BUS_CONNECTED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
    #[inline(always)]
    pub fn mcu_pd_on(&self) -> MCU_PD_ON_R {
        MCU_PD_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
    #[inline(always)]
    pub fn aux_pd_on(&self) -> AUX_PD_ON_R {
        AUX_PD_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    pub fn jtag_pd_on(&self) -> JTAG_PD_ON_R {
        JTAG_PD_ON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
    #[inline(always)]
    pub fn aux_pwr_dwn(&self) -> AUX_PWR_DWN_R {
        AUX_PWR_DWN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
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
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    #[must_use]
    pub fn aux_reset_done(&mut self) -> AUX_RESET_DONE_W<1> {
        AUX_RESET_DONE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
    #[inline(always)]
    #[must_use]
    pub fn aux_bus_connected(&mut self) -> AUX_BUS_CONNECTED_W<2> {
        AUX_BUS_CONNECTED_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pd_on(&mut self) -> MCU_PD_ON_W<4> {
        MCU_PD_ON_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
    #[inline(always)]
    #[must_use]
    pub fn aux_pd_on(&mut self) -> AUX_PD_ON_W<5> {
        AUX_PD_ON_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_pd_on(&mut self) -> JTAG_PD_ON_W<6> {
        JTAG_PD_ON_W::new(self)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
    #[inline(always)]
    #[must_use]
    pub fn aux_pwr_dwn(&mut self) -> AUX_PWR_DWN_W<9> {
        AUX_PWR_DWN_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](index.html) module"]
pub struct PWRSTAT_SPEC;
impl crate::RegisterSpec for PWRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrstat::R](R) reader structure"]
impl crate::Readable for PWRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrstat::W](W) writer structure"]
impl crate::Writable for PWRSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRSTAT to value 0x0380_0000"]
impl crate::Resettable for PWRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0380_0000;
}
