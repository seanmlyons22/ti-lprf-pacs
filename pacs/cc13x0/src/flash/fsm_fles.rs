#[doc = "Register `FSM_FLES` reader"]
pub type R = crate::R<FsmFlesSpec>;
#[doc = "Register `FSM_FLES` writer"]
pub type W = crate::W<FsmFlesSpec>;
#[doc = "Field `BLK_OTP` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type BlkOtpR = crate::FieldReader;
#[doc = "Field `BLK_OTP` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type BlkOtpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLK_TIOTP` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type BlkTiotpR = crate::FieldReader;
#[doc = "Field `BLK_TIOTP` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type BlkTiotpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn blk_otp(&self) -> BlkOtpR {
        BlkOtpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn blk_tiotp(&self) -> BlkTiotpR {
        BlkTiotpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn blk_otp(&mut self) -> BlkOtpW<FsmFlesSpec> {
        BlkOtpW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn blk_tiotp(&mut self) -> BlkTiotpW<FsmFlesSpec> {
        BlkTiotpW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_fles::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_fles::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmFlesSpec;
impl crate::RegisterSpec for FsmFlesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_fles::R`](R) reader structure"]
impl crate::Readable for FsmFlesSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_fles::W`](W) writer structure"]
impl crate::Writable for FsmFlesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_FLES to value 0"]
impl crate::Resettable for FsmFlesSpec {
    const RESET_VALUE: u32 = 0;
}
