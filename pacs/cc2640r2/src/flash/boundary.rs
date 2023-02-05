#[doc = "Register `BOUNDARY` reader"]
pub struct R(crate::R<BOUNDARY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOUNDARY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOUNDARY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOUNDARY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOUNDARY` writer"]
pub struct W(crate::W<BOUNDARY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOUNDARY_SPEC>;
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
impl From<crate::W<BOUNDARY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOUNDARY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTENABLE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type INPUTENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUTENABLE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type INPUTENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOUNDARY_SPEC, u8, u8, 4, O>;
#[doc = "Field `SYS_WS_READ_STATES` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SYS_WS_READ_STATES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYS_WS_READ_STATES` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type SYS_WS_READ_STATES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOUNDARY_SPEC, u8, u8, 4, O>;
#[doc = "Field `SYS_REPAIR_EN` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type SYS_REPAIR_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYS_REPAIR_EN` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type SYS_REPAIR_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOUNDARY_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SYS_DIEID_AUTOLOAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SYS_DIEID_AUTOLOAD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `EFC_FDI` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EFC_FDI_R = crate::BitReader<bool>;
#[doc = "Field `EFC_FDI` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EFC_FDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_OVERRIDE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_SELF_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_SELF_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `OUTPUTENABLE` reader - 17:14\\]
Internal. Only to be used through TI provided API."]
pub type OUTPUTENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTPUTENABLE` writer - 17:14\\]
Internal. Only to be used through TI provided API."]
pub type OUTPUTENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOUNDARY_SPEC, u8, u8, 4, O>;
#[doc = "Field `EFC_AUTOLOAD_ERROR` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type EFC_AUTOLOAD_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `EFC_AUTOLOAD_ERROR` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type EFC_AUTOLOAD_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `EFC_INSTRUCTION_ERROR` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `EFC_INSTRUCTION_ERROR` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `EFC_INSTRUCTION_INFO` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_INFO_R = crate::BitReader<bool>;
#[doc = "Field `EFC_INSTRUCTION_INFO` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_INFO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `EFC_SELF_TEST_ERROR` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type EFC_SELF_TEST_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `EFC_SELF_TEST_ERROR` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type EFC_SELF_TEST_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `SPARE` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type SPARE_R = crate::BitReader<bool>;
#[doc = "Field `SPARE` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type SPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `DISROW0` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type DISROW0_R = crate::BitReader<bool>;
#[doc = "Field `DISROW0` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type DISROW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOUNDARY_SPEC, bool, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOUNDARY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inputenable(&self) -> INPUTENABLE_R {
        INPUTENABLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATES_R {
        SYS_WS_READ_STATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_EN_R {
        SYS_REPAIR_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_EN_R {
        SYS_DIEID_AUTOLOAD_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fdi(&self) -> EFC_FDI_R {
        EFC_FDI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_EN_R {
        SYS_ECC_OVERRIDE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_EN_R {
        SYS_ECC_SELF_TEST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn outputenable(&self) -> OUTPUTENABLE_R {
        OUTPUTENABLE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERROR_R {
        EFC_AUTOLOAD_ERROR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERROR_R {
        EFC_INSTRUCTION_ERROR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFO_R {
        EFC_INSTRUCTION_INFO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERROR_R {
        EFC_SELF_TEST_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disrow0(&self) -> DISROW0_R {
        DISROW0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn inputenable(&mut self) -> INPUTENABLE_W<0> {
        INPUTENABLE_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ws_read_states(&mut self) -> SYS_WS_READ_STATES_W<4> {
        SYS_WS_READ_STATES_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_repair_en(&mut self) -> SYS_REPAIR_EN_W<8> {
        SYS_REPAIR_EN_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_dieid_autoload_en(&mut self) -> SYS_DIEID_AUTOLOAD_EN_W<10> {
        SYS_DIEID_AUTOLOAD_EN_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_fdi(&mut self) -> EFC_FDI_W<11> {
        EFC_FDI_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ecc_override_en(&mut self) -> SYS_ECC_OVERRIDE_EN_W<12> {
        SYS_ECC_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ecc_self_test_en(&mut self) -> SYS_ECC_SELF_TEST_EN_W<13> {
        SYS_ECC_SELF_TEST_EN_W::new(self)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn outputenable(&mut self) -> OUTPUTENABLE_W<14> {
        OUTPUTENABLE_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_autoload_error(&mut self) -> EFC_AUTOLOAD_ERROR_W<18> {
        EFC_AUTOLOAD_ERROR_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_instruction_error(&mut self) -> EFC_INSTRUCTION_ERROR_W<19> {
        EFC_INSTRUCTION_ERROR_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_instruction_info(&mut self) -> EFC_INSTRUCTION_INFO_W<20> {
        EFC_INSTRUCTION_INFO_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_self_test_error(&mut self) -> EFC_SELF_TEST_ERROR_W<21> {
        EFC_SELF_TEST_ERROR_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<22> {
        SPARE_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn disrow0(&mut self) -> DISROW0_W<23> {
        DISROW0_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boundary](index.html) module"]
pub struct BOUNDARY_SPEC;
impl crate::RegisterSpec for BOUNDARY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boundary::R](R) reader structure"]
impl crate::Readable for BOUNDARY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boundary::W](W) writer structure"]
impl crate::Writable for BOUNDARY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOUNDARY to value 0"]
impl crate::Resettable for BOUNDARY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
