#[doc = "Register `FLASH_OTP_DATA4` reader"]
pub type R = crate::R<FlashOtpData4Spec>;
#[doc = "Register `FLASH_OTP_DATA4` writer"]
pub type W = crate::W<FlashOtpData4Spec>;
#[doc = "Field `VIN_AT_X_EXT_RD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXExtRdR = crate::FieldReader;
#[doc = "Field `VIN_AT_X_EXT_RD` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXExtRdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_IDLE_EXT_RD` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleExtRdR = crate::BitReader;
#[doc = "Field `DIS_IDLE_EXT_RD` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleExtRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_STANDBY_EXT_RD` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyExtRdR = crate::BitReader;
#[doc = "Field `DIS_STANDBY_EXT_RD` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyExtRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` reader - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelExtRdR = crate::FieldReader;
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` writer - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelExtRdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelExtRdR = crate::BitReader;
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelExtRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIN_AT_X_INT_RD` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXIntRdR = crate::FieldReader;
#[doc = "Field `VIN_AT_X_INT_RD` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXIntRdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_IDLE_INT_RD` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleIntRdR = crate::BitReader;
#[doc = "Field `DIS_IDLE_INT_RD` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleIntRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_STANDBY_INT_RD` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyIntRdR = crate::BitReader;
#[doc = "Field `DIS_STANDBY_INT_RD` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyIntRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_PW_SEL_INT_RD` reader - 14:13\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelIntRdR = crate::FieldReader;
#[doc = "Field `STANDBY_PW_SEL_INT_RD` writer - 14:13\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelIntRdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelIntRdR = crate::BitReader;
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelIntRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIN_AT_X_EXT_WRT` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXExtWrtR = crate::FieldReader;
#[doc = "Field `VIN_AT_X_EXT_WRT` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXExtWrtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_IDLE_EXT_WRT` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleExtWrtR = crate::BitReader;
#[doc = "Field `DIS_IDLE_EXT_WRT` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleExtWrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_STANDBY_EXT_WRT` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyExtWrtR = crate::BitReader;
#[doc = "Field `DIS_STANDBY_EXT_WRT` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyExtWrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelExtWrtR = crate::FieldReader;
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelExtWrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelExtWrtR = crate::BitReader;
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelExtWrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIN_AT_X_INT_WRT` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXIntWrtR = crate::FieldReader;
#[doc = "Field `VIN_AT_X_INT_WRT` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXIntWrtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIS_IDLE_INT_WRT` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleIntWrtR = crate::BitReader;
#[doc = "Field `DIS_IDLE_INT_WRT` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type DisIdleIntWrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_STANDBY_INT_WRT` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyIntWrtR = crate::BitReader;
#[doc = "Field `DIS_STANDBY_INT_WRT` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyIntWrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` reader - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelIntWrtR = crate::FieldReader;
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` writer - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelIntWrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelIntWrtR = crate::BitReader;
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelIntWrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&self) -> VinAtXExtRdR {
        VinAtXExtRdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_rd(&self) -> DisIdleExtRdR {
        DisIdleExtRdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&self) -> DisStandbyExtRdR {
        DisStandbyExtRdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&self) -> StandbyPwSelExtRdR {
        StandbyPwSelExtRdR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&self) -> StandbyModeSelExtRdR {
        StandbyModeSelExtRdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&self) -> VinAtXIntRdR {
        VinAtXIntRdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_rd(&self) -> DisIdleIntRdR {
        DisIdleIntRdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_rd(&self) -> DisStandbyIntRdR {
        DisStandbyIntRdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&self) -> StandbyPwSelIntRdR {
        StandbyPwSelIntRdR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&self) -> StandbyModeSelIntRdR {
        StandbyModeSelIntRdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&self) -> VinAtXExtWrtR {
        VinAtXExtWrtR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_wrt(&self) -> DisIdleExtWrtR {
        DisIdleExtWrtR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&self) -> DisStandbyExtWrtR {
        DisStandbyExtWrtR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&self) -> StandbyPwSelExtWrtR {
        StandbyPwSelExtWrtR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&self) -> StandbyModeSelExtWrtR {
        StandbyModeSelExtWrtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&self) -> VinAtXIntWrtR {
        VinAtXIntWrtR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_wrt(&self) -> DisIdleIntWrtR {
        DisIdleIntWrtR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&self) -> DisStandbyIntWrtR {
        DisStandbyIntWrtR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&self) -> StandbyPwSelIntWrtR {
        StandbyPwSelIntWrtR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&self) -> StandbyModeSelIntWrtR {
        StandbyModeSelIntWrtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_ext_rd(&mut self) -> VinAtXExtRdW<FlashOtpData4Spec> {
        VinAtXExtRdW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_idle_ext_rd(&mut self) -> DisIdleExtRdW<FlashOtpData4Spec> {
        DisIdleExtRdW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_ext_rd(&mut self) -> DisStandbyExtRdW<FlashOtpData4Spec> {
        DisStandbyExtRdW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_ext_rd(&mut self) -> StandbyPwSelExtRdW<FlashOtpData4Spec> {
        StandbyPwSelExtRdW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_ext_rd(&mut self) -> StandbyModeSelExtRdW<FlashOtpData4Spec> {
        StandbyModeSelExtRdW::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_int_rd(&mut self) -> VinAtXIntRdW<FlashOtpData4Spec> {
        VinAtXIntRdW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_idle_int_rd(&mut self) -> DisIdleIntRdW<FlashOtpData4Spec> {
        DisIdleIntRdW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_int_rd(&mut self) -> DisStandbyIntRdW<FlashOtpData4Spec> {
        DisStandbyIntRdW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_int_rd(&mut self) -> StandbyPwSelIntRdW<FlashOtpData4Spec> {
        StandbyPwSelIntRdW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_int_rd(&mut self) -> StandbyModeSelIntRdW<FlashOtpData4Spec> {
        StandbyModeSelIntRdW::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_ext_wrt(&mut self) -> VinAtXExtWrtW<FlashOtpData4Spec> {
        VinAtXExtWrtW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_idle_ext_wrt(&mut self) -> DisIdleExtWrtW<FlashOtpData4Spec> {
        DisIdleExtWrtW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_ext_wrt(&mut self) -> DisStandbyExtWrtW<FlashOtpData4Spec> {
        DisStandbyExtWrtW::new(self, 20)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_ext_wrt(&mut self) -> StandbyPwSelExtWrtW<FlashOtpData4Spec> {
        StandbyPwSelExtWrtW::new(self, 21)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_ext_wrt(&mut self) -> StandbyModeSelExtWrtW<FlashOtpData4Spec> {
        StandbyModeSelExtWrtW::new(self, 23)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x_int_wrt(&mut self) -> VinAtXIntWrtW<FlashOtpData4Spec> {
        VinAtXIntWrtW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_idle_int_wrt(&mut self) -> DisIdleIntWrtW<FlashOtpData4Spec> {
        DisIdleIntWrtW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_standby_int_wrt(&mut self) -> DisStandbyIntWrtW<FlashOtpData4Spec> {
        DisStandbyIntWrtW::new(self, 28)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_pw_sel_int_wrt(&mut self) -> StandbyPwSelIntWrtW<FlashOtpData4Spec> {
        StandbyPwSelIntWrtW::new(self, 29)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn standby_mode_sel_int_wrt(&mut self) -> StandbyModeSelIntWrtW<FlashOtpData4Spec> {
        StandbyModeSelIntWrtW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_otp_data4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_otp_data4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashOtpData4Spec;
impl crate::RegisterSpec for FlashOtpData4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_otp_data4::R`](R) reader structure"]
impl crate::Readable for FlashOtpData4Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_otp_data4::W`](W) writer structure"]
impl crate::Writable for FlashOtpData4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_OTP_DATA4 to value 0x9898_9f9f"]
impl crate::Resettable for FlashOtpData4Spec {
    const RESET_VALUE: u32 = 0x9898_9f9f;
}
