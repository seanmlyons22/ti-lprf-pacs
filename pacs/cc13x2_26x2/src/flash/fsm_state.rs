#[doc = "Register `FSM_STATE` reader"]
pub type R = crate::R<FsmStateSpec>;
#[doc = "Register `FSM_STATE` writer"]
pub type W = crate::W<FsmStateSpec>;
#[doc = "Field `RESERVED0` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `OTP_ACT` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type OtpActR = crate::BitReader;
#[doc = "Field `TIOTP_ACT` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type TiotpActR = crate::BitReader;
#[doc = "Field `FSM_ACT` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FsmActR = crate::BitReader;
#[doc = "Field `RESERVED9` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `EXECUTEZ` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type ExecutezR = crate::BitReader;
#[doc = "Field `CTRLENZ` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type CtrlenzR = crate::BitReader;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp_act(&self) -> OtpActR {
        OtpActR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tiotp_act(&self) -> TiotpActR {
        TiotpActR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_act(&self) -> FsmActR {
        FsmActR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&self) -> ExecutezR {
        ExecutezR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CtrlenzR {
        CtrlenzR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmStateSpec;
impl crate::RegisterSpec for FsmStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_state::R`](R) reader structure"]
impl crate::Readable for FsmStateSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_state::W`](W) writer structure"]
impl crate::Writable for FsmStateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_STATE to value 0x0c00"]
impl crate::Resettable for FsmStateSpec {
    const RESET_VALUE: u32 = 0x0c00;
}
