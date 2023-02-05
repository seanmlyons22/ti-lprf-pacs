#[doc = "Register `IMSET` reader"]
pub type R = crate::R<ImsetSpec>;
#[doc = "Register `IMSET` writer"]
pub type W = crate::W<ImsetSpec>;
#[doc = "0:0\\]
Set the MIS.TGT mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tgt {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Tgt> for bool {
    #[inline(always)]
    fn from(variant: Tgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGT` writer - 0:0\\]
Set the MIS.TGT mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tgt::NoEffect)
    }
}
#[doc = "1:1\\]
Set the MIS.ZERO mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zero {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Zero> for bool {
    #[inline(always)]
    fn from(variant: Zero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZERO` writer - 1:1\\]
Set the MIS.ZERO mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Zero::NoEffect)
    }
}
#[doc = "2:2\\]
Set the MIS.DBLTRANS mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbltrans {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Dbltrans> for bool {
    #[inline(always)]
    fn from(variant: Dbltrans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLTRANS` writer - 2:2\\]
Set the MIS.DBLTRANS mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dbltrans::NoEffect)
    }
}
#[doc = "3:3\\]
Set the MIS.CNTRCHNG mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntrchng {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Cntrchng> for bool {
    #[inline(always)]
    fn from(variant: Cntrchng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTRCHNG` writer - 3:3\\]
Set the MIS.CNTRCHNG mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Cntrchng::NoEffect)
    }
}
#[doc = "4:4\\]
Set the MIS.DIRCHNG mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirchng {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Dirchng> for bool {
    #[inline(always)]
    fn from(variant: Dirchng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRCHNG` writer - 4:4\\]
Set the MIS.DIRCHNG mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dirchng::NoEffect)
    }
}
#[doc = "5:5\\]
Set the MIS.IDX mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idx {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Idx> for bool {
    #[inline(always)]
    fn from(variant: Idx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDX` writer - 5:5\\]
Set the MIS.IDX mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Idx::NoEffect)
    }
}
#[doc = "6:6\\]
Set the MIS.FAULT mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<Fault> for bool {
    #[inline(always)]
    fn from(variant: Fault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT` writer - 6:6\\]
Set the MIS.FAULT mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::NoEffect)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "8:8\\]
Set the MIS.C0CC mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0cc {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<C0cc> for bool {
    #[inline(always)]
    fn from(variant: C0cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0CC` writer - 8:8\\]
Set the MIS.C0CC mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(C0cc::NoEffect)
    }
}
#[doc = "9:9\\]
Set the MIS.C1CC mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1cc {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<C1cc> for bool {
    #[inline(always)]
    fn from(variant: C1cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1CC` writer - 9:9\\]
Set the MIS.C1CC mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(C1cc::NoEffect)
    }
}
#[doc = "10:10\\]
Set the MIS.C2CC mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2cc {
    #[doc = "1: Set"]
    Set = 1,
    #[doc = "0: No effect"]
    NoEffect = 0,
}
impl From<C2cc> for bool {
    #[inline(always)]
    fn from(variant: C2cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C2CC` writer - 10:10\\]
Set the MIS.C2CC mask."]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(C2cc::NoEffect)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
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
Set the MIS.TGT mask."]
    #[inline(always)]
    #[must_use]
    pub fn tgt(&mut self) -> TgtW<ImsetSpec> {
        TgtW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set the MIS.ZERO mask."]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZeroW<ImsetSpec> {
        ZeroW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set the MIS.DBLTRANS mask."]
    #[inline(always)]
    #[must_use]
    pub fn dbltrans(&mut self) -> DbltransW<ImsetSpec> {
        DbltransW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Set the MIS.CNTRCHNG mask."]
    #[inline(always)]
    #[must_use]
    pub fn cntrchng(&mut self) -> CntrchngW<ImsetSpec> {
        CntrchngW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Set the MIS.DIRCHNG mask."]
    #[inline(always)]
    #[must_use]
    pub fn dirchng(&mut self) -> DirchngW<ImsetSpec> {
        DirchngW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Set the MIS.IDX mask."]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IdxW<ImsetSpec> {
        IdxW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Set the MIS.FAULT mask."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<ImsetSpec> {
        FaultW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Set the MIS.C0CC mask."]
    #[inline(always)]
    #[must_use]
    pub fn c0cc(&mut self) -> C0ccW<ImsetSpec> {
        C0ccW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set the MIS.C1CC mask."]
    #[inline(always)]
    #[must_use]
    pub fn c1cc(&mut self) -> C1ccW<ImsetSpec> {
        C1ccW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Set the MIS.C2CC mask."]
    #[inline(always)]
    #[must_use]
    pub fn c2cc(&mut self) -> C2ccW<ImsetSpec> {
        C2ccW::new(self, 10)
    }
}
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImsetSpec;
impl crate::RegisterSpec for ImsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imset::R`](R) reader structure"]
impl crate::Readable for ImsetSpec {}
#[doc = "`write(|w| ..)` method takes [`imset::W`](W) writer structure"]
impl crate::Writable for ImsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSET to value 0"]
impl crate::Resettable for ImsetSpec {
    const RESET_VALUE: u32 = 0;
}
