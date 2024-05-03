#[doc = "Register `FLASH_OTP_DATA3` reader"]
pub type R = crate::R<FlashOtpData3Spec>;
#[doc = "Register `FLASH_OTP_DATA3` writer"]
pub type W = crate::W<FlashOtpData3Spec>;
#[doc = "Field `WAIT_SYSCODE` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type WaitSyscodeR = crate::FieldReader;
#[doc = "Field `WAIT_SYSCODE` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type WaitSyscodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLASH_SIZE` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FlashSizeR = crate::FieldReader;
#[doc = "Field `FLASH_SIZE` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FlashSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIM_1P7` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type Trim1p7R = crate::FieldReader;
#[doc = "Field `TRIM_1P7` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type Trim1p7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAX_EC_LEVEL` reader - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MaxEcLevelR = crate::FieldReader;
#[doc = "Field `MAX_EC_LEVEL` writer - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MaxEcLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DO_PRECOND` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type DoPrecondR = crate::BitReader;
#[doc = "Field `DO_PRECOND` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type DoPrecondW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC_STEP_SIZE` reader - 31:23\\]
Internal. Only to be used through TI provided API."]
pub type EcStepSizeR = crate::FieldReader<u16>;
#[doc = "Field `EC_STEP_SIZE` writer - 31:23\\]
Internal. Only to be used through TI provided API."]
pub type EcStepSizeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wait_syscode(&self) -> WaitSyscodeR {
        WaitSyscodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&self) -> FlashSizeR {
        FlashSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> Trim1p7R {
        Trim1p7R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MaxEcLevelR {
        MaxEcLevelR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DoPrecondR {
        DoPrecondR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EcStepSizeR {
        EcStepSizeR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wait_syscode(&mut self) -> WaitSyscodeW<FlashOtpData3Spec> {
        WaitSyscodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flash_size(&mut self) -> FlashSizeW<FlashOtpData3Spec> {
        FlashSizeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_1p7(&mut self) -> Trim1p7W<FlashOtpData3Spec> {
        Trim1p7W::new(self, 16)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_ec_level(&mut self) -> MaxEcLevelW<FlashOtpData3Spec> {
        MaxEcLevelW::new(self, 18)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn do_precond(&mut self) -> DoPrecondW<FlashOtpData3Spec> {
        DoPrecondW::new(self, 22)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ec_step_size(&mut self) -> EcStepSizeW<FlashOtpData3Spec> {
        EcStepSizeW::new(self, 23)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_otp_data3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_otp_data3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashOtpData3Spec;
impl crate::RegisterSpec for FlashOtpData3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_otp_data3::R`](R) reader structure"]
impl crate::Readable for FlashOtpData3Spec {}
#[doc = "`write(|w| ..)` method takes [`flash_otp_data3::W`](W) writer structure"]
impl crate::Writable for FlashOtpData3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_OTP_DATA3 to value 0x0011_0003"]
impl crate::Resettable for FlashOtpData3Spec {
    const RESET_VALUE: u32 = 0x0011_0003;
}
