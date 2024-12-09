#[doc = "Register `PARK` reader"]
pub type R = crate::R<ParkSpec>;
#[doc = "Register `PARK` writer"]
pub type W = crate::W<ParkSpec>;
#[doc = "1:0\\]
Park Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl {
    #[doc = "3: Enter parkmode on fault or debug."]
    Both = 3,
    #[doc = "2: Enter park mode on debug."]
    Debug = 2,
    #[doc = "1: Enter park mode on fault."]
    Fault = 1,
    #[doc = "0: Disable park mode."]
    Dis = 0,
}
impl From<Ctl> for u8 {
    #[inline(always)]
    fn from(variant: Ctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl {
    type Ux = u8;
}
impl crate::IsEnum for Ctl {}
#[doc = "Field `CTL` reader - 1:0\\]
Park Control."]
pub type CtlR = crate::FieldReader<Ctl>;
impl CtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl {
        match self.bits {
            3 => Ctl::Both,
            2 => Ctl::Debug,
            1 => Ctl::Fault,
            0 => Ctl::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Enter parkmode on fault or debug."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Ctl::Both
    }
    #[doc = "Enter park mode on debug."]
    #[inline(always)]
    pub fn is_debug(&self) -> bool {
        *self == Ctl::Debug
    }
    #[doc = "Enter park mode on fault."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Ctl::Fault
    }
    #[doc = "Disable park mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ctl::Dis
    }
}
#[doc = "Field `CTL` writer - 1:0\\]
Park Control."]
pub type CtlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl, crate::Safe>;
impl<'a, REG> CtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enter parkmode on fault or debug."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Both)
    }
    #[doc = "Enter park mode on debug."]
    #[inline(always)]
    pub fn debug(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Debug)
    }
    #[doc = "Enter park mode on fault."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Fault)
    }
    #[doc = "Disable park mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Dis)
    }
}
#[doc = "2:2\\]
IO Park State 0 Park state for IO output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iops0 {
    #[doc = "1: Output is set high in park mode."]
    High = 1,
    #[doc = "0: Output is set low in park mode."]
    Low = 0,
}
impl From<Iops0> for bool {
    #[inline(always)]
    fn from(variant: Iops0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPS0` reader - 2:2\\]
IO Park State 0 Park state for IO output 0."]
pub type Iops0R = crate::BitReader<Iops0>;
impl Iops0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iops0 {
        match self.bits {
            true => Iops0::High,
            false => Iops0::Low,
        }
    }
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iops0::High
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iops0::Low
    }
}
#[doc = "Field `IOPS0` writer - 2:2\\]
IO Park State 0 Park state for IO output 0."]
pub type Iops0W<'a, REG> = crate::BitWriter<'a, REG, Iops0>;
impl<'a, REG> Iops0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iops0::High)
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iops0::Low)
    }
}
#[doc = "3:3\\]
IO Complementary Park State 0 Park state for IO Complementary output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocps0 {
    #[doc = "1: Output is set high in park mode."]
    High = 1,
    #[doc = "0: Output is set low in park mode."]
    Low = 0,
}
impl From<Iocps0> for bool {
    #[inline(always)]
    fn from(variant: Iocps0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCPS0` reader - 3:3\\]
IO Complementary Park State 0 Park state for IO Complementary output 0."]
pub type Iocps0R = crate::BitReader<Iocps0>;
impl Iocps0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocps0 {
        match self.bits {
            true => Iocps0::High,
            false => Iocps0::Low,
        }
    }
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iocps0::High
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iocps0::Low
    }
}
#[doc = "Field `IOCPS0` writer - 3:3\\]
IO Complementary Park State 0 Park state for IO Complementary output 0."]
pub type Iocps0W<'a, REG> = crate::BitWriter<'a, REG, Iocps0>;
impl<'a, REG> Iocps0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iocps0::High)
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iocps0::Low)
    }
}
#[doc = "4:4\\]
IO Park State 1 Park state for IO output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iops1 {
    #[doc = "1: Output is set high in park mode."]
    High = 1,
    #[doc = "0: Output is set low in park mode."]
    Low = 0,
}
impl From<Iops1> for bool {
    #[inline(always)]
    fn from(variant: Iops1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPS1` reader - 4:4\\]
IO Park State 1 Park state for IO output 1."]
pub type Iops1R = crate::BitReader<Iops1>;
impl Iops1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iops1 {
        match self.bits {
            true => Iops1::High,
            false => Iops1::Low,
        }
    }
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iops1::High
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iops1::Low
    }
}
#[doc = "Field `IOPS1` writer - 4:4\\]
IO Park State 1 Park state for IO output 1."]
pub type Iops1W<'a, REG> = crate::BitWriter<'a, REG, Iops1>;
impl<'a, REG> Iops1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iops1::High)
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iops1::Low)
    }
}
#[doc = "5:5\\]
IO Complementary Park State 1 Park state for IO Complementary output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocps1 {
    #[doc = "1: Output is set high in park mode."]
    High = 1,
    #[doc = "0: Output is set low in park mode."]
    Low = 0,
}
impl From<Iocps1> for bool {
    #[inline(always)]
    fn from(variant: Iocps1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCPS1` reader - 5:5\\]
IO Complementary Park State 1 Park state for IO Complementary output 1."]
pub type Iocps1R = crate::BitReader<Iocps1>;
impl Iocps1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocps1 {
        match self.bits {
            true => Iocps1::High,
            false => Iocps1::Low,
        }
    }
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iocps1::High
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iocps1::Low
    }
}
#[doc = "Field `IOCPS1` writer - 5:5\\]
IO Complementary Park State 1 Park state for IO Complementary output 1."]
pub type Iocps1W<'a, REG> = crate::BitWriter<'a, REG, Iocps1>;
impl<'a, REG> Iocps1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iocps1::High)
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iocps1::Low)
    }
}
#[doc = "6:6\\]
IO Park State 2 Park state for IO output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iops2 {
    #[doc = "1: Output is set high in park mode."]
    High = 1,
    #[doc = "0: Output is set low in park mode."]
    Low = 0,
}
impl From<Iops2> for bool {
    #[inline(always)]
    fn from(variant: Iops2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPS2` reader - 6:6\\]
IO Park State 2 Park state for IO output 2."]
pub type Iops2R = crate::BitReader<Iops2>;
impl Iops2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iops2 {
        match self.bits {
            true => Iops2::High,
            false => Iops2::Low,
        }
    }
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iops2::High
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iops2::Low
    }
}
#[doc = "Field `IOPS2` writer - 6:6\\]
IO Park State 2 Park state for IO output 2."]
pub type Iops2W<'a, REG> = crate::BitWriter<'a, REG, Iops2>;
impl<'a, REG> Iops2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iops2::High)
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iops2::Low)
    }
}
#[doc = "7:7\\]
IO Complementary Park State 2 Park state for IO Complementary output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocps2 {
    #[doc = "1: Output is set high in park mode."]
    High = 1,
    #[doc = "0: Output is set low in park mode."]
    Low = 0,
}
impl From<Iocps2> for bool {
    #[inline(always)]
    fn from(variant: Iocps2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCPS2` reader - 7:7\\]
IO Complementary Park State 2 Park state for IO Complementary output 2."]
pub type Iocps2R = crate::BitReader<Iocps2>;
impl Iocps2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocps2 {
        match self.bits {
            true => Iocps2::High,
            false => Iocps2::Low,
        }
    }
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iocps2::High
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iocps2::Low
    }
}
#[doc = "Field `IOCPS2` writer - 7:7\\]
IO Complementary Park State 2 Park state for IO Complementary output 2."]
pub type Iocps2W<'a, REG> = crate::BitWriter<'a, REG, Iocps2>;
impl<'a, REG> Iocps2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output is set high in park mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iocps2::High)
    }
    #[doc = "Output is set low in park mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iocps2::Low)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Park Control."]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
