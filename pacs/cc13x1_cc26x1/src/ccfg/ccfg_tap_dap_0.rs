#[doc = "Register `CCFG_TAP_DAP_0` reader"]
pub struct R(crate::R<CCFG_TAP_DAP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_TAP_DAP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_TAP_DAP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_TAP_DAP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_TAP_DAP_0` writer"]
pub struct W(crate::W<CCFG_TAP_DAP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_TAP_DAP_0_SPEC>;
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
impl From<crate::W<CCFG_TAP_DAP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_TAP_DAP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_TAP_ENABLE` reader - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
pub type TEST_TAP_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_TAP_ENABLE` writer - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
pub type TEST_TAP_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCFG_TAP_DAP_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `PWRPROF_TAP_ENABLE` reader - 15:8\\]
Enable PWRPROF TAP. 0xC5: PWRPROF TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PWRPROF TAP access will remain disabled out of power-up/system-reset."]
pub type PWRPROF_TAP_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRPROF_TAP_ENABLE` writer - 15:8\\]
Enable PWRPROF TAP. 0xC5: PWRPROF TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PWRPROF TAP access will remain disabled out of power-up/system-reset."]
pub type PWRPROF_TAP_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCFG_TAP_DAP_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `CPU_DAP_ENABLE` reader - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
pub type CPU_DAP_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_DAP_ENABLE` writer - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
pub type CPU_DAP_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCFG_TAP_DAP_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn test_tap_enable(&self) -> TEST_TAP_ENABLE_R {
        TEST_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PWRPROF TAP. 0xC5: PWRPROF TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PWRPROF TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pwrprof_tap_enable(&self) -> PWRPROF_TAP_ENABLE_R {
        PWRPROF_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn cpu_dap_enable(&self) -> CPU_DAP_ENABLE_R {
        CPU_DAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn test_tap_enable(&mut self) -> TEST_TAP_ENABLE_W<0> {
        TEST_TAP_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PWRPROF TAP. 0xC5: PWRPROF TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PWRPROF TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn pwrprof_tap_enable(&mut self) -> PWRPROF_TAP_ENABLE_W<8> {
        PWRPROF_TAP_ENABLE_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_dap_enable(&mut self) -> CPU_DAP_ENABLE_W<16> {
        CPU_DAP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Access Points Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_tap_dap_0](index.html) module"]
pub struct CCFG_TAP_DAP_0_SPEC;
impl crate::RegisterSpec for CCFG_TAP_DAP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_tap_dap_0::R](R) reader structure"]
impl crate::Readable for CCFG_TAP_DAP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_tap_dap_0::W](W) writer structure"]
impl crate::Writable for CCFG_TAP_DAP_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_TAP_DAP_0 to value 0xffc5_c5c5"]
impl crate::Resettable for CCFG_TAP_DAP_0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffc5_c5c5;
}
