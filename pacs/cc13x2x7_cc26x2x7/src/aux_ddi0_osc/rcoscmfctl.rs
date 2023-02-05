#[doc = "Register `RCOSCMFCTL` reader"]
pub struct R(crate::R<RCOSCMFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCOSCMFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCOSCMFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCOSCMFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCOSCMFCTL` writer"]
pub struct W(crate::W<RCOSCMFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCOSCMFCTL_SPEC>;
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
impl From<crate::W<RCOSCMFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCOSCMFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSC_MF_BIAS_ADJ` reader - 3:0\\]
Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
pub type RCOSC_MF_BIAS_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_MF_BIAS_ADJ` writer - 3:0\\]
Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
pub type RCOSC_MF_BIAS_ADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSCMFCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RCOSC_MF_RES_FINE` reader - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
pub type RCOSC_MF_RES_FINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_MF_RES_FINE` writer - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
pub type RCOSC_MF_RES_FINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSCMFCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RCOSC_MF_RES_COARSE` reader - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
pub type RCOSC_MF_RES_COARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_MF_RES_COARSE` writer - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
pub type RCOSC_MF_RES_COARSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSCMFCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RCOSC_MF_REG_SEL` reader - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
pub type RCOSC_MF_REG_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_MF_REG_SEL` writer - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
pub type RCOSC_MF_REG_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCOSCMFCTL_SPEC, bool, O>;
#[doc = "Field `RCOSC_MF_CAP_ARRAY` reader - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
pub type RCOSC_MF_CAP_ARRAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_MF_CAP_ARRAY` writer - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
pub type RCOSC_MF_CAP_ARRAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCOSCMFCTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `SPARE16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPARE16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCOSCMFCTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
    #[inline(always)]
    pub fn rcosc_mf_bias_adj(&self) -> RCOSC_MF_BIAS_ADJ_R {
        RCOSC_MF_BIAS_ADJ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline(always)]
    pub fn rcosc_mf_res_fine(&self) -> RCOSC_MF_RES_FINE_R {
        RCOSC_MF_RES_FINE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline(always)]
    pub fn rcosc_mf_res_coarse(&self) -> RCOSC_MF_RES_COARSE_R {
        RCOSC_MF_RES_COARSE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
    #[inline(always)]
    pub fn rcosc_mf_reg_sel(&self) -> RCOSC_MF_REG_SEL_R {
        RCOSC_MF_REG_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline(always)]
    pub fn rcosc_mf_cap_array(&self) -> RCOSC_MF_CAP_ARRAY_R {
        RCOSC_MF_CAP_ARRAY_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> SPARE16_R {
        SPARE16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_bias_adj(&mut self) -> RCOSC_MF_BIAS_ADJ_W<0> {
        RCOSC_MF_BIAS_ADJ_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_res_fine(&mut self) -> RCOSC_MF_RES_FINE_W<4> {
        RCOSC_MF_RES_FINE_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_res_coarse(&mut self) -> RCOSC_MF_RES_COARSE_W<6> {
        RCOSC_MF_RES_COARSE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_reg_sel(&mut self) -> RCOSC_MF_REG_SEL_W<8> {
        RCOSC_MF_REG_SEL_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_cap_array(&mut self) -> RCOSC_MF_CAP_ARRAY_W<9> {
        RCOSC_MF_CAP_ARRAY_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare16(&mut self) -> SPARE16_W<16> {
        SPARE16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCOSC_MF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcoscmfctl](index.html) module"]
pub struct RCOSCMFCTL_SPEC;
impl crate::RegisterSpec for RCOSCMFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcoscmfctl::R](R) reader structure"]
impl crate::Readable for RCOSCMFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcoscmfctl::W](W) writer structure"]
impl crate::Writable for RCOSCMFCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCOSCMFCTL to value 0"]
impl crate::Resettable for RCOSCMFCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
