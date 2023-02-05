#[doc = "Register `CLKCFG0` reader"]
pub type R = crate::R<Clkcfg0Spec>;
#[doc = "Register `CLKCFG0` writer"]
pub type W = crate::W<Clkcfg0Spec>;
#[doc = "0:0\\]
IP clock configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Gpio> for bool {
    #[inline(always)]
    fn from(variant: Gpio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` reader - 0:0\\]
IP clock configuration"]
pub type GpioR = crate::BitReader<Gpio>;
impl GpioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio {
        match self.bits {
            true => Gpio::ClkEn,
            false => Gpio::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Gpio::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Gpio::ClkDis
    }
}
#[doc = "1:1\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrfd {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Lrfd> for bool {
    #[inline(always)]
    fn from(variant: Lrfd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRFD` reader - 1:1\\]
IP clock configuration"]
pub type LrfdR = crate::BitReader<Lrfd>;
impl LrfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrfd {
        match self.bits {
            true => Lrfd::ClkEn,
            false => Lrfd::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Lrfd::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Lrfd::ClkDis
    }
}
#[doc = "2:2\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart0 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Uart0> for bool {
    #[inline(always)]
    fn from(variant: Uart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - 2:2\\]
IP clock configuration"]
pub type Uart0R = crate::BitReader<Uart0>;
impl Uart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart0 {
        match self.bits {
            true => Uart0::ClkEn,
            false => Uart0::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Uart0::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Uart0::ClkDis
    }
}
#[doc = "Field `RESERVED3` reader - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "6:6\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - 6:6\\]
IP clock configuration"]
pub type I2c0R = crate::BitReader<I2c0>;
impl I2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0 {
        match self.bits {
            true => I2c0::ClkEn,
            false => I2c0::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == I2c0::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == I2c0::ClkDis
    }
}
#[doc = "Field `RESERVED7` reader - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader;
#[doc = "10:10\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - 10:10\\]
IP clock configuration"]
pub type Spi0R = crate::BitReader<Spi0>;
impl Spi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi0 {
        match self.bits {
            true => Spi0::ClkEn,
            false => Spi0::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Spi0::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Spi0::ClkDis
    }
}
#[doc = "Field `RESERVED11` reader - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "14:14\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - 14:14\\]
IP clock configuration"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            true => Adc0::ClkEn,
            false => Adc0::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Adc0::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Adc0::ClkDis
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "16:16\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Laes {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Laes> for bool {
    #[inline(always)]
    fn from(variant: Laes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAES` reader - 16:16\\]
IP clock configuration"]
pub type LaesR = crate::BitReader<Laes>;
impl LaesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Laes {
        match self.bits {
            true => Laes::ClkEn,
            false => Laes::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Laes::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Laes::ClkDis
    }
}
#[doc = "17:17\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - 17:17\\]
IP clock configuration"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            true => Dma::ClkEn,
            false => Dma::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Dma::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Dma::ClkDis
    }
}
#[doc = "Field `RESERVED18` reader - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "27:27\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt0 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Lgpt0> for bool {
    #[inline(always)]
    fn from(variant: Lgpt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT0` reader - 27:27\\]
IP clock configuration"]
pub type Lgpt0R = crate::BitReader<Lgpt0>;
impl Lgpt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt0 {
        match self.bits {
            true => Lgpt0::ClkEn,
            false => Lgpt0::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Lgpt0::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Lgpt0::ClkDis
    }
}
#[doc = "28:28\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt1 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Lgpt1> for bool {
    #[inline(always)]
    fn from(variant: Lgpt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT1` reader - 28:28\\]
IP clock configuration"]
pub type Lgpt1R = crate::BitReader<Lgpt1>;
impl Lgpt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt1 {
        match self.bits {
            true => Lgpt1::ClkEn,
            false => Lgpt1::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Lgpt1::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Lgpt1::ClkDis
    }
}
#[doc = "29:29\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt2 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Lgpt2> for bool {
    #[inline(always)]
    fn from(variant: Lgpt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT2` reader - 29:29\\]
IP clock configuration"]
pub type Lgpt2R = crate::BitReader<Lgpt2>;
impl Lgpt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt2 {
        match self.bits {
            true => Lgpt2::ClkEn,
            false => Lgpt2::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Lgpt2::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Lgpt2::ClkDis
    }
}
#[doc = "30:30\\]
IP clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt3 {
    #[doc = "1: Clock is enabled"]
    ClkEn = 1,
    #[doc = "0: Clock is disabled"]
    ClkDis = 0,
}
impl From<Lgpt3> for bool {
    #[inline(always)]
    fn from(variant: Lgpt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT3` reader - 30:30\\]
IP clock configuration"]
pub type Lgpt3R = crate::BitReader<Lgpt3>;
impl Lgpt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt3 {
        match self.bits {
            true => Lgpt3::ClkEn,
            false => Lgpt3::ClkDis,
        }
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_clk_en(&self) -> bool {
        *self == Lgpt3::ClkEn
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_clk_dis(&self) -> bool {
        *self == Lgpt3::ClkDis
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
IP clock configuration"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
IP clock configuration"]
    #[inline(always)]
    pub fn lrfd(&self) -> LrfdR {
        LrfdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
IP clock configuration"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
IP clock configuration"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
IP clock configuration"]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
IP clock configuration"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
IP clock configuration"]
    #[inline(always)]
    pub fn laes(&self) -> LaesR {
        LaesR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
IP clock configuration"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:26 - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - 27:27\\]
IP clock configuration"]
    #[inline(always)]
    pub fn lgpt0(&self) -> Lgpt0R {
        Lgpt0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
IP clock configuration"]
    #[inline(always)]
    pub fn lgpt1(&self) -> Lgpt1R {
        Lgpt1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
IP clock configuration"]
    #[inline(always)]
    pub fn lgpt2(&self) -> Lgpt2R {
        Lgpt2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
IP clock configuration"]
    #[inline(always)]
    pub fn lgpt3(&self) -> Lgpt3R {
        Lgpt3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Clock Configuration Register 0. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET0 and CLKENCLR0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkcfg0Spec;
impl crate::RegisterSpec for Clkcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg0::R`](R) reader structure"]
impl crate::Readable for Clkcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg0::W`](W) writer structure"]
impl crate::Writable for Clkcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG0 to value 0x01"]
impl crate::Resettable for Clkcfg0Spec {
    const RESET_VALUE: u32 = 0x01;
}
