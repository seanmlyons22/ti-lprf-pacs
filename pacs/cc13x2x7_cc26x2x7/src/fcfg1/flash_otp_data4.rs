#[doc = "Register `FLASH_OTP_DATA4` reader"]
pub type R = crate::R<FlashOtpData4Spec>;
#[doc = "Register `FLASH_OTP_DATA4` writer"]
pub type W = crate::W<FlashOtpData4Spec>;
#[doc = "Field `VIN_AT_X_EXT_RD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXExtRdR = crate::FieldReader;
#[doc = "Field `RESERVED1` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `DIS_STANDBY_EXT_RD` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyExtRdR = crate::BitReader;
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` reader - 6:5\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelExtRdR = crate::FieldReader;
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelExtRdR = crate::BitReader;
#[doc = "Field `VIN_AT_X_INT_RD` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXIntRdR = crate::FieldReader;
#[doc = "Field `RESERVED2` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `DIS_STANDBY_INT_RD` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyIntRdR = crate::BitReader;
#[doc = "Field `STANDBY_PW_SEL_INT_RD` reader - 14:13\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelIntRdR = crate::FieldReader;
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelIntRdR = crate::BitReader;
#[doc = "Field `VIN_AT_X_EXT_WRT` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXExtWrtR = crate::FieldReader;
#[doc = "Field `RESERVED3` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `DIS_STANDBY_EXT_WRT` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyExtWrtR = crate::BitReader;
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelExtWrtR = crate::FieldReader;
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelExtWrtR = crate::BitReader;
#[doc = "Field `VIN_AT_X_INT_WRT` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXIntWrtR = crate::FieldReader;
#[doc = "Field `RESERVED4` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `DIS_STANDBY_INT_WRT` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DisStandbyIntWrtR = crate::BitReader;
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` reader - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type StandbyPwSelIntWrtR = crate::FieldReader;
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type StandbyModeSelIntWrtR = crate::BitReader;
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
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 1) != 0)
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
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 1) != 0)
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
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
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
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 1) != 0)
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
impl W {}
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
