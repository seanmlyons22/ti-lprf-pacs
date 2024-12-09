#[doc = "Register `FSM_ADDR` reader"]
pub type R = crate::R<FsmAddrSpec>;
#[doc = "Register `FSM_ADDR` writer"]
pub type W = crate::W<FsmAddrSpec>;
#[doc = "Field `CUR_ADDR` reader - 27:0\\]
Internal. Only to be used through TI provided API."]
pub type CurAddrR = crate::FieldReader<u32>;
#[doc = "Field `BANK` reader - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type BankR = crate::FieldReader;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31R = crate::BitReader;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_addr(&self) -> CurAddrR {
        CurAddrR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&self) -> BankR {
        BankR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmAddrSpec;
impl crate::RegisterSpec for FsmAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_addr::R`](R) reader structure"]
impl crate::Readable for FsmAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_addr::W`](W) writer structure"]
impl crate::Writable for FsmAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_ADDR to value 0"]
impl crate::Resettable for FsmAddrSpec {
    const RESET_VALUE: u32 = 0;
}
