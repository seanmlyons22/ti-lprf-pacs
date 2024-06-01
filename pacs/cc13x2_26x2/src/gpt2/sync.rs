#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "1:0\\]
Synchronize GPT Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync0 {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    Both = 3,
    #[doc = "2: A timeout event for Timer B of GPT0 is triggered"]
    Timerb = 2,
    #[doc = "1: A timeout event for Timer A of GPT0 is triggered"]
    Timera = 1,
    #[doc = "0: No Sync. GPT0 is not affected."]
    Nosync = 0,
}
impl From<Sync0> for u8 {
    #[inline(always)]
    fn from(variant: Sync0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync0 {
    type Ux = u8;
}
impl crate::IsEnum for Sync0 {}
#[doc = "Field `SYNC0` reader - 1:0\\]
Synchronize GPT Timer 0"]
pub type Sync0R = crate::FieldReader<Sync0>;
impl Sync0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync0 {
        match self.bits {
            3 => Sync0::Both,
            2 => Sync0::Timerb,
            1 => Sync0::Timera,
            0 => Sync0::Nosync,
            _ => unreachable!(),
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sync0::Both
    }
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == Sync0::Timerb
    }
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == Sync0::Timera
    }
    #[doc = "No Sync. GPT0 is not affected."]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == Sync0::Nosync
    }
}
#[doc = "Field `SYNC0` writer - 1:0\\]
Synchronize GPT Timer 0"]
pub type Sync0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sync0, crate::Safe>;
impl<'a, REG> Sync0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0::Both)
    }
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0::Timerb)
    }
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0::Timera)
    }
    #[doc = "No Sync. GPT0 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut crate::W<REG> {
        self.variant(Sync0::Nosync)
    }
}
#[doc = "3:2\\]
Synchronize GPT Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync1 {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    Both = 3,
    #[doc = "2: A timeout event for Timer B of GPT1 is triggered"]
    Timerb = 2,
    #[doc = "1: A timeout event for Timer A of GPT1 is triggered"]
    Timera = 1,
    #[doc = "0: No Sync. GPT1 is not affected."]
    Nosync = 0,
}
impl From<Sync1> for u8 {
    #[inline(always)]
    fn from(variant: Sync1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync1 {
    type Ux = u8;
}
impl crate::IsEnum for Sync1 {}
#[doc = "Field `SYNC1` reader - 3:2\\]
Synchronize GPT Timer 1"]
pub type Sync1R = crate::FieldReader<Sync1>;
impl Sync1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync1 {
        match self.bits {
            3 => Sync1::Both,
            2 => Sync1::Timerb,
            1 => Sync1::Timera,
            0 => Sync1::Nosync,
            _ => unreachable!(),
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sync1::Both
    }
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == Sync1::Timerb
    }
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == Sync1::Timera
    }
    #[doc = "No Sync. GPT1 is not affected."]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == Sync1::Nosync
    }
}
#[doc = "Field `SYNC1` writer - 3:2\\]
Synchronize GPT Timer 1"]
pub type Sync1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sync1, crate::Safe>;
impl<'a, REG> Sync1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1::Both)
    }
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1::Timerb)
    }
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1::Timera)
    }
    #[doc = "No Sync. GPT1 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut crate::W<REG> {
        self.variant(Sync1::Nosync)
    }
}
#[doc = "5:4\\]
Synchronize GPT Timer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync2 {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    Both = 3,
    #[doc = "2: A timeout event for Timer B of GPT2 is triggered"]
    Timerb = 2,
    #[doc = "1: A timeout event for Timer A of GPT2 is triggered"]
    Timera = 1,
    #[doc = "0: No Sync. GPT2 is not affected."]
    Nosync = 0,
}
impl From<Sync2> for u8 {
    #[inline(always)]
    fn from(variant: Sync2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync2 {
    type Ux = u8;
}
impl crate::IsEnum for Sync2 {}
#[doc = "Field `SYNC2` reader - 5:4\\]
Synchronize GPT Timer 2."]
pub type Sync2R = crate::FieldReader<Sync2>;
impl Sync2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync2 {
        match self.bits {
            3 => Sync2::Both,
            2 => Sync2::Timerb,
            1 => Sync2::Timera,
            0 => Sync2::Nosync,
            _ => unreachable!(),
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sync2::Both
    }
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == Sync2::Timerb
    }
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == Sync2::Timera
    }
    #[doc = "No Sync. GPT2 is not affected."]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == Sync2::Nosync
    }
}
#[doc = "Field `SYNC2` writer - 5:4\\]
Synchronize GPT Timer 2."]
pub type Sync2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sync2, crate::Safe>;
impl<'a, REG> Sync2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sync2::Both)
    }
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut crate::W<REG> {
        self.variant(Sync2::Timerb)
    }
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut crate::W<REG> {
        self.variant(Sync2::Timera)
    }
    #[doc = "No Sync. GPT2 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut crate::W<REG> {
        self.variant(Sync2::Nosync)
    }
}
#[doc = "7:6\\]
Synchronize GPT Timer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync3 {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    Both = 3,
    #[doc = "2: A timeout event for Timer B of GPT3 is triggered"]
    Timerb = 2,
    #[doc = "1: A timeout event for Timer A of GPT3 is triggered"]
    Timera = 1,
    #[doc = "0: No Sync. GPT3 is not affected."]
    Nosync = 0,
}
impl From<Sync3> for u8 {
    #[inline(always)]
    fn from(variant: Sync3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync3 {
    type Ux = u8;
}
impl crate::IsEnum for Sync3 {}
#[doc = "Field `SYNC3` reader - 7:6\\]
Synchronize GPT Timer 3."]
pub type Sync3R = crate::FieldReader<Sync3>;
impl Sync3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync3 {
        match self.bits {
            3 => Sync3::Both,
            2 => Sync3::Timerb,
            1 => Sync3::Timera,
            0 => Sync3::Nosync,
            _ => unreachable!(),
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sync3::Both
    }
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == Sync3::Timerb
    }
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == Sync3::Timera
    }
    #[doc = "No Sync. GPT3 is not affected."]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == Sync3::Nosync
    }
}
#[doc = "Field `SYNC3` writer - 7:6\\]
Synchronize GPT Timer 3."]
pub type Sync3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sync3, crate::Safe>;
impl<'a, REG> Sync3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sync3::Both)
    }
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut crate::W<REG> {
        self.variant(Sync3::Timerb)
    }
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut crate::W<REG> {
        self.variant(Sync3::Timera)
    }
    #[doc = "No Sync. GPT3 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut crate::W<REG> {
        self.variant(Sync3::Nosync)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Synchronize GPT Timer 0"]
    #[inline(always)]
    pub fn sync0(&self) -> Sync0R {
        Sync0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Synchronize GPT Timer 1"]
    #[inline(always)]
    pub fn sync1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronize GPT Timer 2."]
    #[inline(always)]
    pub fn sync2(&self) -> Sync2R {
        Sync2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Synchronize GPT Timer 3."]
    #[inline(always)]
    pub fn sync3(&self) -> Sync3R {
        Sync3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Synchronize GPT Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn sync0(&mut self) -> Sync0W<SyncSpec> {
        Sync0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Synchronize GPT Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn sync1(&mut self) -> Sync1W<SyncSpec> {
        Sync1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronize GPT Timer 2."]
    #[inline(always)]
    #[must_use]
    pub fn sync2(&mut self) -> Sync2W<SyncSpec> {
        Sync2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Synchronize GPT Timer 3."]
    #[inline(always)]
    #[must_use]
    pub fn sync3(&mut self) -> Sync3W<SyncSpec> {
        Sync3W::new(self, 6)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SyncSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Synch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SyncSpec {
    const RESET_VALUE: u32 = 0;
}
