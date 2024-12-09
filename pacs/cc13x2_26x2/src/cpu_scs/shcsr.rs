#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<ShcsrSpec>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<ShcsrSpec>;
#[doc = "0:0\\]
MemManage exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memfaultact {
    #[doc = "1: Exception is active"]
    Active = 1,
    #[doc = "0: Exception is not active"]
    Notactive = 0,
}
impl From<Memfaultact> for bool {
    #[inline(always)]
    fn from(variant: Memfaultact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTACT` reader - 0:0\\]
MemManage exception active"]
pub type MemfaultactR = crate::BitReader<Memfaultact>;
impl MemfaultactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memfaultact {
        match self.bits {
            true => Memfaultact::Active,
            false => Memfaultact::Notactive,
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Memfaultact::Active
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Memfaultact::Notactive
    }
}
#[doc = "1:1\\]
BusFault exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busfaultact {
    #[doc = "1: Exception is active"]
    Active = 1,
    #[doc = "0: Exception is not active"]
    Notactive = 0,
}
impl From<Busfaultact> for bool {
    #[inline(always)]
    fn from(variant: Busfaultact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTACT` reader - 1:1\\]
BusFault exception active"]
pub type BusfaultactR = crate::BitReader<Busfaultact>;
impl BusfaultactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busfaultact {
        match self.bits {
            true => Busfaultact::Active,
            false => Busfaultact::Notactive,
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busfaultact::Active
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Busfaultact::Notactive
    }
}
#[doc = "Field `RESERVED2` reader - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::BitReader;
#[doc = "3:3\\]
UsageFault exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgfaultact {
    #[doc = "1: Exception is active"]
    Active = 1,
    #[doc = "0: Exception is not active"]
    Notactive = 0,
}
impl From<Usgfaultact> for bool {
    #[inline(always)]
    fn from(variant: Usgfaultact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTACT` reader - 3:3\\]
UsageFault exception active"]
pub type UsgfaultactR = crate::BitReader<Usgfaultact>;
impl UsgfaultactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usgfaultact {
        match self.bits {
            true => Usgfaultact::Active,
            false => Usgfaultact::Notactive,
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Usgfaultact::Active
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Usgfaultact::Notactive
    }
}
#[doc = "Field `RESERVED4` reader - 6:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "7:7\\]
SVCall active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcallact {
    #[doc = "1: Exception is active"]
    Active = 1,
    #[doc = "0: Exception is not active"]
    Notactive = 0,
}
impl From<Svcallact> for bool {
    #[inline(always)]
    fn from(variant: Svcallact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLACT` reader - 7:7\\]
SVCall active"]
pub type SvcallactR = crate::BitReader<Svcallact>;
impl SvcallactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcallact {
        match self.bits {
            true => Svcallact::Active,
            false => Svcallact::Notactive,
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Svcallact::Active
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Svcallact::Notactive
    }
}
#[doc = "8:8\\]
Debug monitor active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monitoract {
    #[doc = "1: Exception is active"]
    Active = 1,
    #[doc = "0: Exception is not active"]
    Notactive = 0,
}
impl From<Monitoract> for bool {
    #[inline(always)]
    fn from(variant: Monitoract) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONITORACT` reader - 8:8\\]
Debug monitor active"]
pub type MonitoractR = crate::BitReader<Monitoract>;
impl MonitoractR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monitoract {
        match self.bits {
            true => Monitoract::Active,
            false => Monitoract::Notactive,
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Monitoract::Active
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Monitoract::Notactive
    }
}
#[doc = "Field `RESERVED9` reader - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `PENDSVACT` reader - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
pub type PendsvactR = crate::BitReader;
#[doc = "11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systickact {
    #[doc = "1: Exception is active"]
    Active = 1,
    #[doc = "0: Exception is not active"]
    Notactive = 0,
}
impl From<Systickact> for bool {
    #[inline(always)]
    fn from(variant: Systickact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTICKACT` reader - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
pub type SystickactR = crate::BitReader<Systickact>;
impl SystickactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systickact {
        match self.bits {
            true => Systickact::Active,
            false => Systickact::Notactive,
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Systickact::Active
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Systickact::Notactive
    }
}
#[doc = "12:12\\]
Usage fault pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgfaultpended {
    #[doc = "1: Exception is pending."]
    Pending = 1,
    #[doc = "0: Exception is not active"]
    Notpending = 0,
}
impl From<Usgfaultpended> for bool {
    #[inline(always)]
    fn from(variant: Usgfaultpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTPENDED` reader - 12:12\\]
Usage fault pending"]
pub type UsgfaultpendedR = crate::BitReader<Usgfaultpended>;
impl UsgfaultpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usgfaultpended {
        match self.bits {
            true => Usgfaultpended::Pending,
            false => Usgfaultpended::Notpending,
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Usgfaultpended::Pending
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == Usgfaultpended::Notpending
    }
}
#[doc = "13:13\\]
MemManage exception pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memfaultpended {
    #[doc = "1: Exception is pending."]
    Pending = 1,
    #[doc = "0: Exception is not active"]
    Notpending = 0,
}
impl From<Memfaultpended> for bool {
    #[inline(always)]
    fn from(variant: Memfaultpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTPENDED` reader - 13:13\\]
MemManage exception pending"]
pub type MemfaultpendedR = crate::BitReader<Memfaultpended>;
impl MemfaultpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memfaultpended {
        match self.bits {
            true => Memfaultpended::Pending,
            false => Memfaultpended::Notpending,
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Memfaultpended::Pending
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == Memfaultpended::Notpending
    }
}
#[doc = "14:14\\]
BusFault pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busfaultpended {
    #[doc = "1: Exception is pending."]
    Pending = 1,
    #[doc = "0: Exception is not active"]
    Notpending = 0,
}
impl From<Busfaultpended> for bool {
    #[inline(always)]
    fn from(variant: Busfaultpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTPENDED` reader - 14:14\\]
BusFault pending"]
pub type BusfaultpendedR = crate::BitReader<Busfaultpended>;
impl BusfaultpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busfaultpended {
        match self.bits {
            true => Busfaultpended::Pending,
            false => Busfaultpended::Notpending,
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Busfaultpended::Pending
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == Busfaultpended::Notpending
    }
}
#[doc = "15:15\\]
SVCall pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcallpended {
    #[doc = "1: Exception is pending."]
    Pending = 1,
    #[doc = "0: Exception is not active"]
    Notpending = 0,
}
impl From<Svcallpended> for bool {
    #[inline(always)]
    fn from(variant: Svcallpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLPENDED` reader - 15:15\\]
SVCall pending"]
pub type SvcallpendedR = crate::BitReader<Svcallpended>;
impl SvcallpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcallpended {
        match self.bits {
            true => Svcallpended::Pending,
            false => Svcallpended::Notpending,
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Svcallpended::Pending
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == Svcallpended::Notpending
    }
}
#[doc = "16:16\\]
MemManage fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memfaultena {
    #[doc = "1: Exception enabled"]
    En = 1,
    #[doc = "0: Exception disabled"]
    Dis = 0,
}
impl From<Memfaultena> for bool {
    #[inline(always)]
    fn from(variant: Memfaultena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTENA` reader - 16:16\\]
MemManage fault system handler enable"]
pub type MemfaultenaR = crate::BitReader<Memfaultena>;
impl MemfaultenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memfaultena {
        match self.bits {
            true => Memfaultena::En,
            false => Memfaultena::Dis,
        }
    }
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Memfaultena::En
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memfaultena::Dis
    }
}
#[doc = "Field `MEMFAULTENA` writer - 16:16\\]
MemManage fault system handler enable"]
pub type MemfaultenaW<'a, REG> = crate::BitWriter<'a, REG, Memfaultena>;
impl<'a, REG> MemfaultenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultena::En)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memfaultena::Dis)
    }
}
#[doc = "17:17\\]
Bus fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busfaultena {
    #[doc = "1: Exception enabled"]
    En = 1,
    #[doc = "0: Exception disabled"]
    Dis = 0,
}
impl From<Busfaultena> for bool {
    #[inline(always)]
    fn from(variant: Busfaultena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTENA` reader - 17:17\\]
Bus fault system handler enable"]
pub type BusfaultenaR = crate::BitReader<Busfaultena>;
impl BusfaultenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busfaultena {
        match self.bits {
            true => Busfaultena::En,
            false => Busfaultena::Dis,
        }
    }
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Busfaultena::En
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Busfaultena::Dis
    }
}
#[doc = "Field `BUSFAULTENA` writer - 17:17\\]
Bus fault system handler enable"]
pub type BusfaultenaW<'a, REG> = crate::BitWriter<'a, REG, Busfaultena>;
impl<'a, REG> BusfaultenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultena::En)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Busfaultena::Dis)
    }
}
#[doc = "18:18\\]
Usage fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usgfaultena {
    #[doc = "1: Exception enabled"]
    En = 1,
    #[doc = "0: Exception disabled"]
    Dis = 0,
}
impl From<Usgfaultena> for bool {
    #[inline(always)]
    fn from(variant: Usgfaultena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTENA` reader - 18:18\\]
Usage fault system handler enable"]
pub type UsgfaultenaR = crate::BitReader<Usgfaultena>;
impl UsgfaultenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usgfaultena {
        match self.bits {
            true => Usgfaultena::En,
            false => Usgfaultena::Dis,
        }
    }
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Usgfaultena::En
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Usgfaultena::Dis
    }
}
#[doc = "Field `USGFAULTENA` writer - 18:18\\]
Usage fault system handler enable"]
pub type UsgfaultenaW<'a, REG> = crate::BitWriter<'a, REG, Usgfaultena>;
impl<'a, REG> UsgfaultenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultena::En)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Usgfaultena::Dis)
    }
}
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MemManage exception active"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MemfaultactR {
        MemfaultactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BusFault exception active"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BusfaultactR {
        BusfaultactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UsageFault exception active"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> UsgfaultactR {
        UsgfaultactR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
SVCall active"]
    #[inline(always)]
    pub fn svcallact(&self) -> SvcallactR {
        SvcallactR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug monitor active"]
    #[inline(always)]
    pub fn monitoract(&self) -> MonitoractR {
        MonitoractR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PendsvactR {
        PendsvactR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn systickact(&self) -> SystickactR {
        SystickactR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Usage fault pending"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> UsgfaultpendedR {
        UsgfaultpendedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MemManage exception pending"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MemfaultpendedR {
        MemfaultpendedR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
BusFault pending"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BusfaultpendedR {
        BusfaultpendedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
SVCall pending"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SvcallpendedR {
        SvcallpendedR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MemManage fault system handler enable"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MemfaultenaR {
        MemfaultenaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Bus fault system handler enable"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BusfaultenaR {
        BusfaultenaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Usage fault system handler enable"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> UsgfaultenaR {
        UsgfaultenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
MemManage fault system handler enable"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultena(&mut self) -> MemfaultenaW<ShcsrSpec> {
        MemfaultenaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Bus fault system handler enable"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultena(&mut self) -> BusfaultenaW<ShcsrSpec> {
        BusfaultenaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Usage fault system handler enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultena(&mut self) -> UsgfaultenaW<ShcsrSpec> {
        UsgfaultenaW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<ShcsrSpec> {
        Reserved19W::new(self, 19)
    }
}
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShcsrSpec;
impl crate::RegisterSpec for ShcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for ShcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for ShcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for ShcsrSpec {
    const RESET_VALUE: u32 = 0;
}
