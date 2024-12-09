#[doc = "Register `FSM_ERR_ADDR` reader"]
pub type R = crate::R<FsmErrAddrSpec>;
#[doc = "Register `FSM_ERR_ADDR` writer"]
pub type W = crate::W<FsmErrAddrSpec>;
#[doc = "Field `FSM_ERR_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmErrAddrR = crate::FieldReader<u32>;
#[doc = "Field `RESERVED24` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `FSM_ERR_BANK` reader - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type FsmErrBankR = crate::FieldReader;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_err_addr(&self) -> FsmErrAddrR {
        FsmErrAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_err_bank(&self) -> FsmErrBankR {
        FsmErrBankR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_err_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_err_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmErrAddrSpec;
impl crate::RegisterSpec for FsmErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_err_addr::R`](R) reader structure"]
impl crate::Readable for FsmErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_err_addr::W`](W) writer structure"]
impl crate::Writable for FsmErrAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ERR_ADDR to value 0"]
impl crate::Resettable for FsmErrAddrSpec {
    const RESET_VALUE: u32 = 0;
}
