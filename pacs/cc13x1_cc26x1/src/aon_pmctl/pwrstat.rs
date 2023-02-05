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
#[doc = "Field `AUX_RESET_DONE` reader - 0:0\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub type AUX_RESET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_RESET_DONE` writer - 0:0\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub type AUX_RESET_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `AUX_BUS_RESET_DONE` reader - 1:1\\]
Indicates Reset Done from AUX Bus: 0: AUX Bus is being reset 1: AUX Bus reset is released"]
pub type AUX_BUS_RESET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_BUS_RESET_DONE` writer - 1:1\\]
Indicates Reset Done from AUX Bus: 0: AUX Bus is being reset 1: AUX Bus reset is released"]
pub type AUX_BUS_RESET_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `JTAG_PD_ON` reader - 2:2\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub type JTAG_PD_ON_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_PD_ON` writer - 2:2\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub type JTAG_PD_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRSTAT_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    pub fn aux_reset_done(&self) -> AUX_RESET_DONE_R {
        AUX_RESET_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX Bus: 0: AUX Bus is being reset 1: AUX Bus reset is released"]
    #[inline(always)]
    pub fn aux_bus_reset_done(&self) -> AUX_BUS_RESET_DONE_R {
        AUX_BUS_RESET_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    pub fn jtag_pd_on(&self) -> JTAG_PD_ON_R {
        JTAG_PD_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    #[must_use]
    pub fn aux_reset_done(&mut self) -> AUX_RESET_DONE_W<0> {
        AUX_RESET_DONE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX Bus: 0: AUX Bus is being reset 1: AUX Bus reset is released"]
    #[inline(always)]
    #[must_use]
    pub fn aux_bus_reset_done(&mut self) -> AUX_BUS_RESET_DONE_W<1> {
        AUX_BUS_RESET_DONE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_pd_on(&mut self) -> JTAG_PD_ON_W<2> {
        JTAG_PD_ON_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](index.html) module"]
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
#[doc = "`reset()` method sets PWRSTAT to value 0x03c0_0003"]
impl crate::Resettable for PWRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x03c0_0003;
}
