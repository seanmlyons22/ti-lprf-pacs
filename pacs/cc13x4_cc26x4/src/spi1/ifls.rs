#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "7: Trigger when TX FIFO has >= 1 byte free"]
    Level1 = 7,
    #[doc = "6: Reserved"]
    LvlRes6 = 6,
    #[doc = "5: TX FIFO is empty"]
    LvlEmpty = 5,
    #[doc = "4: Reserved"]
    LvlRes4 = 4,
    #[doc = "3: TX FIFO &lt;= 1/4 empty"]
    Lvl1_4 = 3,
    #[doc = "2: TX FIFO &lt;= 1/2 empty (default)"]
    Lvl1_2 = 2,
    #[doc = "1: TX FIFO &lt;= 3/4 empty"]
    Lvl3_4 = 1,
    #[doc = "0: Reserved"]
    LvlOff = 0,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
#[doc = "Field `TXIFLSEL` reader - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txiflsel {
        match self.bits {
            7 => Txiflsel::Level1,
            6 => Txiflsel::LvlRes6,
            5 => Txiflsel::LvlEmpty,
            4 => Txiflsel::LvlRes4,
            3 => Txiflsel::Lvl1_4,
            2 => Txiflsel::Lvl1_2,
            1 => Txiflsel::Lvl3_4,
            0 => Txiflsel::LvlOff,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger when TX FIFO has >= 1 byte free"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Txiflsel::Level1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == Txiflsel::LvlRes6
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_lvl_empty(&self) -> bool {
        *self == Txiflsel::LvlEmpty
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == Txiflsel::LvlRes4
    }
    #[doc = "TX FIFO &lt;= 1/4 empty"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Txiflsel::Lvl1_4
    }
    #[doc = "TX FIFO &lt;= 1/2 empty (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Txiflsel::Lvl1_2
    }
    #[doc = "TX FIFO &lt;= 3/4 empty"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Txiflsel::Lvl3_4
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == Txiflsel::LvlOff
    }
}
#[doc = "Field `TXIFLSEL` writer - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel, crate::Safe>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when TX FIFO has >= 1 byte free"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Level1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlRes6)
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn lvl_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlEmpty)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlRes4)
    }
    #[doc = "TX FIFO &lt;= 1/4 empty"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1_4)
    }
    #[doc = "TX FIFO &lt;= 1/2 empty (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl1_2)
    }
    #[doc = "TX FIFO &lt;= 3/4 empty"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl3_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::LvlOff)
    }
}
#[doc = "5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "7: Trigger when RX FIFO contains >= 1 byte"]
    Level1 = 7,
    #[doc = "6: Reserved"]
    LvlRes6 = 6,
    #[doc = "5: RX FIFO is full"]
    LvlFull = 5,
    #[doc = "4: Reserved"]
    LvlRes4 = 4,
    #[doc = "3: RX FIFO >= 3/4 full"]
    Lvl3_4 = 3,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    Lvl1_2 = 2,
    #[doc = "1: RX FIFO >= 1/4 full"]
    Lvl1_4 = 1,
    #[doc = "0: Reserved"]
    LvlOff = 0,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
#[doc = "Field `RXIFLSEL` reader - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxiflsel {
        match self.bits {
            7 => Rxiflsel::Level1,
            6 => Rxiflsel::LvlRes6,
            5 => Rxiflsel::LvlFull,
            4 => Rxiflsel::LvlRes4,
            3 => Rxiflsel::Lvl3_4,
            2 => Rxiflsel::Lvl1_2,
            1 => Rxiflsel::Lvl1_4,
            0 => Rxiflsel::LvlOff,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger when RX FIFO contains >= 1 byte"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Rxiflsel::Level1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == Rxiflsel::LvlRes6
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_lvl_full(&self) -> bool {
        *self == Rxiflsel::LvlFull
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == Rxiflsel::LvlRes4
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Rxiflsel::Lvl3_4
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Rxiflsel::Lvl1_2
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Rxiflsel::Lvl1_4
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == Rxiflsel::LvlOff
    }
}
#[doc = "Field `RXIFLSEL` writer - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel, crate::Safe>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when RX FIFO contains >= 1 byte"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Level1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlRes6)
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn lvl_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlFull)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlRes4)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl3_4)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_2)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl1_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::LvlOff)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TxiflselW<IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RxiflselW<IflsSpec> {
        RxiflselW::new(self, 3)
    }
}
#[doc = "The IFLS register is the interrupt FIFO level select register. This register can be used to define the levels at which the TX, RX FIFO interrupt flags are triggered. Out of reset, the TXIFLSEL and RXIFLSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
