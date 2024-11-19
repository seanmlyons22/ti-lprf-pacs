#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "0:0\\]
Masked interrupt status for TXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "1: TXIFG requests an interrupt service routine"]
    Set = 1,
    #[doc = "0: TXIFG did not request an interrupt service routine"]
    Clr = 0,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` reader - 0:0\\]
Masked interrupt status for TXIFG"]
pub type TxifgR = crate::BitReader<Txifg>;
impl TxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txifg {
        match self.bits {
            true => Txifg::Set,
            false => Txifg::Clr,
        }
    }
    #[doc = "TXIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Txifg::Set
    }
    #[doc = "TXIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Txifg::Clr
    }
}
#[doc = "Field `TXIFG` writer - 0:0\\]
Masked interrupt status for TXIFG"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG, Txifg>;
impl<'a, REG> TxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Set)
    }
    #[doc = "TXIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Clr)
    }
}
#[doc = "1:1\\]
Masked interrupt status for RXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "1: RXIFG requests an interrupt service routine"]
    Set = 1,
    #[doc = "0: RXIFG did not request an interrupt service routine"]
    Clr = 0,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` reader - 1:1\\]
Masked interrupt status for RXIFG"]
pub type RxifgR = crate::BitReader<Rxifg>;
impl RxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxifg {
        match self.bits {
            true => Rxifg::Set,
            false => Rxifg::Clr,
        }
    }
    #[doc = "RXIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxifg::Set
    }
    #[doc = "RXIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxifg::Clr
    }
}
#[doc = "Field `RXIFG` writer - 1:1\\]
Masked interrupt status for RXIFG"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG, Rxifg>;
impl<'a, REG> RxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Set)
    }
    #[doc = "RXIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Clr)
    }
}
#[doc = "2:2\\]
Masked interrupt status for PWRUPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "1: PWRUPIFG requests an interrupt service routine"]
    Set = 1,
    #[doc = "0: PWRUPIFG did not request an interrupt service routine"]
    Clr = 0,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` reader - 2:2\\]
Masked interrupt status for PWRUPIFG"]
pub type PwrupifgR = crate::BitReader<Pwrupifg>;
impl PwrupifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrupifg {
        match self.bits {
            true => Pwrupifg::Set,
            false => Pwrupifg::Clr,
        }
    }
    #[doc = "PWRUPIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrupifg::Set
    }
    #[doc = "PWRUPIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrupifg::Clr
    }
}
#[doc = "Field `PWRUPIFG` writer - 2:2\\]
Masked interrupt status for PWRUPIFG"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrupifg>;
impl<'a, REG> PwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWRUPIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Set)
    }
    #[doc = "PWRUPIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Clr)
    }
}
#[doc = "3:3\\]
Masked interrupt status for PWRDWNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "1: PWRDWNIFG requests an interrupt service routine"]
    Set = 1,
    #[doc = "0: PWRDWNIFG did not request an interrupt service routine"]
    Clr = 0,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` reader - 3:3\\]
Masked interrupt status for PWRDWNIFG"]
pub type PwrdwnifgR = crate::BitReader<Pwrdwnifg>;
impl PwrdwnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrdwnifg {
        match self.bits {
            true => Pwrdwnifg::Set,
            false => Pwrdwnifg::Clr,
        }
    }
    #[doc = "PWRDWNIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrdwnifg::Set
    }
    #[doc = "PWRDWNIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrdwnifg::Clr
    }
}
#[doc = "Field `PWRDWNIFG` writer - 3:3\\]
Masked interrupt status for PWRDWNIFG"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrdwnifg>;
impl<'a, REG> PwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWRDWNIFG requests an interrupt service routine"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Set)
    }
    #[doc = "PWRDWNIFG did not request an interrupt service routine"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Clr)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt status for TXIFG"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt status for RXIFG"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt status for PWRUPIFG"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt status for PWRDWNIFG"]
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
Masked interrupt status for TXIFG"]
    #[inline(always)]
    #[must_use]
    pub fn txifg(&mut self) -> TxifgW<MisSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt status for RXIFG"]
    #[inline(always)]
    #[must_use]
    pub fn rxifg(&mut self) -> RxifgW<MisSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt status for PWRUPIFG"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupifg(&mut self) -> PwrupifgW<MisSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt status for PWRDWNIFG"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<MisSpec> {
        PwrdwnifgW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<MisSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}