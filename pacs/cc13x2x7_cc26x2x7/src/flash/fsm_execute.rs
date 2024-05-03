#[doc = "Register `FSM_EXECUTE` reader"]
pub type R = crate::R<FsmExecuteSpec>;
#[doc = "Register `FSM_EXECUTE` writer"]
pub type W = crate::W<FsmExecuteSpec>;
#[doc = "Field `FSMEXECUTE` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmexecuteR = crate::FieldReader;
#[doc = "Field `FSMEXECUTE` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type FsmexecuteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED5` reader - 15:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED5` writer - 15:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SUSPEND_NOW` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type SuspendNowR = crate::FieldReader;
#[doc = "Field `SUSPEND_NOW` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type SuspendNowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmexecute(&self) -> FsmexecuteR {
        FsmexecuteR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn suspend_now(&self) -> SuspendNowR {
        SuspendNowR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsmexecute(&mut self) -> FsmexecuteW<FsmExecuteSpec> {
        FsmexecuteW::new(self, 0)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<FsmExecuteSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn suspend_now(&mut self) -> SuspendNowW<FsmExecuteSpec> {
        SuspendNowW::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<FsmExecuteSpec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_execute::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_execute::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmExecuteSpec;
impl crate::RegisterSpec for FsmExecuteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_execute::R`](R) reader structure"]
impl crate::Readable for FsmExecuteSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_execute::W`](W) writer structure"]
impl crate::Writable for FsmExecuteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_EXECUTE to value 0x000a_000a"]
impl crate::Resettable for FsmExecuteSpec {
    const RESET_VALUE: u32 = 0x000a_000a;
}
