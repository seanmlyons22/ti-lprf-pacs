#[doc = "Register `PWRSTAT` reader"]
pub type R = crate::R<PwrstatSpec>;
#[doc = "Register `PWRSTAT` writer"]
pub type W = crate::W<PwrstatSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `AUX_RESET_DONE` reader - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub type AuxResetDoneR = crate::BitReader;
#[doc = "Field `AUX_BUS_CONNECTED` reader - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
pub type AuxBusConnectedR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `MCU_PD_ON` reader - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
pub type McuPdOnR = crate::BitReader;
#[doc = "Field `AUX_PD_ON` reader - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
pub type AuxPdOnR = crate::BitReader;
#[doc = "Field `JTAG_PD_ON` reader - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub type JtagPdOnR = crate::BitReader;
#[doc = "Field `RESERVED7` reader - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `AUX_PWR_DWN` reader - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
pub type AuxPwrDwnR = crate::BitReader;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    pub fn aux_reset_done(&self) -> AuxResetDoneR {
        AuxResetDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
    #[inline(always)]
    pub fn aux_bus_connected(&self) -> AuxBusConnectedR {
        AuxBusConnectedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
    #[inline(always)]
    pub fn mcu_pd_on(&self) -> McuPdOnR {
        McuPdOnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
    #[inline(always)]
    pub fn aux_pd_on(&self) -> AuxPdOnR {
        AuxPdOnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    pub fn jtag_pd_on(&self) -> JtagPdOnR {
        JtagPdOnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
    #[inline(always)]
    pub fn aux_pwr_dwn(&self) -> AuxPwrDwnR {
        AuxPwrDwnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<PwrstatSpec> {
        Reserved10W::new(self, 10)
    }
}
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrstatSpec;
impl crate::RegisterSpec for PwrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrstat::R`](R) reader structure"]
impl crate::Readable for PwrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrstat::W`](W) writer structure"]
impl crate::Writable for PwrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRSTAT to value 0x0380_0000"]
impl crate::Resettable for PwrstatSpec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
