#[doc = "Register `FEDACSTAT` reader"]
pub type R = crate::R<FedacstatSpec>;
#[doc = "Register `FEDACSTAT` writer"]
pub type W = crate::W<FedacstatSpec>;
#[doc = "Field `ERR_PRF_FLG` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type ErrPrfFlgR = crate::FieldReader<u32>;
#[doc = "Field `FSM_DONE` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type FsmDoneR = crate::BitReader;
#[doc = "Field `FSM_DONE` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type FsmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RVF_INT` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type RvfIntR = crate::BitReader;
#[doc = "Field `RVF_INT` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub type RvfIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type Reserved26R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn err_prf_flg(&self) -> ErrPrfFlgR {
        ErrPrfFlgR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_done(&self) -> FsmDoneR {
        FsmDoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf_int(&self) -> RvfIntR {
        RvfIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_done(&mut self) -> FsmDoneW<FedacstatSpec> {
        FsmDoneW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rvf_int(&mut self) -> RvfIntW<FedacstatSpec> {
        RvfIntW::new(self, 25)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FedacstatSpec;
impl crate::RegisterSpec for FedacstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fedacstat::R`](R) reader structure"]
impl crate::Readable for FedacstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fedacstat::W`](W) writer structure"]
impl crate::Writable for FedacstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEDACSTAT to value 0"]
impl crate::Resettable for FedacstatSpec {
    const RESET_VALUE: u32 = 0;
}