IO Park State 0 Park state for IO output 0."]
    #[inline(always)]
    pub fn iops0(&self) -> Iops0R {
        Iops0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
IO Complementary Park State 0 Park state for IO Complementary output 0."]
    #[inline(always)]
    pub fn iocps0(&self) -> Iocps0R {
        Iocps0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
IO Park State 1 Park state for IO output 1."]
    #[inline(always)]
    pub fn iops1(&self) -> Iops1R {
        Iops1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
IO Complementary Park State 1 Park state for IO Complementary output 1."]
    #[inline(always)]
    pub fn iocps1(&self) -> Iocps1R {
        Iocps1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
IO Park State 2 Park state for IO output 2."]
    #[inline(always)]
    pub fn iops2(&self) -> Iops2R {
        Iops2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
IO Complementary Park State 2 Park state for IO Complementary output 2."]
    #[inline(always)]
    pub fn iocps2(&self) -> Iocps2R {
        Iocps2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Park Control."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CtlW<ParkSpec> {
        CtlW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
IO Park State 0 Park state for IO output 0."]
    #[inline(always)]
    #[must_use]
    pub fn iops0(&mut self) -> Iops0W<ParkSpec> {
        Iops0W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
IO Complementary Park State 0 Park state for IO Complementary output 0."]
    #[inline(always)]
    #[must_use]
    pub fn iocps0(&mut self) -> Iocps0W<ParkSpec> {
        Iocps0W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
IO Park State 1 Park state for IO output 1."]
    #[inline(always)]
    #[must_use]
    pub fn iops1(&mut self) -> Iops1W<ParkSpec> {
        Iops1W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
IO Complementary Park State 1 Park state for IO Complementary output 1."]
    #[inline(always)]
    #[must_use]
    pub fn iocps1(&mut self) -> Iocps1W<ParkSpec> {
        Iocps1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
IO Park State 2 Park state for IO output 2."]
    #[inline(always)]
    #[must_use]
    pub fn iops2(&mut self) -> Iops2W<ParkSpec> {
        Iops2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
IO Complementary Park State 2 Park state for IO Complementary output 2."]
    #[inline(always)]
    #[must_use]
    pub fn iocps2(&mut self) -> Iocps2W<ParkSpec> {
        Iocps2W::new(self, 7)
    }
}
#[doc = "Park This register configures how the outputs should be set in Park mode. Park mode is either entered by debug halt or fault. Park mode is activated when the counter stops. Park mode is inactive when the counter starts. When park mode is active all outputs are set to their predefined states. For IO output signals which have enabled dead band, a dead band insertion will be done before switching to the predefined state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`park::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`park::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParkSpec;
impl crate::RegisterSpec for ParkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`park::R`](R) reader structure"]
impl crate::Readable for ParkSpec {}
#[doc = "`write(|w| ..)` method takes [`park::W`](W) writer structure"]
impl crate::Writable for ParkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARK to value 0"]
impl crate::Resettable for ParkSpec {
    const RESET_VALUE: u32 = 0;
}
