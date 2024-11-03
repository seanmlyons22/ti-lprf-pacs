#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "0:0\\]
This bit indicates if the system time is initialized and running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "1: system timer is running"]
    Run = 1,
    #[doc = "0: system timer is not running."]
    Stop = 0,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - 0:0\\]
This bit indicates if the system time is initialized and running."]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            true => Val::Run,
            false => Val::Stop,
        }
    }
    #[doc = "system timer is running"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Val::Run
    }
    #[doc = "system timer is not running."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Val::Stop
    }
}
#[doc = "Field `VAL` writer - 0:0\\]
This bit indicates if the system time is initialized and running."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "system timer is running"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Run)
    }
    #[doc = "system timer is not running."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Stop)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYNCUP` reader - 4:4\\]
This bit indicates sync status of Systimer with RTC. The bitfield has a reset value of '1' , which gets cleared to '0' after the systimer synchronizes with RTC on the first LFTICK edge. A write to this bit resynchronizes the Systimer with RTC on the next LFTICK edge. A read value of '1' indicates the synchronization is ongoing and a read of '0' indicates the synchronization is done."]
pub type SyncupR = crate::BitReader;
#[doc = "Field `SYNCUP` writer - 4:4\\]
This bit indicates sync status of Systimer with RTC. The bitfield has a reset value of '1' , which gets cleared to '0' after the systimer synchronizes with RTC on the first LFTICK edge. A write to this bit resynchronizes the Systimer with RTC on the next LFTICK edge. A read value of '1' indicates the synchronization is ongoing and a read of '0' indicates the synchronization is done."]
pub type SyncupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit indicates if the system time is initialized and running."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit indicates sync status of Systimer with RTC. The bitfield has a reset value of '1' , which gets cleared to '0' after the systimer synchronizes with RTC on the first LFTICK edge. A write to this bit resynchronizes the Systimer with RTC on the next LFTICK edge. A read value of '1' indicates the synchronization is ongoing and a read of '0' indicates the synchronization is done."]
    #[inline(always)]
    pub fn syncup(&self) -> SyncupR {
        SyncupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit indicates if the system time is initialized and running."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<StatusSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<StatusSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit indicates sync status of Systimer with RTC. The bitfield has a reset value of '1' , which gets cleared to '0' after the systimer synchronizes with RTC on the first LFTICK edge. A write to this bit resynchronizes the Systimer with RTC on the next LFTICK edge. A read value of '1' indicates the synchronization is ongoing and a read of '0' indicates the synchronization is done."]
    #[inline(always)]
    #[must_use]
    pub fn syncup(&mut self) -> SyncupW<StatusSpec> {
        SyncupW::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<StatusSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "Systimer status register. This register can be used to read the running status of the timer and to resync the Systimer with RTC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x10"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x10;
}
