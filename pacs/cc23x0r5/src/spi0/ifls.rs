#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "2:0\\]
Transmit FIFO Level Select. The trigger points for the transmit interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txsel {
    #[doc = "7: Trigger when TX FIFO has $gt;= 1 byte free"]
    Level1 = 7,
    #[doc = "6: Reserved"]
    LvlRes6 = 6,
    #[doc = "5: TX FIFO is empty"]
    LvlEmpty = 5,
    #[doc = "4: Reserved"]
    LvlRes4 = 4,
    #[doc = "3: TX FIFO $lt;= 1/4 empty"]
    Lvl1_4 = 3,
    #[doc = "2: TX FIFO $lt;= 1/2 empty (default)"]
    Lvl1_2 = 2,
    #[doc = "1: TX FIFO $lt;= 3/4 empty"]
    Lvl3_4 = 1,
    #[doc = "0: Reserved"]
    LvlOff = 0,
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
Transmit FIFO Level Select. The trigger points for the transmit interrupt are as follows:"]
pub type TxselR = crate::FieldReader<Txsel>;
impl TxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txsel {
        match self.bits {
            7 => Txsel::Level1,
            6 => Txsel::LvlRes6,
            5 => Txsel::LvlEmpty,
            4 => Txsel::LvlRes4,
            3 => Txsel::Lvl1_4,
            2 => Txsel::Lvl1_2,
            1 => Txsel::Lvl3_4,
            0 => Txsel::LvlOff,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger when TX FIFO has $gt;= 1 byte free"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Txsel::Level1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == Txsel::LvlRes6
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_lvl_empty(&self) -> bool {
        *self == Txsel::LvlEmpty
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == Txsel::LvlRes4
    }
    #[doc = "TX FIFO $lt;= 1/4 empty"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Txsel::Lvl1_4
    }
    #[doc = "TX FIFO $lt;= 1/2 empty (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Txsel::Lvl1_2
    }
    #[doc = "TX FIFO $lt;= 3/4 empty"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Txsel::Lvl3_4
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == Txsel::LvlOff
    }
}
#[doc = "Field `TXSEL` writer - 2:0\\]
Transmit FIFO Level Select. The trigger points for the transmit interrupt are as follows:"]
pub type TxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txsel, crate::Safe>;
impl<'a, REG> TxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when TX FIFO has $gt;= 1 byte free"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Level1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::LvlRes6)
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn lvl_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::LvlEmpty)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::LvlRes4)
    }
    #[doc = "TX FIFO $lt;= 1/4 empty"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Lvl1_4)
    }
    #[doc = "TX FIFO $lt;= 1/2 empty (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Lvl1_2)
    }
    #[doc = "TX FIFO $lt;= 3/4 empty"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::Lvl3_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(Txsel::LvlOff)
    }
}
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "10:8\\]
Receive FIFO Level Select. The trigger points for the receive interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxsel {
    #[doc = "7: Trigger when RX FIFO contains $gt;= 1 byte"]
    Level1 = 7,
    #[doc = "6: Reserved"]
    LvlRes6 = 6,
    #[doc = "5: RX FIFO is full"]
    LvlFull = 5,
    #[doc = "4: Reserved"]
    LvlRes4 = 4,
    #[doc = "3: RX FIFO $gt;= 3/4 full"]
    Lvl3_4 = 3,
    #[doc = "2: RX FIFO $gt;= 1/2 full (default)"]
    Lvl1_2 = 2,
    #[doc = "1: RX FIFO $gt;= 1/4 full"]
    Lvl1_4 = 1,
    #[doc = "0: Reserved"]
    LvlOff = 0,
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
#[doc = "Field `RXSEL` reader - 10:8\\]
Receive FIFO Level Select. The trigger points for the receive interrupt are as follows:"]
pub type RxselR = crate::FieldReader<Rxsel>;
impl RxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxsel {
        match self.bits {
            7 => Rxsel::Level1,
            6 => Rxsel::LvlRes6,
            5 => Rxsel::LvlFull,
            4 => Rxsel::LvlRes4,
            3 => Rxsel::Lvl3_4,
            2 => Rxsel::Lvl1_2,
            1 => Rxsel::Lvl1_4,
            0 => Rxsel::LvlOff,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger when RX FIFO contains $gt;= 1 byte"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Rxsel::Level1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == Rxsel::LvlRes6
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_lvl_full(&self) -> bool {
        *self == Rxsel::LvlFull
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == Rxsel::LvlRes4
    }
    #[doc = "RX FIFO $gt;= 3/4 full"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == Rxsel::Lvl3_4
    }
    #[doc = "RX FIFO $gt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == Rxsel::Lvl1_2
    }
    #[doc = "RX FIFO $gt;= 1/4 full"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == Rxsel::Lvl1_4
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == Rxsel::LvlOff
    }
}
#[doc = "Field `RXSEL` writer - 10:8\\]
Receive FIFO Level Select. The trigger points for the receive interrupt are as follows:"]
pub type RxselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxsel, crate::Safe>;
impl<'a, REG> RxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger when RX FIFO contains $gt;= 1 byte"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Level1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::LvlRes6)
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn lvl_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::LvlFull)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::LvlRes4)
    }
    #[doc = "RX FIFO $gt;= 3/4 full"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Lvl3_4)
    }
    #[doc = "RX FIFO $gt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Lvl1_2)
    }
    #[doc = "RX FIFO $gt;= 1/4 full"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::Lvl1_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut crate::W<REG> {
        self.variant(Rxsel::LvlOff)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Transmit FIFO Level Select. The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn txsel(&self) -> TxselR {
        TxselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Receive FIFO Level Select. The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn rxsel(&self) -> RxselR {
        RxselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Transmit FIFO Level Select. The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn txsel(&mut self) -> TxselW<IflsSpec> {
        TxselW::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<IflsSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Receive FIFO Level Select. The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RxselW<IflsSpec> {
        RxselW::new(self, 8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<IflsSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "Interrupt FIFO level select register. This register can be used to define the levels at which the RIS.TX, RIS.RX flags are triggered. The interrupts are generated based on FIFO level. Out of reset, the IFLS.TXSEL and IFLS.RXSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets IFLS to value 0x0202"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0x0202;
}
