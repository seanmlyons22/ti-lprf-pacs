#[doc = "Register `FLASH_OTP_DATA4` reader"]
pub struct R(crate::R<FLASH_OTP_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OTP_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OTP_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OTP_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_OTP_DATA4` writer"]
pub struct W(crate::W<FLASH_OTP_DATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OTP_DATA4_SPEC>;
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
impl From<crate::W<FLASH_OTP_DATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OTP_DATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VIN_AT_X_EXT_RD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_EXT_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIN_AT_X_EXT_RD` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_EXT_RD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED1` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `DIS_STANDBY_EXT_RD` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_EXT_RD_R = crate::BitReader<bool>;
#[doc = "Field `DIS_STANDBY_EXT_RD` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_EXT_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` reader - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_EXT_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` writer - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_EXT_RD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 2, O>;
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_EXT_RD_R = crate::BitReader<bool>;
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_EXT_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `VIN_AT_X_INT_RD` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_INT_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIN_AT_X_INT_RD` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_INT_RD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED2` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED2` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `DIS_STANDBY_INT_RD` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_INT_RD_R = crate::BitReader<bool>;
#[doc = "Field `DIS_STANDBY_INT_RD` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_INT_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `STANDBY_PW_SEL_INT_RD` reader - 14:13\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_INT_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STANDBY_PW_SEL_INT_RD` writer - 14:13\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_INT_RD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 2, O>;
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_INT_RD_R = crate::BitReader<bool>;
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_INT_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `VIN_AT_X_EXT_WRT` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_EXT_WRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIN_AT_X_EXT_WRT` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_EXT_WRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED3` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `DIS_STANDBY_EXT_WRT` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_EXT_WRT_R = crate::BitReader<bool>;
#[doc = "Field `DIS_STANDBY_EXT_WRT` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_EXT_WRT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_EXT_WRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_EXT_WRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 2, O>;
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_EXT_WRT_R = crate::BitReader<bool>;
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_EXT_WRT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `VIN_AT_X_INT_WRT` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_INT_WRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VIN_AT_X_INT_WRT` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type VIN_AT_X_INT_WRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED4` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED4` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `DIS_STANDBY_INT_WRT` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_INT_WRT_R = crate::BitReader<bool>;
#[doc = "Field `DIS_STANDBY_INT_WRT` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DIS_STANDBY_INT_WRT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` reader - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_INT_WRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` writer - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_PW_SEL_INT_WRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA4_SPEC, u8, u8, 2, O>;
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_INT_WRT_R = crate::BitReader<bool>;
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type STANDBY_MODE_SEL_INT_WRT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLASH_OTP_DATA4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&self) -> VIN_AT_X_EXT_RD_R {
        VIN_AT_X_EXT_RD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&self) -> DIS_STANDBY_EXT_RD_R {
        DIS_STANDBY_EXT_RD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&self) -> STANDBY_PW_SEL_EXT_RD_R {
        STANDBY_PW_SEL_EXT_RD_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&self) -> STANDBY_MODE_SEL_EXT_RD_R {
        STANDBY_MODE_SEL_EXT_RD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&self) -> VIN_AT_X_INT_RD_R {
        VIN_AT_X_INT_RD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_rd(&self) -> DIS_STANDBY_INT_RD_R {
        DIS_STANDBY_INT_RD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&self) -> STANDBY_PW_SEL_INT_RD_R {
        STANDBY_PW_SEL_INT_RD_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&self) -> STANDBY_MODE_SEL_INT_RD_R {
        STANDBY_MODE_SEL_INT_RD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&self) -> VIN_AT_X_EXT_WRT_R {
        VIN_AT_X_EXT_WRT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&self) -> DIS_STANDBY_EXT_WRT_R {
        DIS_STANDBY_EXT_WRT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&self) -> STANDBY_PW_SEL_EXT_WRT_R {
        STANDBY_PW_SEL_EXT_WRT_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&self) -> STANDBY_MODE_SEL_EXT_WRT_R {
        STANDBY_MODE_SEL_EXT_WRT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&self) -> VIN_AT_X_INT_WRT_R {
        VIN_AT_X_INT_WRT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&self) -> DIS_STANDBY_INT_WRT_R {
        DIS_STANDBY_INT_WRT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&self) -> STANDBY_PW_SEL_INT_WRT_R {
        STANDBY_PW_SEL_INT_WRT_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&self) -> STANDBY_MODE_SEL_INT_WRT_R {
        STANDBY_MODE_SEL_INT_WRT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_ext_rd(&mut self) -> VIN_AT_X_EXT_RD_W<0> {
        VIN_AT_X_EXT_RD_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<3> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_ext_rd(&mut self) -> DIS_STANDBY_EXT_RD_W<4> {
        DIS_STANDBY_EXT_RD_W::new(self)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_ext_rd(&mut self) -> STANDBY_PW_SEL_EXT_RD_W<5> {
        STANDBY_PW_SEL_EXT_RD_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_ext_rd(&mut self) -> STANDBY_MODE_SEL_EXT_RD_W<7> {
        STANDBY_MODE_SEL_EXT_RD_W::new(self)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_int_rd(&mut self) -> VIN_AT_X_INT_RD_W<8> {
        VIN_AT_X_INT_RD_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<11> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_int_rd(&mut self) -> DIS_STANDBY_INT_RD_W<12> {
        DIS_STANDBY_INT_RD_W::new(self)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_int_rd(&mut self) -> STANDBY_PW_SEL_INT_RD_W<13> {
        STANDBY_PW_SEL_INT_RD_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_int_rd(&mut self) -> STANDBY_MODE_SEL_INT_RD_W<15> {
        STANDBY_MODE_SEL_INT_RD_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_ext_wrt(&mut self) -> VIN_AT_X_EXT_WRT_W<16> {
        VIN_AT_X_EXT_WRT_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<19> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_ext_wrt(&mut self) -> DIS_STANDBY_EXT_WRT_W<20> {
        DIS_STANDBY_EXT_WRT_W::new(self)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_ext_wrt(&mut self) -> STANDBY_PW_SEL_EXT_WRT_W<21> {
        STANDBY_PW_SEL_EXT_WRT_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_ext_wrt(&mut self) -> STANDBY_MODE_SEL_EXT_WRT_W<23> {
        STANDBY_MODE_SEL_EXT_WRT_W::new(self)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_int_wrt(&mut self) -> VIN_AT_X_INT_WRT_W<24> {
        VIN_AT_X_INT_WRT_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<27> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_int_wrt(&mut self) -> DIS_STANDBY_INT_WRT_W<28> {
        DIS_STANDBY_INT_WRT_W::new(self)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_int_wrt(&mut self) -> STANDBY_PW_SEL_INT_WRT_W<29> {
        STANDBY_PW_SEL_INT_WRT_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_int_wrt(&mut self) -> STANDBY_MODE_SEL_INT_WRT_W<31> {
        STANDBY_MODE_SEL_INT_WRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data4](index.html) module"]
pub struct FLASH_OTP_DATA4_SPEC;
impl crate::RegisterSpec for FLASH_OTP_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_otp_data4::R](R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_otp_data4::W](W) writer structure"]
impl crate::Writable for FLASH_OTP_DATA4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_OTP_DATA4 to value 0x9898_9f9f"]
impl crate::Resettable for FLASH_OTP_DATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0x9898_9f9f;
}
