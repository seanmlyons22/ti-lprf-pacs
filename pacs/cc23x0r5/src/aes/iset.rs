#[doc = "Register `ISET` reader"]
pub type R = crate::R<IsetSpec>;
#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "0:0\\]
Set ECB Done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbdone {
    #[doc = "1: Set interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ecbdone> for bool {
    #[inline(always)]
    fn from(variant: Ecbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBDONE` writer - 0:0\\]
Set ECB Done interrupt"]
pub type EcbdoneW<'a, REG> = crate::BitWriter<'a, REG, Ecbdone>;
impl<'a, REG> EcbdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdone::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdone::Noeff)
    }
}
#[doc = "1:1\\]
Set ECB Start interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbstart {
    #[doc = "1: Set interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ecbstart> for bool {
    #[inline(always)]
    fn from(variant: Ecbstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBSTART` writer - 1:1\\]
Set ECB Start interrupt"]
pub type EcbstartW<'a, REG> = crate::BitWriter<'a, REG, Ecbstart>;
impl<'a, REG> EcbstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstart::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstart::Noeff)
    }
}
#[doc = "2:2\\]
Set DMA Channel A Done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chadone {
    #[doc = "1: Set interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Chadone> for bool {
    #[inline(always)]
    fn from(variant: Chadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHADONE` writer - 2:2\\]
Set DMA Channel A Done interrupt"]
pub type ChadoneW<'a, REG> = crate::BitWriter<'a, REG, Chadone>;
impl<'a, REG> ChadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Chadone::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Chadone::Noeff)
    }
}
#[doc = "3:3\\]
Set DMA Channel B Done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chbdone {
    #[doc = "1: Set interrupt"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Chbdone> for bool {
    #[inline(always)]
    fn from(variant: Chbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHBDONE` writer - 3:3\\]
Set DMA Channel B Done interrupt"]
pub type ChbdoneW<'a, REG> = crate::BitWriter<'a, REG, Chbdone>;
impl<'a, REG> ChbdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set interrupt"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdone::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdone::Noeff)
    }
}
#[doc = "Field `RESERVED2` writer - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set ECB Done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ecbdone(&mut self) -> EcbdoneW<IsetSpec> {
        EcbdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set ECB Start interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ecbstart(&mut self) -> EcbstartW<IsetSpec> {
        EcbstartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set DMA Channel A Done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn chadone(&mut self) -> ChadoneW<IsetSpec> {
        ChadoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Set DMA Channel B Done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn chbdone(&mut self) -> ChbdoneW<IsetSpec> {
        ChbdoneW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IsetSpec> {
        Reserved2W::new(self, 4)
    }
}
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iset::R`](R) reader structure"]
impl crate::Readable for IsetSpec {}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
