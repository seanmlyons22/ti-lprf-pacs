#[doc = "Register `FLASH_OTP_DATA3` reader"]
pub type R = crate::R<FlashOtpData3Spec>;
#[doc = "Register `FLASH_OTP_DATA3` writer"]
pub type W = crate::W<FlashOtpData3Spec>;
#[doc = "Field `FLASH_SIZE` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FlashSizeR = crate::FieldReader;
#[doc = "Field `FLASH_SIZE` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type FlashSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&self) -> FlashSizeR {
        FlashSizeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flash_size(&mut self) -> FlashSizeW<FlashOtpData3Spec> {
        FlashSizeW::new(self, 0)
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
#[doc = "`reset()` method sets FLASH_OTP_DATA3 to value 0xffff_fff8"]
impl crate::Resettable for FlashOtpData3Spec {
    const RESET_VALUE: u32 = 0xffff_fff8;
}
