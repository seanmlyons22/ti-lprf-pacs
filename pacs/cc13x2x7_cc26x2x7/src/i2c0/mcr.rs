#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpbk {
    #[doc = "1: Enable Test Mode"]
    En = 1,
    #[doc = "0: Disable Test Mode"]
    Dis = 0,
}
impl From<Lpbk> for bool {
    #[inline(always)]
    fn from(variant: Lpbk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBK` reader - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
pub type LpbkR = crate::BitReader<Lpbk>;
impl LpbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpbk {
        match self.bits {
            true => Lpbk::En,
            false => Lpbk::Dis,
        }
    }
    #[doc = "Enable Test Mode"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Lpbk::En
    }
    #[doc = "Disable Test Mode"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lpbk::Dis
    }
}
#[doc = "Field `LPBK` writer - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
pub type LpbkW<'a, REG> = crate::BitWriter<'a, REG, Lpbk>;
impl<'a, REG> LpbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Test Mode"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Lpbk::En)
    }
    #[doc = "Disable Test Mode"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lpbk::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "4:4\\]
I2C master function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mfe {
    #[doc = "1: Master mode is enabled."]
    En = 1,
    #[doc = "0: Master mode is disabled."]
    Dis = 0,
}
impl From<Mfe> for bool {
    #[inline(always)]
    fn from(variant: Mfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFE` reader - 4:4\\]
I2C master function enable"]
pub type MfeR = crate::BitReader<Mfe>;
impl MfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mfe {
        match self.bits {
            true => Mfe::En,
            false => Mfe::Dis,
        }
    }
    #[doc = "Master mode is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mfe::En
    }
    #[doc = "Master mode is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mfe::Dis
    }
}
#[doc = "Field `MFE` writer - 4:4\\]
I2C master function enable"]
pub type MfeW<'a, REG> = crate::BitWriter<'a, REG, Mfe>;
impl<'a, REG> MfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mfe::En)
    }
    #[doc = "Master mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mfe::Dis)
    }
}
#[doc = "5:5\\]
I2C slave function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfe {
    #[doc = "1: Slave mode is enabled."]
    En = 1,
    #[doc = "0: Slave mode is disabled."]
    Dis = 0,
}
impl From<Sfe> for bool {
    #[inline(always)]
    fn from(variant: Sfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFE` reader - 5:5\\]
I2C slave function enable"]
pub type SfeR = crate::BitReader<Sfe>;
impl SfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sfe {
        match self.bits {
            true => Sfe::En,
            false => Sfe::Dis,
        }
    }
    #[doc = "Slave mode is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Sfe::En
    }
    #[doc = "Slave mode is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sfe::Dis
    }
}
#[doc = "Field `SFE` writer - 5:5\\]
I2C slave function enable"]
pub type SfeW<'a, REG> = crate::BitWriter<'a, REG, Sfe>;
impl<'a, REG> SfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Sfe::En)
    }
    #[doc = "Slave mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sfe::Dis)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
    #[inline(always)]
    pub fn lpbk(&self) -> LpbkR {
        LpbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
I2C master function enable"]
    #[inline(always)]
    pub fn mfe(&self) -> MfeR {
        MfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C slave function enable"]
    #[inline(always)]
    pub fn sfe(&self) -> SfeR {
        SfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LpbkW<McrSpec> {
        LpbkW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<McrSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
I2C master function enable"]
    #[inline(always)]
    #[must_use]
    pub fn mfe(&mut self) -> MfeW<McrSpec> {
        MfeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C slave function enable"]
    #[inline(always)]
    #[must_use]
    pub fn sfe(&mut self) -> SfeW<McrSpec> {
        SfeW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<McrSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
