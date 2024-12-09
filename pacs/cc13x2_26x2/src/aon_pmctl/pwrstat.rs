#[doc = "Register `PWRSTAT` reader"]
pub type R = crate::R<PwrstatSpec>;
#[doc = "Register `PWRSTAT` writer"]
pub type W = crate::W<PwrstatSpec>;
#[doc = "Field `AUX_RESET_DONE` reader - 0:0\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub type AuxResetDoneR = crate::BitReader;
#[doc = "Field `AUX_BUS_RESET_DONE` reader - 1:1\\]
Indicates Reset Done from AUX Bus: 0: AUX Bus is being reset 1: AUX Bus reset is released"]
pub type AuxBusResetDoneR = crate::BitReader;
#[doc = "Field `JTAG_PD_ON` reader - 2:2\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub type JtagPdOnR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    pub fn aux_reset_done(&self) -> AuxResetDoneR {
        AuxResetDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX Bus: 0: AUX Bus is being reset 1: AUX Bus reset is released"]
    #[inline(always)]
    pub fn aux_bus_reset_done(&self) -> AuxBusResetDoneR {
        AuxBusResetDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    pub fn jtag_pd_on(&self) -> JtagPdOnR {
        JtagPdOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<PwrstatSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PWRSTAT to value 0x03c0_0003"]
impl crate::Resettable for PwrstatSpec {
    const RESET_VALUE: u32 = 0x03c0_0003;
}
