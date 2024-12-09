#[doc = "Register `ICLR` reader"]
pub type R = crate::R<IclrSpec>;
#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "0:0\\]
Clear ECB Done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbdone {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
    #[doc = "0: `0`"]
    Noeff = 0,
}
impl From<Ecbdone> for bool {
    #[inline(always)]
    fn from(variant: Ecbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBDONE` writer - 0:0\\]
Clear ECB Done interrupt"]
pub type EcbdoneW<'a, REG> = crate::BitWriter<'a, REG, Ecbdone>;
impl<'a, REG> EcbdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdone::Clr)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbdone::Noeff)
    }
}
#[doc = "1:1\\]
Clear ECB Start interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbstart {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
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
Clear ECB Start interrupt"]
pub type EcbstartW<'a, REG> = crate::BitWriter<'a, REG, Ecbstart>;
impl<'a, REG> EcbstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstart::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbstart::Noeff)
    }
}
#[doc = "2:2\\]
Clear DMA Channel A Done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chadone {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
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
Clear DMA Channel A Done interrupt"]
pub type ChadoneW<'a, REG> = crate::BitWriter<'a, REG, Chadone>;
impl<'a, REG> ChadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Chadone::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Chadone::Noeff)
    }
}
#[doc = "3:3\\]
Clear DMA Channel B Done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chbdone {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
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
Clear DMA Channel B Done interrupt"]
pub type ChbdoneW<'a, REG> = crate::BitWriter<'a, REG, Chbdone>;
impl<'a, REG> ChbdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdone::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Chbdone::Noeff)
    }
}
#[doc = "Field `RESERVED2` writer - "]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear ECB Done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ecbdone(&mut self) -> EcbdoneW<IclrSpec> {
        EcbdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear ECB Start interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ecbstart(&mut self) -> EcbstartW<IclrSpec> {
        EcbstartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear DMA Channel A Done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn chadone(&mut self) -> ChadoneW<IclrSpec> {
        ChadoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear DMA Channel B Done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn chbdone(&mut self) -> ChbdoneW<IclrSpec> {
        ChbdoneW::new(self, 3)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IclrSpec> {
        Reserved2W::new(self, 4)
    }
}
#[doc = "Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iclr::R`](R) reader structure"]
impl crate::Readable for IclrSpec {}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
