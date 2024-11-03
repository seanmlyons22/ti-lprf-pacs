#[doc = "Register `DESCEX0` reader"]
pub type R = crate::R<Descex0Spec>;
#[doc = "Register `DESCEX0` writer"]
pub type W = crate::W<Descex0Spec>;
#[doc = "0:0\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Gpio> for bool {
    #[inline(always)]
    fn from(variant: Gpio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` reader - 0:0\\]
IP status on device"]
pub type GpioR = crate::BitReader<Gpio>;
impl GpioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio {
        match self.bits {
            true => Gpio::IpAvail,
            false => Gpio::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Gpio::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Gpio::IpUnavail
    }
}
#[doc = "Field `GPIO` writer - 0:0\\]
IP status on device"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG, Gpio>;
impl<'a, REG> GpioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::IpUnavail)
    }
}
#[doc = "1:1\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrfd {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Lrfd> for bool {
    #[inline(always)]
    fn from(variant: Lrfd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRFD` reader - 1:1\\]
IP status on device"]
pub type LrfdR = crate::BitReader<Lrfd>;
impl LrfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrfd {
        match self.bits {
            true => Lrfd::IpAvail,
            false => Lrfd::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Lrfd::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Lrfd::IpUnavail
    }
}
#[doc = "Field `LRFD` writer - 1:1\\]
IP status on device"]
pub type LrfdW<'a, REG> = crate::BitWriter<'a, REG, Lrfd>;
impl<'a, REG> LrfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Lrfd::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Lrfd::IpUnavail)
    }
}
#[doc = "2:2\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart0 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Uart0> for bool {
    #[inline(always)]
    fn from(variant: Uart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - 2:2\\]
IP status on device"]
pub type Uart0R = crate::BitReader<Uart0>;
impl Uart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart0 {
        match self.bits {
            true => Uart0::IpAvail,
            false => Uart0::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Uart0::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Uart0::IpUnavail
    }
}
#[doc = "Field `UART0` writer - 2:2\\]
IP status on device"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG, Uart0>;
impl<'a, REG> Uart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::IpUnavail)
    }
}
#[doc = "Field `RESERVED3` reader - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "6:6\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - 6:6\\]
IP status on device"]
pub type I2c0R = crate::BitReader<I2c0>;
impl I2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0 {
        match self.bits {
            true => I2c0::IpAvail,
            false => I2c0::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == I2c0::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == I2c0::IpUnavail
    }
}
#[doc = "Field `I2C0` writer - 6:6\\]
IP status on device"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG, I2c0>;
impl<'a, REG> I2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::IpUnavail)
    }
}
#[doc = "Field `RESERVED7` reader - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `RESERVED7` writer - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "10:10\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - 10:10\\]
IP status on device"]
pub type Spi0R = crate::BitReader<Spi0>;
impl Spi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi0 {
        match self.bits {
            true => Spi0::IpAvail,
            false => Spi0::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Spi0::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Spi0::IpUnavail
    }
}
#[doc = "Field `SPI0` writer - 10:10\\]
IP status on device"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG, Spi0>;
impl<'a, REG> Spi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::IpUnavail)
    }
}
#[doc = "Field `RESERVED11` reader - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader;
#[doc = "Field `RESERVED11` writer - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "14:14\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - 14:14\\]
IP status on device"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            true => Adc0::IpAvail,
            false => Adc0::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Adc0::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Adc0::IpUnavail
    }
}
#[doc = "Field `ADC0` writer - 14:14\\]
IP status on device"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::IpUnavail)
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "16:16\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Laes {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Laes> for bool {
    #[inline(always)]
    fn from(variant: Laes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAES` reader - 16:16\\]
IP status on device"]
pub type LaesR = crate::BitReader<Laes>;
impl LaesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Laes {
        match self.bits {
            true => Laes::IpAvail,
            false => Laes::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Laes::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Laes::IpUnavail
    }
}
#[doc = "Field `LAES` writer - 16:16\\]
IP status on device"]
pub type LaesW<'a, REG> = crate::BitWriter<'a, REG, Laes>;
impl<'a, REG> LaesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Laes::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Laes::IpUnavail)
    }
}
#[doc = "17:17\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - 17:17\\]
IP status on device"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            true => Dma::IpAvail,
            false => Dma::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Dma::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Dma::IpUnavail
    }
}
#[doc = "Field `DMA` writer - 17:17\\]
IP status on device"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::IpUnavail)
    }
}
#[doc = "Field `RESERVED18` reader - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "27:27\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt0 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Lgpt0> for bool {
    #[inline(always)]
    fn from(variant: Lgpt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT0` reader - 27:27\\]
