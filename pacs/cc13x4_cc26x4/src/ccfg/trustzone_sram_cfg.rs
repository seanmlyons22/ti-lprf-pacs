#[doc = "Register `TRUSTZONE_SRAM_CFG` reader"]
pub struct R(crate::R<TRUSTZONE_SRAM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRUSTZONE_SRAM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRUSTZONE_SRAM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRUSTZONE_SRAM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRUSTZONE_SRAM_CFG` writer"]
pub struct W(crate::W<TRUSTZONE_SRAM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRUSTZONE_SRAM_CFG_SPEC>;
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
impl From<crate::W<TRUSTZONE_SRAM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRUSTZONE_SRAM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSCADDR_BOUNDARY` reader - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NSCADDR_BOUNDARY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSCADDR_BOUNDARY` writer - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NSCADDR_BOUNDARY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRUSTZONE_SRAM_CFG_SPEC, u16, u16, 9, O>;
#[doc = "Field `NSADDR_BOUNDARY` reader - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NSADDR_BOUNDARY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSADDR_BOUNDARY` writer - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
pub type NSADDR_BOUNDARY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRUSTZONE_SRAM_CFG_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    pub fn nscaddr_boundary(&self) -> NSCADDR_BOUNDARY_R {
        NSCADDR_BOUNDARY_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    pub fn nsaddr_boundary(&self) -> NSADDR_BOUNDARY_R {
        NSADDR_BOUNDARY_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Value will be written to PRCM:SRAMNSCADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn nscaddr_boundary(&mut self) -> NSCADDR_BOUNDARY_W<0> {
        NSCADDR_BOUNDARY_W::new(self)
    }
    #[doc = "Bits 9:17 - 17:9\\]
Value will be written to PRCM:SRAMNSADDR.BOUNDARY by ROM boot FW only if CCFG_TI_OPTIONS.IDAU_CFG_ENABLE != 0xC5."]
    #[inline(always)]
    #[must_use]
    pub fn nsaddr_boundary(&mut self) -> NSADDR_BOUNDARY_W<9> {
        NSADDR_BOUNDARY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trustzone configuration register for MCU SRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trustzone_sram_cfg](index.html) module"]
pub struct TRUSTZONE_SRAM_CFG_SPEC;
impl crate::RegisterSpec for TRUSTZONE_SRAM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trustzone_sram_cfg::R](R) reader structure"]
impl crate::Readable for TRUSTZONE_SRAM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trustzone_sram_cfg::W](W) writer structure"]
impl crate::Writable for TRUSTZONE_SRAM_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRUSTZONE_SRAM_CFG to value 0xffff_ffff"]
impl crate::Resettable for TRUSTZONE_SRAM_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
