#[doc = "Register `DMABUSCFG` reader"]
pub type R = crate::R<DmabuscfgSpec>;
#[doc = "Register `DMABUSCFG` writer"]
pub type W = crate::W<DmabuscfgSpec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "8:8\\]
Endianess for the AHB master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbMst1Bigend {
    #[doc = "1: Big Endian"]
    BigEndian = 1,
    #[doc = "0: Little Endian"]
    LittleEndian = 0,
}
impl From<AhbMst1Bigend> for bool {
    #[inline(always)]
    fn from(variant: AhbMst1Bigend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_BIGEND` reader - 8:8\\]
Endianess for the AHB master"]
pub type AhbMst1BigendR = crate::BitReader<AhbMst1Bigend>;
impl AhbMst1BigendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbMst1Bigend {
        match self.bits {
            true => AhbMst1Bigend::BigEndian,
            false => AhbMst1Bigend::LittleEndian,
        }
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == AhbMst1Bigend::BigEndian
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == AhbMst1Bigend::LittleEndian
    }
}
#[doc = "Field `AHB_MST1_BIGEND` writer - 8:8\\]
Endianess for the AHB master"]
pub type AhbMst1BigendW<'a, REG> = crate::BitWriter<'a, REG, AhbMst1Bigend>;
impl<'a, REG> AhbMst1BigendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1Bigend::BigEndian)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1Bigend::LittleEndian)
    }
}
#[doc = "9:9\\]
Locked transform on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbMst1LockEn {
    #[doc = "1: Transfers are locked"]
    Locked = 1,
    #[doc = "0: Transfers are not locked"]
    NotLocked = 0,
}
impl From<AhbMst1LockEn> for bool {
    #[inline(always)]
    fn from(variant: AhbMst1LockEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_LOCK_EN` reader - 9:9\\]
Locked transform on AHB"]
pub type AhbMst1LockEnR = crate::BitReader<AhbMst1LockEn>;
impl AhbMst1LockEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbMst1LockEn {
        match self.bits {
            true => AhbMst1LockEn::Locked,
            false => AhbMst1LockEn::NotLocked,
        }
    }
    #[doc = "Transfers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == AhbMst1LockEn::Locked
    }
    #[doc = "Transfers are not locked"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == AhbMst1LockEn::NotLocked
    }
}
#[doc = "Field `AHB_MST1_LOCK_EN` writer - 9:9\\]
Locked transform on AHB"]
pub type AhbMst1LockEnW<'a, REG> = crate::BitWriter<'a, REG, AhbMst1LockEn>;
impl<'a, REG> AhbMst1LockEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfers are locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1LockEn::Locked)
    }
    #[doc = "Transfers are not locked"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1LockEn::NotLocked)
    }
}
#[doc = "10:10\\]
Burst length type of AHB transfer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbMst1IncrEn {
    #[doc = "1: Fixed length bursts or single transfers"]
    Specified = 1,
    #[doc = "0: Unspecified length burst transfers"]
    Unspecified = 0,
}
impl From<AhbMst1IncrEn> for bool {
    #[inline(always)]
    fn from(variant: AhbMst1IncrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_INCR_EN` reader - 10:10\\]
Burst length type of AHB transfer"]
pub type AhbMst1IncrEnR = crate::BitReader<AhbMst1IncrEn>;
impl AhbMst1IncrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbMst1IncrEn {
        match self.bits {
            true => AhbMst1IncrEn::Specified,
            false => AhbMst1IncrEn::Unspecified,
        }
    }
    #[doc = "Fixed length bursts or single transfers"]
    #[inline(always)]
    pub fn is_specified(&self) -> bool {
        *self == AhbMst1IncrEn::Specified
    }
    #[doc = "Unspecified length burst transfers"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == AhbMst1IncrEn::Unspecified
    }
}
#[doc = "Field `AHB_MST1_INCR_EN` writer - 10:10\\]
Burst length type of AHB transfer"]
pub type AhbMst1IncrEnW<'a, REG> = crate::BitWriter<'a, REG, AhbMst1IncrEn>;
impl<'a, REG> AhbMst1IncrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed length bursts or single transfers"]
    #[inline(always)]
    pub fn specified(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1IncrEn::Specified)
    }
    #[doc = "Unspecified length burst transfers"]
    #[inline(always)]
    pub fn unspecified(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1IncrEn::Unspecified)
    }
}
#[doc = "11:11\\]
Idle insertion between consecutive burst transfers on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbMst1IdleEn {
    #[doc = "1: Idle transfer insertion enabled"]
    Idle = 1,
    #[doc = "0: Do not insert idle transfers."]
    NoIdle = 0,
}
impl From<AhbMst1IdleEn> for bool {
    #[inline(always)]
    fn from(variant: AhbMst1IdleEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_IDLE_EN` reader - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
pub type AhbMst1IdleEnR = crate::BitReader<AhbMst1IdleEn>;
impl AhbMst1IdleEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbMst1IdleEn {
        match self.bits {
            true => AhbMst1IdleEn::Idle,
            false => AhbMst1IdleEn::NoIdle,
        }
    }
    #[doc = "Idle transfer insertion enabled"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == AhbMst1IdleEn::Idle
    }
    #[doc = "Do not insert idle transfers."]
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == AhbMst1IdleEn::NoIdle
    }
}
#[doc = "Field `AHB_MST1_IDLE_EN` writer - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
pub type AhbMst1IdleEnW<'a, REG> = crate::BitWriter<'a, REG, AhbMst1IdleEn>;
impl<'a, REG> AhbMst1IdleEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle transfer insertion enabled"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1IdleEn::Idle)
    }
    #[doc = "Do not insert idle transfers."]
    #[inline(always)]
    pub fn no_idle(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1IdleEn::NoIdle)
    }
}
#[doc = "15:12\\]
Maximum burst size that can be performed on the AHB bus\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AhbMst1BurstSize {
    #[doc = "6: 64 bytes"]
    _64Byte = 6,
    #[doc = "5: 32 bytes"]
    _32Byte = 5,
    #[doc = "4: 16 bytes"]
    _16Byte = 4,
    #[doc = "3: 8 bytes"]
    _8Byte = 3,
    #[doc = "2: 4 bytes"]
    _4Byte = 2,
}
impl From<AhbMst1BurstSize> for u8 {
    #[inline(always)]
    fn from(variant: AhbMst1BurstSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AhbMst1BurstSize {
    type Ux = u8;
}
impl crate::IsEnum for AhbMst1BurstSize {}
#[doc = "Field `AHB_MST1_BURST_SIZE` reader - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
pub type AhbMst1BurstSizeR = crate::FieldReader<AhbMst1BurstSize>;
impl AhbMst1BurstSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AhbMst1BurstSize> {
        match self.bits {
            6 => Some(AhbMst1BurstSize::_64Byte),
            5 => Some(AhbMst1BurstSize::_32Byte),
            4 => Some(AhbMst1BurstSize::_16Byte),
            3 => Some(AhbMst1BurstSize::_8Byte),
            2 => Some(AhbMst1BurstSize::_4Byte),
            _ => None,
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == AhbMst1BurstSize::_64Byte
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == AhbMst1BurstSize::_32Byte
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == AhbMst1BurstSize::_16Byte
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == AhbMst1BurstSize::_8Byte
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == AhbMst1BurstSize::_4Byte
    }
}
#[doc = "Field `AHB_MST1_BURST_SIZE` writer - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
pub type AhbMst1BurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, AhbMst1BurstSize>;
impl<'a, REG> AhbMst1BurstSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1BurstSize::_64Byte)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1BurstSize::_32Byte)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1BurstSize::_16Byte)
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1BurstSize::_8Byte)
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut crate::W<REG> {
        self.variant(AhbMst1BurstSize::_4Byte)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AhbMst1BigendR {
        AhbMst1BigendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AhbMst1LockEnR {
        AhbMst1LockEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AhbMst1IncrEnR {
        AhbMst1IncrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AhbMst1IdleEnR {
        AhbMst1IdleEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AhbMst1BurstSizeR {
        AhbMst1BurstSizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<DmabuscfgSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_bigend(&mut self) -> AhbMst1BigendW<DmabuscfgSpec> {
        AhbMst1BigendW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_lock_en(&mut self) -> AhbMst1LockEnW<DmabuscfgSpec> {
        AhbMst1LockEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_incr_en(&mut self) -> AhbMst1IncrEnW<DmabuscfgSpec> {
        AhbMst1IncrEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_idle_en(&mut self) -> AhbMst1IdleEnW<DmabuscfgSpec> {
        AhbMst1IdleEnW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_burst_size(&mut self) -> AhbMst1BurstSizeW<DmabuscfgSpec> {
        AhbMst1BurstSizeW::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<DmabuscfgSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabuscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabuscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmabuscfgSpec;
impl crate::RegisterSpec for DmabuscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabuscfg::R`](R) reader structure"]
impl crate::Readable for DmabuscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dmabuscfg::W`](W) writer structure"]
impl crate::Writable for DmabuscfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABUSCFG to value 0x2400"]
impl crate::Resettable for DmabuscfgSpec {
    const RESET_VALUE: u32 = 0x2400;
}
