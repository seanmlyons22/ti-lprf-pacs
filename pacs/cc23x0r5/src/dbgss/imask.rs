#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "0:0\\]
TXIFG interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txifg {
    #[doc = "1: Enable Interrupt Mask"]
    En = 1,
    #[doc = "0: Disable Interrupt Mask"]
    Dis = 0,
}
impl From<Txifg> for bool {
    #[inline(always)]
    fn from(variant: Txifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIFG` reader - 0:0\\]
TXIFG interrupt mask"]
pub type TxifgR = crate::BitReader<Txifg>;
impl TxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txifg {
        match self.bits {
            true => Txifg::En,
            false => Txifg::Dis,
        }
    }
    #[doc = "Enable Interrupt Mask"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Txifg::En
    }
    #[doc = "Disable Interrupt Mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Txifg::Dis
    }
}
#[doc = "Field `TXIFG` writer - 0:0\\]
TXIFG interrupt mask"]
pub type TxifgW<'a, REG> = crate::BitWriter<'a, REG, Txifg>;
impl<'a, REG> TxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Interrupt Mask"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::En)
    }
    #[doc = "Disable Interrupt Mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Txifg::Dis)
    }
}
#[doc = "1:1\\]
RXIFG interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxifg {
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
}
impl From<Rxifg> for bool {
    #[inline(always)]
    fn from(variant: Rxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIFG` reader - 1:1\\]
RXIFG interrupt mask"]
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
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Rxifg::Set
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Rxifg::Clr
    }
}
#[doc = "Field `RXIFG` writer - 1:1\\]
RXIFG interrupt mask"]
pub type RxifgW<'a, REG> = crate::BitWriter<'a, REG, Rxifg>;
impl<'a, REG> RxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Set)
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rxifg::Clr)
    }
}
#[doc = "2:2\\]
PWRUPIFG interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrupifg {
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
}
impl From<Pwrupifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUPIFG` reader - 2:2\\]
PWRUPIFG interrupt mask"]
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
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrupifg::Set
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrupifg::Clr
    }
}
#[doc = "Field `PWRUPIFG` writer - 2:2\\]
PWRUPIFG interrupt mask"]
pub type PwrupifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrupifg>;
impl<'a, REG> PwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Set)
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrupifg::Clr)
    }
}
#[doc = "3:3\\]
PWRDWNIFG interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdwnifg {
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    Set = 1,
    #[doc = "0: Interrupt is masked out"]
    Clr = 0,
}
impl From<Pwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: Pwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWNIFG` reader - 3:3\\]
PWRDWNIFG interrupt mask"]
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
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Pwrdwnifg::Set
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Pwrdwnifg::Clr
    }
}
#[doc = "Field `PWRDWNIFG` writer - 3:3\\]
PWRDWNIFG interrupt mask"]
pub type PwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, Pwrdwnifg>;
impl<'a, REG> PwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in MIS will be set"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Set)
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdwnifg::Clr)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TXIFG interrupt mask"]
    #[inline(always)]
    pub fn txifg(&self) -> TxifgR {
        TxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RXIFG interrupt mask"]
    #[inline(always)]
    pub fn rxifg(&self) -> RxifgR {
        RxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
PWRUPIFG interrupt mask"]
    #[inline(always)]
    pub fn pwrupifg(&self) -> PwrupifgR {
        PwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
PWRDWNIFG interrupt mask"]
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
TXIFG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn txifg(&mut self) -> TxifgW<ImaskSpec> {
        TxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RXIFG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxifg(&mut self) -> RxifgW<ImaskSpec> {
        RxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
PWRUPIFG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupifg(&mut self) -> PwrupifgW<ImaskSpec> {
        PwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
PWRDWNIFG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwnifg(&mut self) -> PwrdwnifgW<ImaskSpec> {
        PwrdwnifgW::new(self, 3)
    }
}
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
