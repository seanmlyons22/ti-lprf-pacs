#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "0:0\\]
Status of the TGT interrupt. The interrupt is set when CNTR = TGT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgt {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Tgt> for bool {
    #[inline(always)]
    fn from(variant: Tgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGT` reader - 0:0\\]
Status of the TGT interrupt. The interrupt is set when CNTR = TGT."]
pub type TgtR = crate::BitReader<Tgt>;
impl TgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tgt {
        match self.bits {
            true => Tgt::Set,
            false => Tgt::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tgt::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Tgt::Clr
    }
}
#[doc = "Field `TGT` writer - 0:0\\]
Status of the TGT interrupt. The interrupt is set when CNTR = TGT."]
pub type TgtW<'a, REG> = crate::BitWriter<'a, REG, Tgt>;
impl<'a, REG> TgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tgt::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Tgt::Clr)
    }
}
#[doc = "1:1\\]
Status of the ZERO interrupt. The interrupt is set when CNTR = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zero {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Zero> for bool {
    #[inline(always)]
    fn from(variant: Zero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZERO` reader - 1:1\\]
Status of the ZERO interrupt. The interrupt is set when CNTR = 0."]
pub type ZeroR = crate::BitReader<Zero>;
impl ZeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zero {
        match self.bits {
            true => Zero::Set,
            false => Zero::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Zero::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Zero::Clr
    }
}
#[doc = "Field `ZERO` writer - 1:1\\]
Status of the ZERO interrupt. The interrupt is set when CNTR = 0."]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG, Zero>;
impl<'a, REG> ZeroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Zero::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Zero::Clr)
    }
}
#[doc = "2:2\\]
Status of the DBLTRANS interrupt. The interrupt is set when a double transition has happened during QDEC mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbltrans {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Dbltrans> for bool {
    #[inline(always)]
    fn from(variant: Dbltrans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLTRANS` reader - 2:2\\]
Status of the DBLTRANS interrupt. The interrupt is set when a double transition has happened during QDEC mode."]
pub type DbltransR = crate::BitReader<Dbltrans>;
impl DbltransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbltrans {
        match self.bits {
            true => Dbltrans::Set,
            false => Dbltrans::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dbltrans::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dbltrans::Clr
    }
}
#[doc = "Field `DBLTRANS` writer - 2:2\\]
Status of the DBLTRANS interrupt. The interrupt is set when a double transition has happened during QDEC mode."]
pub type DbltransW<'a, REG> = crate::BitWriter<'a, REG, Dbltrans>;
impl<'a, REG> DbltransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dbltrans::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dbltrans::Clr)
    }
}
#[doc = "3:3\\]
Status of the CNTRCHNG interrupt. The interrupt is set when the counter increments or decrements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntrchng {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Cntrchng> for bool {
    #[inline(always)]
    fn from(variant: Cntrchng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTRCHNG` reader - 3:3\\]
Status of the CNTRCHNG interrupt. The interrupt is set when the counter increments or decrements."]
pub type CntrchngR = crate::BitReader<Cntrchng>;
impl CntrchngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntrchng {
        match self.bits {
            true => Cntrchng::Set,
            false => Cntrchng::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cntrchng::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Cntrchng::Clr
    }
}
#[doc = "Field `CNTRCHNG` writer - 3:3\\]
Status of the CNTRCHNG interrupt. The interrupt is set when the counter increments or decrements."]
pub type CntrchngW<'a, REG> = crate::BitWriter<'a, REG, Cntrchng>;
impl<'a, REG> CntrchngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrchng::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrchng::Clr)
    }
}
#[doc = "4:4\\]
Status of the DIRCHNG interrupt. The interrupt is set when the direction of the counter changes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirchng {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Dirchng> for bool {
    #[inline(always)]
    fn from(variant: Dirchng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRCHNG` reader - 4:4\\]
Status of the DIRCHNG interrupt. The interrupt is set when the direction of the counter changes."]
pub type DirchngR = crate::BitReader<Dirchng>;
impl DirchngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dirchng {
        match self.bits {
            true => Dirchng::Set,
            false => Dirchng::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Dirchng::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Dirchng::Clr
    }
}
#[doc = "Field `DIRCHNG` writer - 4:4\\]
Status of the DIRCHNG interrupt. The interrupt is set when the direction of the counter changes."]
pub type DirchngW<'a, REG> = crate::BitWriter<'a, REG, Dirchng>;
impl<'a, REG> DirchngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dirchng::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dirchng::Clr)
    }
}
#[doc = "5:5\\]
Status of the IDX interrupt. The interrupt is set when IDX is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idx {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Idx> for bool {
    #[inline(always)]
    fn from(variant: Idx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDX` reader - 5:5\\]
Status of the IDX interrupt. The interrupt is set when IDX is active."]
pub type IdxR = crate::BitReader<Idx>;
impl IdxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idx {
        match self.bits {
            true => Idx::Set,
            false => Idx::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Idx::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Idx::Clr
    }
}
#[doc = "Field `IDX` writer - 5:5\\]
Status of the IDX interrupt. The interrupt is set when IDX is active."]
pub type IdxW<'a, REG> = crate::BitWriter<'a, REG, Idx>;
impl<'a, REG> IdxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Idx::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Idx::Clr)
    }
}
#[doc = "6:6\\]
Status of the FAULT interrupt. The interrupt is set immediately on active fault input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<Fault> for bool {
    #[inline(always)]
    fn from(variant: Fault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT` reader - 6:6\\]
Status of the FAULT interrupt. The interrupt is set immediately on active fault input."]
pub type FaultR = crate::BitReader<Fault>;
impl FaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault {
        match self.bits {
            true => Fault::Set,
            false => Fault::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Fault::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Fault::Clr
    }
}
#[doc = "Field `FAULT` writer - 6:6\\]
Status of the FAULT interrupt. The interrupt is set immediately on active fault input."]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG, Fault>;
impl<'a, REG> FaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::Clr)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
Status of the C0CC interrupt. The interrupt is set when C0CC has capture or compare event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0cc {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<C0cc> for bool {
    #[inline(always)]
    fn from(variant: C0cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CC` reader - 8:8\\]
Status of the C0CC interrupt. The interrupt is set when C0CC has capture or compare event."]
pub type C0ccR = crate::BitReader<C0cc>;
impl C0ccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0cc {
        match self.bits {
            true => C0cc::Set,
            false => C0cc::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == C0cc::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == C0cc::Clr
    }
}
#[doc = "Field `C0CC` writer - 8:8\\]
Status of the C0CC interrupt. The interrupt is set when C0CC has capture or compare event."]
pub type C0ccW<'a, REG> = crate::BitWriter<'a, REG, C0cc>;
impl<'a, REG> C0ccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(C0cc::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(C0cc::Clr)
    }
}
#[doc = "9:9\\]
Status of the C1CC interrupt. The interrupt is set when C1CC has capture or compare event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1cc {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<C1cc> for bool {
    #[inline(always)]
    fn from(variant: C1cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1CC` reader - 9:9\\]
Status of the C1CC interrupt. The interrupt is set when C1CC has capture or compare event."]
pub type C1ccR = crate::BitReader<C1cc>;
impl C1ccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1cc {
        match self.bits {
            true => C1cc::Set,
            false => C1cc::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == C1cc::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == C1cc::Clr
    }
}
#[doc = "Field `C1CC` writer - 9:9\\]
Status of the C1CC interrupt. The interrupt is set when C1CC has capture or compare event."]
pub type C1ccW<'a, REG> = crate::BitWriter<'a, REG, C1cc>;
impl<'a, REG> C1ccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(C1cc::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(C1cc::Clr)
    }
}
#[doc = "10:10\\]
Status of the C2CC interrupt. The interrupt is set when C2CC has capture or compare event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2cc {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: Cleared"]
    Clr = 0,
}
impl From<C2cc> for bool {
    #[inline(always)]
    fn from(variant: C2cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C2CC` reader - 10:10\\]
Status of the C2CC interrupt. The interrupt is set when C2CC has capture or compare event."]
pub type C2ccR = crate::BitReader<C2cc>;
impl C2ccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C2cc {
        match self.bits {
            true => C2cc::Set,
            false => C2cc::Clr,
        }
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == C2cc::Set
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == C2cc::Clr
    }
}
#[doc = "Field `C2CC` writer - 10:10\\]
Status of the C2CC interrupt. The interrupt is set when C2CC has capture or compare event."]
pub type C2ccW<'a, REG> = crate::BitWriter<'a, REG, C2cc>;
impl<'a, REG> C2ccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(C2cc::Set)
    }
    #[doc = "Cleared"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(C2cc::Clr)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of the TGT interrupt. The interrupt is set when CNTR = TGT."]
    #[inline(always)]
    pub fn tgt(&self) -> TgtR {
        TgtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of the ZERO interrupt. The interrupt is set when CNTR = 0."]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of the DBLTRANS interrupt. The interrupt is set when a double transition has happened during QDEC mode."]
    #[inline(always)]
    pub fn dbltrans(&self) -> DbltransR {
        DbltransR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of the CNTRCHNG interrupt. The interrupt is set when the counter increments or decrements."]
    #[inline(always)]
    pub fn cntrchng(&self) -> CntrchngR {
        CntrchngR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of the DIRCHNG interrupt. The interrupt is set when the direction of the counter changes."]
    #[inline(always)]
    pub fn dirchng(&self) -> DirchngR {
        DirchngR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of the IDX interrupt. The interrupt is set when IDX is active."]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of the FAULT interrupt. The interrupt is set immediately on active fault input."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of the C0CC interrupt. The interrupt is set when C0CC has capture or compare event."]
    #[inline(always)]
    pub fn c0cc(&self) -> C0ccR {
        C0ccR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of the C1CC interrupt. The interrupt is set when C1CC has capture or compare event."]
    #[inline(always)]
    pub fn c1cc(&self) -> C1ccR {
        C1ccR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of the C2CC interrupt. The interrupt is set when C2CC has capture or compare event."]
    #[inline(always)]
    pub fn c2cc(&self) -> C2ccR {
        C2ccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of the TGT interrupt. The interrupt is set when CNTR = TGT."]
    #[inline(always)]
    #[must_use]
    pub fn tgt(&mut self) -> TgtW<RisSpec> {
        TgtW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of the ZERO interrupt. The interrupt is set when CNTR = 0."]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZeroW<RisSpec> {
        ZeroW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of the DBLTRANS interrupt. The interrupt is set when a double transition has happened during QDEC mode."]
    #[inline(always)]
    #[must_use]
    pub fn dbltrans(&mut self) -> DbltransW<RisSpec> {
        DbltransW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of the CNTRCHNG interrupt. The interrupt is set when the counter increments or decrements."]
    #[inline(always)]
    #[must_use]
    pub fn cntrchng(&mut self) -> CntrchngW<RisSpec> {
        CntrchngW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of the DIRCHNG interrupt. The interrupt is set when the direction of the counter changes."]
    #[inline(always)]
    #[must_use]
    pub fn dirchng(&mut self) -> DirchngW<RisSpec> {
        DirchngW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of the IDX interrupt. The interrupt is set when IDX is active."]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IdxW<RisSpec> {
        IdxW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of the FAULT interrupt. The interrupt is set immediately on active fault input."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<RisSpec> {
        FaultW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<RisSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of the C0CC interrupt. The interrupt is set when C0CC has capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn c0cc(&mut self) -> C0ccW<RisSpec> {
        C0ccW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of the C1CC interrupt. The interrupt is set when C1CC has capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn c1cc(&mut self) -> C1ccW<RisSpec> {
        C1ccW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of the C2CC interrupt. The interrupt is set when C2CC has capture or compare event."]
    #[inline(always)]
    #[must_use]
    pub fn c2cc(&mut self) -> C2ccW<RisSpec> {
        C2ccW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<RisSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
