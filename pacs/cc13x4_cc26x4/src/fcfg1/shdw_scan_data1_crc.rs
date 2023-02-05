#[doc = "Register `SHDW_SCAN_DATA1_CRC` reader"]
pub struct R(crate::R<SHDW_SCAN_DATA1_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_SCAN_DATA1_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_SCAN_DATA1_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_SCAN_DATA1_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_SCAN_DATA1_CRC` writer"]
pub struct W(crate::W<SHDW_SCAN_DATA1_CRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_SCAN_DATA1_CRC_SPEC>;
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
impl From<crate::W<SHDW_SCAN_DATA1_CRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_SCAN_DATA1_CRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAP_DAP_LOCK_N` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TAP_DAP_LOCK_N_R = crate::BitReader<bool>;
#[doc = "Field `TAP_DAP_LOCK_N` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TAP_DAP_LOCK_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHDW_SCAN_DATA1_CRC_SPEC, bool, O>;
#[doc = "Field `CRC` reader - 8:1\\]
Internal. Only to be used through TI provided API."]
pub type CRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC` writer - 8:1\\]
Internal. Only to be used through TI provided API."]
pub type CRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_SCAN_DATA1_CRC_SPEC, u8, u8, 8, O>;
#[doc = "Field `FLASH_RDY` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type FLASH_RDY_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_RDY` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type FLASH_RDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHDW_SCAN_DATA1_CRC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tap_dap_lock_n(&self) -> TAP_DAP_LOCK_N_R {
        TAP_DAP_LOCK_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - 8:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tap_dap_lock_n(&mut self) -> TAP_DAP_LOCK_N_W<0> {
        TAP_DAP_LOCK_N_W::new(self)
    }
    #[doc = "Bits 1:8 - 8:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<1> {
        CRC_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flash_rdy(&mut self) -> FLASH_RDY_W<31> {
        FLASH_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_scan_data1_crc](index.html) module"]
pub struct SHDW_SCAN_DATA1_CRC_SPEC;
impl crate::RegisterSpec for SHDW_SCAN_DATA1_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_scan_data1_crc::R](R) reader structure"]
impl crate::Readable for SHDW_SCAN_DATA1_CRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_scan_data1_crc::W](W) writer structure"]
impl crate::Writable for SHDW_SCAN_DATA1_CRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHDW_SCAN_DATA1_CRC to value 0"]
impl crate::Resettable for SHDW_SCAN_DATA1_CRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
