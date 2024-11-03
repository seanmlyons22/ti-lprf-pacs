#[doc = "Register `STA` reader"]
pub type R = crate::R<StaSpec>;
#[doc = "Register `STA` writer"]
pub type W = crate::W<StaSpec>;
#[doc = "0:0\\]
State Field gives the state of the ECB encryption engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "1: ECB encryption active."]
    Busy = 1,
    #[doc = "0: ECB is IDLE."]
    Idle = 0,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - 0:0\\]
State Field gives the state of the ECB encryption engine."]
pub type StateR = crate::BitReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            true => State::Busy,
            false => State::Idle,
        }
    }
    #[doc = "ECB encryption active."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == State::Busy
    }
    #[doc = "ECB is IDLE."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == State::Idle
    }
}
#[doc = "Field `STATE` writer - 0:0\\]
State Field gives the state of the ECB encryption engine."]
pub type StateW<'a, REG> = crate::BitWriter<'a, REG, State>;
impl<'a, REG> StateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECB encryption active."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(State::Busy)
    }
    #[doc = "ECB is IDLE."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(State::Idle)
    }
}
#[doc = "1:1\\]
BUF Status Field gives the status of BUF, indicating EMPTY or FULL, when AUTOCFG.TRGECB = WRBUF3. If AUTOCFG.TRGECB != WRBUF3, then STA.BUFSTA will hold the value 0. Note : Useful for CBC-MAC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufsta {
    #[doc = "1: Data stored in BUF is not yet consumed by the AES engine. Next block of data cannot be written into BUF until STA.STATE = IDLE."]
    Full = 1,
    #[doc = "0: Data stored in BUF is already consumed by the AES engine and next block of data can be written in BUF"]
    Empty = 0,
}
impl From<Bufsta> for bool {
    #[inline(always)]
    fn from(variant: Bufsta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFSTA` reader - 1:1\\]
BUF Status Field gives the status of BUF, indicating EMPTY or FULL, when AUTOCFG.TRGECB = WRBUF3. If AUTOCFG.TRGECB != WRBUF3, then STA.BUFSTA will hold the value 0. Note : Useful for CBC-MAC"]
pub type BufstaR = crate::BitReader<Bufsta>;
impl BufstaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufsta {
        match self.bits {
            true => Bufsta::Full,
            false => Bufsta::Empty,
        }
    }
    #[doc = "Data stored in BUF is not yet consumed by the AES engine. Next block of data cannot be written into BUF until STA.STATE = IDLE."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Bufsta::Full
    }
    #[doc = "Data stored in BUF is already consumed by the AES engine and next block of data can be written in BUF"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Bufsta::Empty
    }
}
#[doc = "Field `BUFSTA` writer - 1:1\\]
BUF Status Field gives the status of BUF, indicating EMPTY or FULL, when AUTOCFG.TRGECB = WRBUF3. If AUTOCFG.TRGECB != WRBUF3, then STA.BUFSTA will hold the value 0. Note : Useful for CBC-MAC"]
pub type BufstaW<'a, REG> = crate::BitWriter<'a, REG, Bufsta>;
impl<'a, REG> BufstaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data stored in BUF is not yet consumed by the AES engine. Next block of data cannot be written into BUF until STA.STATE = IDLE."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Bufsta::Full)
    }
    #[doc = "Data stored in BUF is already consumed by the AES engine and next block of data can be written in BUF"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Bufsta::Empty)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
State Field gives the state of the ECB encryption engine."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BUF Status Field gives the status of BUF, indicating EMPTY or FULL, when AUTOCFG.TRGECB = WRBUF3. If AUTOCFG.TRGECB != WRBUF3, then STA.BUFSTA will hold the value 0. Note : Useful for CBC-MAC"]
    #[inline(always)]
    pub fn bufsta(&self) -> BufstaR {
        BufstaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
State Field gives the state of the ECB encryption engine."]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<StaSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BUF Status Field gives the status of BUF, indicating EMPTY or FULL, when AUTOCFG.TRGECB = WRBUF3. If AUTOCFG.TRGECB != WRBUF3, then STA.BUFSTA will hold the value 0. Note : Useful for CBC-MAC"]
    #[inline(always)]
    #[must_use]
    pub fn bufsta(&mut self) -> BufstaW<StaSpec> {
        BufstaW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<StaSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Status This register provides information on ECB accellerator state and BUF status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaSpec;
impl crate::RegisterSpec for StaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for StaSpec {}
#[doc = "`write(|w| ..)` method takes [`sta::W`](W) writer structure"]
impl crate::Writable for StaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for StaSpec {
    const RESET_VALUE: u32 = 0;
}
