#[doc = "Register `PUMP_TRIM_CFG_0` reader"]
pub struct R(crate::R<PUMP_TRIM_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUMP_TRIM_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUMP_TRIM_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUMP_TRIM_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUMP_TRIM_CFG_0` writer"]
pub struct W(crate::W<PUMP_TRIM_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUMP_TRIM_CFG_0_SPEC>;
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
impl From<crate::W<PUMP_TRIM_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUMP_TRIM_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VHVCT_ERS` reader - 9:0\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_ERS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VHVCT_ERS` writer - 9:0\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_ERS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `VHVCT_PGM` reader - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_PGM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VHVCT_PGM` writer - 19:10\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_PGM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `VHVCT_PV` reader - 29:20\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_PV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VHVCT_PV` writer - 29:20\\]
Internal. Only to be used through TI provided API."]
pub type VHVCT_PV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_0_SPEC, u16, u16, 10, O>;
#[doc = "Field `RESERVED2` reader - 31:30\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 31:30\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUMP_TRIM_CFG_0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_ers(&self) -> VHVCT_ERS_R {
        VHVCT_ERS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pgm(&self) -> VHVCT_PGM_R {
        VHVCT_PGM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - 29:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&self) -> VHVCT_PV_R {
        VHVCT_PV_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_ers(&mut self) -> VHVCT_ERS_W<0> {
        VHVCT_ERS_W::new(self)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_pgm(&mut self) -> VHVCT_PGM_W<10> {
        VHVCT_PGM_W::new(self)
    }
    #[doc = "Bits 20:29 - 29:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vhvct_pv(&mut self) -> VHVCT_PV_W<20> {
        VHVCT_PV_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<30> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pump_trim_cfg_0](index.html) module"]
pub struct PUMP_TRIM_CFG_0_SPEC;
impl crate::RegisterSpec for PUMP_TRIM_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pump_trim_cfg_0::R](R) reader structure"]
impl crate::Readable for PUMP_TRIM_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pump_trim_cfg_0::W](W) writer structure"]
impl crate::Writable for PUMP_TRIM_CFG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUMP_TRIM_CFG_0 to value 0x00b4_c53a"]
impl crate::Resettable for PUMP_TRIM_CFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00b4_c53a;
}
