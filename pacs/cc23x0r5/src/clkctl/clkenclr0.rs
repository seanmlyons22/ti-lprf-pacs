#[doc = "Register `CLKENCLR0` reader"]
pub type R = crate::R<Clkenclr0Spec>;
#[doc = "Register `CLKENCLR0` writer"]
pub type W = crate::W<Clkenclr0Spec>;
#[doc = "0:0\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Gpio> for bool {
    #[inline(always)]
    fn from(variant: Gpio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` writer - 0:0\\]
Configure IP clock enable"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG, Gpio>;
impl<'a, REG> GpioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::ClkUnchgd)
    }
}
#[doc = "1:1\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrfd {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Lrfd> for bool {
    #[inline(always)]
    fn from(variant: Lrfd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRFD` writer - 1:1\\]
Configure IP clock enable"]
pub type LrfdW<'a, REG> = crate::BitWriter<'a, REG, Lrfd>;
impl<'a, REG> LrfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lrfd::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Lrfd::ClkUnchgd)
    }
}
#[doc = "2:2\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart0 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Uart0> for bool {
    #[inline(always)]
    fn from(variant: Uart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` writer - 2:2\\]
Configure IP clock enable"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG, Uart0>;
impl<'a, REG> Uart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::ClkUnchgd)
    }
}
#[doc = "Field `RESERVED3` reader - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "6:6\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` writer - 6:6\\]
Configure IP clock enable"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG, I2c0>;
impl<'a, REG> I2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::ClkUnchgd)
    }
}
#[doc = "Field `RESERVED7` reader - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader;
#[doc = "10:10\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` writer - 10:10\\]
Configure IP clock enable"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG, Spi0>;
impl<'a, REG> Spi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::ClkUnchgd)
    }
}
#[doc = "Field `RESERVED11` reader - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "14:14\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` writer - 14:14\\]
Configure IP clock enable"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::ClkUnchgd)
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "16:16\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Laes {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Laes> for bool {
    #[inline(always)]
    fn from(variant: Laes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAES` writer - 16:16\\]
Configure IP clock enable"]
pub type LaesW<'a, REG> = crate::BitWriter<'a, REG, Laes>;
impl<'a, REG> LaesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Laes::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Laes::ClkUnchgd)
    }
}
#[doc = "17:17\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` writer - 17:17\\]
Configure IP clock enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::ClkUnchgd)
    }
}
#[doc = "Field `RESERVED18` reader - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "27:27\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt0 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Lgpt0> for bool {
    #[inline(always)]
    fn from(variant: Lgpt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT0` writer - 27:27\\]
Configure IP clock enable"]
pub type Lgpt0W<'a, REG> = crate::BitWriter<'a, REG, Lgpt0>;
impl<'a, REG> Lgpt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt0::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt0::ClkUnchgd)
    }
}
#[doc = "28:28\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt1 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Lgpt1> for bool {
    #[inline(always)]
    fn from(variant: Lgpt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT1` writer - 28:28\\]
Configure IP clock enable"]
pub type Lgpt1W<'a, REG> = crate::BitWriter<'a, REG, Lgpt1>;
impl<'a, REG> Lgpt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt1::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt1::ClkUnchgd)
    }
}
#[doc = "29:29\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt2 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Lgpt2> for bool {
    #[inline(always)]
    fn from(variant: Lgpt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT2` writer - 29:29\\]
Configure IP clock enable"]
pub type Lgpt2W<'a, REG> = crate::BitWriter<'a, REG, Lgpt2>;
impl<'a, REG> Lgpt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt2::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt2::ClkUnchgd)
    }
}
#[doc = "30:30\\]
Configure IP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt3 {
    #[doc = "1: Clear IP clock enable"]
    ClkClr = 1,
    #[doc = "0: IP clock enable is unchanged"]
    ClkUnchgd = 0,
}
impl From<Lgpt3> for bool {
    #[inline(always)]
    fn from(variant: Lgpt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT3` writer - 30:30\\]
Configure IP clock enable"]
pub type Lgpt3W<'a, REG> = crate::BitWriter<'a, REG, Lgpt3>;
impl<'a, REG> Lgpt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IP clock enable"]
    #[inline(always)]
    pub fn clk_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt3::ClkClr)
    }
    #[doc = "IP clock enable is unchanged"]
    #[inline(always)]
    pub fn clk_unchgd(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt3::ClkUnchgd)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
impl R {
    #[doc = "Bits 3:5 - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:26 - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GpioW<Clkenclr0Spec> {
        GpioW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrfd(&mut self) -> LrfdW<Clkenclr0Spec> {
        LrfdW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> Uart0W<Clkenclr0Spec> {
        Uart0W::new(self, 2)
    }
    #[doc = "Bit 6 - 6:6\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<Clkenclr0Spec> {
        I2c0W::new(self, 6)
    }
    #[doc = "Bit 10 - 10:10\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> Spi0W<Clkenclr0Spec> {
        Spi0W::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> Adc0W<Clkenclr0Spec> {
        Adc0W::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn laes(&mut self) -> LaesW<Clkenclr0Spec> {
        LaesW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Clkenclr0Spec> {
        DmaW::new(self, 17)
    }
    #[doc = "Bit 27 - 27:27\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt0(&mut self) -> Lgpt0W<Clkenclr0Spec> {
        Lgpt0W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt1(&mut self) -> Lgpt1W<Clkenclr0Spec> {
        Lgpt1W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt2(&mut self) -> Lgpt2W<Clkenclr0Spec> {
        Lgpt2W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Configure IP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt3(&mut self) -> Lgpt3W<Clkenclr0Spec> {
        Lgpt3W::new(self, 30)
    }
}
#[doc = "Clock Enable Clear Register 0. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG0 to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkenclr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkenclr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkenclr0Spec;
impl crate::RegisterSpec for Clkenclr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkenclr0::R`](R) reader structure"]
impl crate::Readable for Clkenclr0Spec {}
#[doc = "`write(|w| ..)` method takes [`clkenclr0::W`](W) writer structure"]
impl crate::Writable for Clkenclr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKENCLR0 to value 0"]
impl crate::Resettable for Clkenclr0Spec {
    const RESET_VALUE: u32 = 0;
}