IP status on device"]
pub type Lgpt0R = crate::BitReader<Lgpt0>;
impl Lgpt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt0 {
        match self.bits {
            true => Lgpt0::IpAvail,
            false => Lgpt0::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Lgpt0::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Lgpt0::IpUnavail
    }
}
#[doc = "Field `LGPT0` writer - 27:27\\]
IP status on device"]
pub type Lgpt0W<'a, REG> = crate::BitWriter<'a, REG, Lgpt0>;
impl<'a, REG> Lgpt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt0::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt0::IpUnavail)
    }
}
#[doc = "28:28\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt1 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Lgpt1> for bool {
    #[inline(always)]
    fn from(variant: Lgpt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT1` reader - 28:28\\]
IP status on device"]
pub type Lgpt1R = crate::BitReader<Lgpt1>;
impl Lgpt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt1 {
        match self.bits {
            true => Lgpt1::IpAvail,
            false => Lgpt1::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Lgpt1::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Lgpt1::IpUnavail
    }
}
#[doc = "Field `LGPT1` writer - 28:28\\]
IP status on device"]
pub type Lgpt1W<'a, REG> = crate::BitWriter<'a, REG, Lgpt1>;
impl<'a, REG> Lgpt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt1::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt1::IpUnavail)
    }
}
#[doc = "29:29\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt2 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Lgpt2> for bool {
    #[inline(always)]
    fn from(variant: Lgpt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT2` reader - 29:29\\]
IP status on device"]
pub type Lgpt2R = crate::BitReader<Lgpt2>;
impl Lgpt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt2 {
        match self.bits {
            true => Lgpt2::IpAvail,
            false => Lgpt2::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Lgpt2::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Lgpt2::IpUnavail
    }
}
#[doc = "Field `LGPT2` writer - 29:29\\]
IP status on device"]
pub type Lgpt2W<'a, REG> = crate::BitWriter<'a, REG, Lgpt2>;
impl<'a, REG> Lgpt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt2::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt2::IpUnavail)
    }
}
#[doc = "30:30\\]
IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgpt3 {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Lgpt3> for bool {
    #[inline(always)]
    fn from(variant: Lgpt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGPT3` reader - 30:30\\]
IP status on device"]
pub type Lgpt3R = crate::BitReader<Lgpt3>;
impl Lgpt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgpt3 {
        match self.bits {
            true => Lgpt3::IpAvail,
            false => Lgpt3::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Lgpt3::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Lgpt3::IpUnavail
    }
}
#[doc = "Field `LGPT3` writer - 30:30\\]
IP status on device"]
pub type Lgpt3W<'a, REG> = crate::BitWriter<'a, REG, Lgpt3>;
impl<'a, REG> Lgpt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn ip_avail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt3::IpAvail)
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn ip_unavail(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt3::IpUnavail)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
IP status on device"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
IP status on device"]
    #[inline(always)]
    pub fn lrfd(&self) -> LrfdR {
        LrfdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
IP status on device"]
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
IP status on device"]
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
IP status on device"]
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
IP status on device"]
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
IP status on device"]
    #[inline(always)]
    pub fn laes(&self) -> LaesR {
        LaesR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
IP status on device"]
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
IP status on device"]
    #[inline(always)]
    pub fn lgpt0(&self) -> Lgpt0R {
        Lgpt0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
IP status on device"]
    #[inline(always)]
    pub fn lgpt1(&self) -> Lgpt1R {
        Lgpt1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
IP status on device"]
    #[inline(always)]
    pub fn lgpt2(&self) -> Lgpt2R {
        Lgpt2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
IP status on device"]
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
impl W {
    #[doc = "Bit 0 - 0:0\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GpioW<Descex0Spec> {
        GpioW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn lrfd(&mut self) -> LrfdW<Descex0Spec> {
        LrfdW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> Uart0W<Descex0Spec> {
        Uart0W::new(self, 2)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Descex0Spec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<Descex0Spec> {
        I2c0W::new(self, 6)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<Descex0Spec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 10 - 10:10\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> Spi0W<Descex0Spec> {
        Spi0W::new(self, 10)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<Descex0Spec> {
        Reserved11W::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> Adc0W<Descex0Spec> {
        Adc0W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Descex0Spec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn laes(&mut self) -> LaesW<Descex0Spec> {
        LaesW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Descex0Spec> {
        DmaW::new(self, 17)
    }
    #[doc = "Bits 18:26 - 26:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<Descex0Spec> {
        Reserved18W::new(self, 18)
    }
    #[doc = "Bit 27 - 27:27\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt0(&mut self) -> Lgpt0W<Descex0Spec> {
        Lgpt0W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt1(&mut self) -> Lgpt1W<Descex0Spec> {
        Lgpt1W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt2(&mut self) -> Lgpt2W<Descex0Spec> {
        Lgpt2W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
IP status on device"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt3(&mut self) -> Lgpt3W<Descex0Spec> {
        Lgpt3W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Descex0Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Extended Description Register 0. This register shows SVT IP availability, HW features and memory size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Descex0Spec;
impl crate::RegisterSpec for Descex0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex0::R`](R) reader structure"]
impl crate::Readable for Descex0Spec {}
#[doc = "`write(|w| ..)` method takes [`descex0::W`](W) writer structure"]
impl crate::Writable for Descex0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX0 to value 0x7803_4447"]
impl crate::Resettable for Descex0Spec {
    const RESET_VALUE: u32 = 0x7803_4447;
}
