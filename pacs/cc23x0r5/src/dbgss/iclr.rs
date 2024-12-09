#[doc = "Register `ICLR` reader"]
pub type R = crate::R<IclrSpec>;
#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "0:0\\]
Clears TXIFG interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
    #[doc = "0: Writing a 0 has no effect"]
    Noeff = 0,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` reader - 0:0\\]
Clears TXIFG interrupt"]
pub type TxifgR = crate::BitReader<Txifg>;
impl TxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txifg {
        match self.bits {
            true => Txifg::Clr,
            false => Txifg::Noeff,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txifg::Clr
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Txifg::Noeff
    }
}
#[doc = "Field `TXIFG` writer - 0:0\\]
Clears TXIFG interrupt"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG, Txifg>;
impl<'a, REG> TxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Clr)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Noeff)
    }
}
#[doc = "1:1\\]
Clears RXIFG interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
    #[doc = "0: Writing a 0 has no effect"]
    Noeff = 0,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` reader - 1:1\\]
Clears RXIFG interrupt"]
pub type RxifgR = crate::BitReader<Rxifg>;
impl RxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxifg {
        match self.bits {
            true => Rxifg::Clr,
            false => Rxifg::Noeff,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxifg::Clr
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Rxifg::Noeff
    }
}
#[doc = "Field `RXIFG` writer - 1:1\\]
Clears RXIFG interrupt"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG, Rxifg>;
impl<'a, REG> RxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Clr)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Noeff)
    }
}
#[doc = "2:2\\]
Clears PWRUPIFG interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
    #[doc = "0: Writing a 0 has no effect"]
    Noeff = 0,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` reader - 2:2\\]
Clears PWRUPIFG interrupt"]
pub type PwrupifgR = crate::BitReader<Pwrupifg>;
impl PwrupifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrupifg {
        match self.bits {
            true => Pwrupifg::Clr,
            false => Pwrupifg::Noeff,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrupifg::Clr
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pwrupifg::Noeff
    }
}
#[doc = "Field `PWRUPIFG` writer - 2:2\\]
Clears PWRUPIFG interrupt"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrupifg>;
impl<'a, REG> PwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Clr)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Noeff)
    }
}
#[doc = "3:3\\]
Clears PWRDWNIFG interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "1: Clear interrupt"]
    Clr = 1,
    #[doc = "0: Writing a 0 has no effect"]
    Noeff = 0,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` reader - 3:3\\]
Clears PWRDWNIFG interrupt"]
pub type PwrdwnifgR = crate::BitReader<Pwrdwnifg>;
impl PwrdwnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrdwnifg {
        match self.bits {
            true => Pwrdwnifg::Clr,
            false => Pwrdwnifg::Noeff,
        }
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrdwnifg::Clr
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Pwrdwnifg::Noeff
    }
}
#[doc = "Field `PWRDWNIFG` writer - 3:3\\]
Clears PWRDWNIFG interrupt"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrdwnifg>;
impl<'a, REG> PwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Clr)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Noeff)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clears TXIFG interrupt"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears RXIFG interrupt"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears PWRUPIFG interrupt"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears PWRDWNIFG interrupt"]
    #[inline(always)]
    pub fn pwrdwnifg(&self) -> PwrdwnifgR {
        PwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears TXIFG interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txifg(&mut self) -> TxifgW<IclrSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears RXIFG interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxifg(&mut self) -> RxifgW<IclrSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears PWRUPIFG interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupifg(&mut self) -> PwrupifgW<IclrSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears PWRDWNIFG interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<IclrSpec> {
        PwrdwnifgW::new(self, 3)
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
