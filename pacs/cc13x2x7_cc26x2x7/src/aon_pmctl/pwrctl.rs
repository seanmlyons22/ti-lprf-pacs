#[doc = "Register `PWRCTL` reader"]
pub struct R(crate::R<PWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTL` writer"]
pub struct W(crate::W<PWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTL_SPEC>;
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
impl From<crate::W<PWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDC_EN` reader - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
pub type DCDC_EN_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_EN` writer - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
pub type DCDC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTL_SPEC, bool, O>;
#[doc = "Field `EXT_REG_MODE` reader - 1:1\\]
Status of source for VDDRsupply: 0: DCDC or GLDO are generating VDDR 1: DCDC and GLDO are bypassed and an external regulator supplies VDDR"]
pub type EXT_REG_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EXT_REG_MODE` writer - 1:1\\]
Status of source for VDDRsupply: 0: DCDC or GLDO are generating VDDR 1: DCDC and GLDO are bypassed and an external regulator supplies VDDR"]
pub type EXT_REG_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTL_SPEC, bool, O>;
#[doc = "Field `DCDC_ACTIVE` reader - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDR in active mode. 1: Use DCDC for regulation of VDDR in active mode. DCDC_EN must also be set for DCDC to be used as regulator for VDDR in active mode"]
pub type DCDC_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_ACTIVE` writer - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDR in active mode. 1: Use DCDC for regulation of VDDR in active mode. DCDC_EN must also be set for DCDC to be used as regulator for VDDR in active mode"]
pub type DCDC_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWRCTL_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_en(&self) -> DCDC_EN_R {
        DCDC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC or GLDO are generating VDDR 1: DCDC and GLDO are bypassed and an external regulator supplies VDDR"]
    #[inline(always)]
    pub fn ext_reg_mode(&self) -> EXT_REG_MODE_R {
        EXT_REG_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDR in active mode. 1: Use DCDC for regulation of VDDR in active mode. DCDC_EN must also be set for DCDC to be used as regulator for VDDR in active mode"]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
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
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en(&mut self) -> DCDC_EN_W<0> {
        DCDC_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC or GLDO are generating VDDR 1: DCDC and GLDO are bypassed and an external regulator supplies VDDR"]
    #[inline(always)]
    #[must_use]
    pub fn ext_reg_mode(&mut self) -> EXT_REG_MODE_W<1> {
        EXT_REG_MODE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDR in active mode. 1: Use DCDC for regulation of VDDR in active mode. DCDC_EN must also be set for DCDC to be used as regulator for VDDR in active mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_active(&mut self) -> DCDC_ACTIVE_W<2> {
        DCDC_ACTIVE_W::new(self)
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
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctl](index.html) module"]
pub struct PWRCTL_SPEC;
impl crate::RegisterSpec for PWRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctl::R](R) reader structure"]
impl crate::Readable for PWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctl::W](W) writer structure"]
impl crate::Writable for PWRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PWRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
