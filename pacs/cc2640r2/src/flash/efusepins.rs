#[doc = "Register `EFUSEPINS` reader"]
pub struct R(crate::R<EFUSEPINS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEPINS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEPINS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEPINS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEPINS` writer"]
pub struct W(crate::W<EFUSEPINS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEPINS_SPEC>;
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
impl From<crate::W<EFUSEPINS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEPINS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS_WS_READ_STATES` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SYS_WS_READ_STATES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYS_WS_READ_STATES` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type SYS_WS_READ_STATES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEPINS_SPEC, u8, u8, 4, O>;
#[doc = "Field `SYS_REPAIR_EN` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type SYS_REPAIR_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYS_REPAIR_EN` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type SYS_REPAIR_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEPINS_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type SYS_DIEID_AUTOLOAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type SYS_DIEID_AUTOLOAD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_FCLRZ` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type EFC_FCLRZ_R = crate::BitReader<bool>;
#[doc = "Field `EFC_FCLRZ` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type EFC_FCLRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_READY` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EFC_READY_R = crate::BitReader<bool>;
#[doc = "Field `EFC_READY` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EFC_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYS_ECC_OVERRIDE_EN` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_AUTOLOAD_ERROR` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EFC_AUTOLOAD_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `EFC_AUTOLOAD_ERROR` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EFC_AUTOLOAD_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_INSTRUCTION_ERROR` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `EFC_INSTRUCTION_ERROR` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_INSTRUCTION_INFO` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_INFO_R = crate::BitReader<bool>;
#[doc = "Field `EFC_INSTRUCTION_INFO` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type EFC_INSTRUCTION_INFO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_SELF_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SYS_ECC_SELF_TEST_EN` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type SYS_ECC_SELF_TEST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_SELF_TEST_ERROR` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type EFC_SELF_TEST_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `EFC_SELF_TEST_ERROR` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub type EFC_SELF_TEST_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `EFC_SELF_TEST_DONE` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type EFC_SELF_TEST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `EFC_SELF_TEST_DONE` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type EFC_SELF_TEST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EFUSEPINS_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EFUSEPINS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATES_R {
        SYS_WS_READ_STATES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_EN_R {
        SYS_REPAIR_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_EN_R {
        SYS_DIEID_AUTOLOAD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fclrz(&self) -> EFC_FCLRZ_R {
        EFC_FCLRZ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_ready(&self) -> EFC_READY_R {
        EFC_READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_EN_R {
        SYS_ECC_OVERRIDE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERROR_R {
        EFC_AUTOLOAD_ERROR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERROR_R {
        EFC_INSTRUCTION_ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFO_R {
        EFC_INSTRUCTION_INFO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_EN_R {
        SYS_ECC_SELF_TEST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERROR_R {
        EFC_SELF_TEST_ERROR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_done(&self) -> EFC_SELF_TEST_DONE_R {
        EFC_SELF_TEST_DONE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ws_read_states(&mut self) -> SYS_WS_READ_STATES_W<0> {
        SYS_WS_READ_STATES_W::new(self)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_repair_en(&mut self) -> SYS_REPAIR_EN_W<4> {
        SYS_REPAIR_EN_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_dieid_autoload_en(&mut self) -> SYS_DIEID_AUTOLOAD_EN_W<6> {
        SYS_DIEID_AUTOLOAD_EN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_fclrz(&mut self) -> EFC_FCLRZ_W<7> {
        EFC_FCLRZ_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_ready(&mut self) -> EFC_READY_W<8> {
        EFC_READY_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ecc_override_en(&mut self) -> SYS_ECC_OVERRIDE_EN_W<9> {
        SYS_ECC_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_autoload_error(&mut self) -> EFC_AUTOLOAD_ERROR_W<10> {
        EFC_AUTOLOAD_ERROR_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_instruction_error(&mut self) -> EFC_INSTRUCTION_ERROR_W<11> {
        EFC_INSTRUCTION_ERROR_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_instruction_info(&mut self) -> EFC_INSTRUCTION_INFO_W<12> {
        EFC_INSTRUCTION_INFO_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sys_ecc_self_test_en(&mut self) -> SYS_ECC_SELF_TEST_EN_W<13> {
        SYS_ECC_SELF_TEST_EN_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_self_test_error(&mut self) -> EFC_SELF_TEST_ERROR_W<14> {
        EFC_SELF_TEST_ERROR_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efc_self_test_done(&mut self) -> EFC_SELF_TEST_DONE_W<15> {
        EFC_SELF_TEST_DONE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusepins](index.html) module"]
pub struct EFUSEPINS_SPEC;
impl crate::RegisterSpec for EFUSEPINS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efusepins::R](R) reader structure"]
impl crate::Readable for EFUSEPINS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efusepins::W](W) writer structure"]
impl crate::Writable for EFUSEPINS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSEPINS to value 0"]
impl crate::Resettable for EFUSEPINS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
