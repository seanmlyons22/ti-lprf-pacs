#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txsel {
    #[doc = "3: Transmit FIFO becomes $lt;= 3/4 full"]
    Threequ = 3,
    #[doc = "2: Transmit FIFO becomes $lt;= 1/2 full"]
    Half = 2,
    #[doc = "1: Transmit FIFO becomes $lt;= 1/4 full"]
    Quarter = 1,
}
impl From<Txsel> for u8 {
    #[inline(always)]
    fn from(variant: Txsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txsel {
    type Ux = u8;
}
impl crate::IsEnum for Txsel {}
#[doc = "Field `TXSEL` reader - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
pub type TxselR = crate::FieldReader<Txsel>;
impl TxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txsel> {
        match self.bits {
            3 => Some(Txsel::Threequ),
            2 => Some(Txsel::Half),
            1 => Some(Txsel::Quarter),
            _ => None,
        }
    }
    #[doc = "Transmit FIFO becomes $lt;= 3/4 full"]
    #[inline(always)]
    pub fn is_threequ(&self) -> bool {
        *self == Txsel::Threequ
    }
    #[doc = "Transmit FIFO becomes $lt;= 1/2 full"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Txsel::Half
    }
    #[doc = "Transmit FIFO becomes $lt;= 1/4 full"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == Txsel::Quarter
    }
}
#[doc = "Field `TXSEL` writer - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
pub type TxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txsel>;
impl<'a, REG> TxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit FIFO becomes $lt;= 3/4 full"]
    #[inline(always)]
    pub fn threequ(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Threequ)
    }
    #[doc = "Transmit FIFO becomes $lt;= 1/2 full"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Half)
    }
    #[doc = "Transmit FIFO becomes $lt;= 1/4 full"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Quarter)
    }
}
#[doc = "5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxsel {
    #[doc = "3: Receive FIFO becomes $gt;= 3/4 full"]
    Threequ = 3,
    #[doc = "2: Receive FIFO becomes $gt;= 1/2 full"]
    Half = 2,
    #[doc = "1: Receive FIFO becomes $gt;= 1/4 full"]
    Quarter = 1,
}
impl From<Rxsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxsel {}
#[doc = "Field `RXSEL` reader - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
pub type RxselR = crate::FieldReader<Rxsel>;
impl RxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxsel> {
        match self.bits {
            3 => Some(Rxsel::Threequ),
            2 => Some(Rxsel::Half),
            1 => Some(Rxsel::Quarter),
            _ => None,
        }
    }
    #[doc = "Receive FIFO becomes $gt;= 3/4 full"]
    #[inline(always)]
    pub fn is_threequ(&self) -> bool {
        *self == Rxsel::Threequ
    }
    #[doc = "Receive FIFO becomes $gt;= 1/2 full"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Rxsel::Half
    }
    #[doc = "Receive FIFO becomes $gt;= 1/4 full"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == Rxsel::Quarter
    }
}
#[doc = "Field `RXSEL` writer - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
pub type RxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxsel>;
impl<'a, REG> RxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive FIFO becomes $gt;= 3/4 full"]
    #[inline(always)]
    pub fn threequ(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Threequ)
    }
    #[doc = "Receive FIFO becomes $gt;= 1/2 full"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Half)
    }
    #[doc = "Receive FIFO becomes $gt;= 1/4 full"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Quarter)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn txsel(&self) -> TxselR {
        TxselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn rxsel(&self) -> RxselR {
        RxselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn txsel(&mut self) -> TxselW<IflsSpec> {
        TxselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RxselW<IflsSpec> {
        RxselW::new(self, 3)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<IflsSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Interrupt FIFO Level Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFLS to value 0x12"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0x12;
}
