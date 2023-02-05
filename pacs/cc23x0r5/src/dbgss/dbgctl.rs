#[doc = "Register `DBGCTL` reader"]
pub type R = crate::R<DbgctlSpec>;
#[doc = "Register `DBGCTL` writer"]
pub type W = crate::W<DbgctlSpec>;
#[doc = "0:0\\]
This bit is used for connecting to IO pads to SWCLK/IO on SW-DP through a software request and establish SWD connection without icemelter trigger for debug purpose.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swdovr {
    #[doc = "1: Force 1 or debug enable mode in which SWD connection is established bypassing ICEMelter sequence"]
    Dbgena = 1,
    #[doc = "0: Transparent mode in which SWD connection is established via ICEMelter Sequence."]
    Trnsprt = 0,
}
impl From<Swdovr> for bool {
    #[inline(always)]
    fn from(variant: Swdovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDOVR` reader - 0:0\\]
This bit is used for connecting to IO pads to SWCLK/IO on SW-DP through a software request and establish SWD connection without icemelter trigger for debug purpose."]
pub type SwdovrR = crate::BitReader<Swdovr>;
impl SwdovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swdovr {
        match self.bits {
            true => Swdovr::Dbgena,
            false => Swdovr::Trnsprt,
        }
    }
    #[doc = "Force 1 or debug enable mode in which SWD connection is established bypassing ICEMelter sequence"]
    #[inline(always)]
    pub fn is_dbgena(&self) -> bool {
        *self == Swdovr::Dbgena
    }
    #[doc = "Transparent mode in which SWD connection is established via ICEMelter Sequence."]
    #[inline(always)]
    pub fn is_trnsprt(&self) -> bool {
        *self == Swdovr::Trnsprt
    }
}
#[doc = "Field `SWDOVR` writer - 0:0\\]
This bit is used for connecting to IO pads to SWCLK/IO on SW-DP through a software request and establish SWD connection without icemelter trigger for debug purpose."]
pub type SwdovrW<'a, REG> = crate::BitWriter<'a, REG, Swdovr>;
impl<'a, REG> SwdovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force 1 or debug enable mode in which SWD connection is established bypassing ICEMelter sequence"]
    #[inline(always)]
    pub fn dbgena(self) -> &'a mut crate::W<REG> {
        self.variant(Swdovr::Dbgena)
    }
    #[doc = "Transparent mode in which SWD connection is established via ICEMelter Sequence."]
    #[inline(always)]
    pub fn trnsprt(self) -> &'a mut crate::W<REG> {
        self.variant(Swdovr::Trnsprt)
    }
}
#[doc = "1:1\\]
This bit field specifies the status of SWD MODE for connection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swdsel {
    #[doc = "1: debug connection enabled."]
    En = 1,
    #[doc = "0: debug connection disabled."]
    Dis = 0,
}
impl From<Swdsel> for bool {
    #[inline(always)]
    fn from(variant: Swdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDSEL` reader - 1:1\\]
This bit field specifies the status of SWD MODE for connection."]
pub type SwdselR = crate::BitReader<Swdsel>;
impl SwdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swdsel {
        match self.bits {
            true => Swdsel::En,
            false => Swdsel::Dis,
        }
    }
    #[doc = "debug connection enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Swdsel::En
    }
    #[doc = "debug connection disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Swdsel::Dis
    }
}
#[doc = "2:2\\]
This bit field specifies the status of JTAG MODE for TEST TAP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jtagsel {
    #[doc = "1: TEST TAP enabled"]
    En = 1,
    #[doc = "0: TEST TAP disabled"]
    Dis = 0,
}
impl From<Jtagsel> for bool {
    #[inline(always)]
    fn from(variant: Jtagsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAGSEL` reader - 2:2\\]
This bit field specifies the status of JTAG MODE for TEST TAP."]
pub type JtagselR = crate::BitReader<Jtagsel>;
impl JtagselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jtagsel {
        match self.bits {
            true => Jtagsel::En,
            false => Jtagsel::Dis,
        }
    }
    #[doc = "TEST TAP enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Jtagsel::En
    }
    #[doc = "TEST TAP disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Jtagsel::Dis
    }
}
#[doc = "3:3\\]
This bit field specify the status of syspwrupack from pmctl.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspwrupack {
    #[doc = "1: syspwrupreq is acknowledged"]
    En = 1,
    #[doc = "0: syspwrupreq is not acknowledged"]
    Dis = 0,
}
impl From<Syspwrupack> for bool {
    #[inline(always)]
    fn from(variant: Syspwrupack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPWRUPACK` reader - 3:3\\]
This bit field specify the status of syspwrupack from pmctl."]
pub type SyspwrupackR = crate::BitReader<Syspwrupack>;
impl SyspwrupackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspwrupack {
        match self.bits {
            true => Syspwrupack::En,
            false => Syspwrupack::Dis,
        }
    }
    #[doc = "syspwrupreq is acknowledged"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Syspwrupack::En
    }
    #[doc = "syspwrupreq is not acknowledged"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Syspwrupack::Dis
    }
}
#[doc = "4:4\\]
This bit field specifies the status of dbgpwrupack from pmctl.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgpwrupack {
    #[doc = "1: dbgpwrupreq is acknowledged."]
    En = 1,
    #[doc = "0: dbgpwrupreq is not acknowledged"]
    Dis = 0,
}
impl From<Dbgpwrupack> for bool {
    #[inline(always)]
    fn from(variant: Dbgpwrupack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGPWRUPACK` reader - 4:4\\]
This bit field specifies the status of dbgpwrupack from pmctl."]
pub type DbgpwrupackR = crate::BitReader<Dbgpwrupack>;
impl DbgpwrupackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgpwrupack {
        match self.bits {
            true => Dbgpwrupack::En,
            false => Dbgpwrupack::Dis,
        }
    }
    #[doc = "dbgpwrupreq is acknowledged."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgpwrupack::En
    }
    #[doc = "dbgpwrupreq is not acknowledged"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgpwrupack::Dis
    }
}
#[doc = "5:5\\]
This bit is used to enable connection between SWD pads and IceMelter\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swdcen {
    #[doc = "1: Connection enabled"]
    En = 1,
    #[doc = "0: Connection disabled"]
    Dis = 0,
}
impl From<Swdcen> for bool {
    #[inline(always)]
    fn from(variant: Swdcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDCEN` reader - 5:5\\]
This bit is used to enable connection between SWD pads and IceMelter"]
pub type SwdcenR = crate::BitReader<Swdcen>;
impl SwdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swdcen {
        match self.bits {
            true => Swdcen::En,
            false => Swdcen::Dis,
        }
    }
    #[doc = "Connection enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Swdcen::En
    }
    #[doc = "Connection disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Swdcen::Dis
    }
}
#[doc = "Field `SWDCEN` writer - 5:5\\]
This bit is used to enable connection between SWD pads and IceMelter"]
pub type SwdcenW<'a, REG> = crate::BitWriter<'a, REG, Swdcen>;
impl<'a, REG> SwdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connection enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Swdcen::En)
    }
    #[doc = "Connection disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Swdcen::Dis)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is used for connecting to IO pads to SWCLK/IO on SW-DP through a software request and establish SWD connection without icemelter trigger for debug purpose."]
    #[inline(always)]
    pub fn swdovr(&self) -> SwdovrR {
        SwdovrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit field specifies the status of SWD MODE for connection."]
    #[inline(always)]
    pub fn swdsel(&self) -> SwdselR {
        SwdselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit field specifies the status of JTAG MODE for TEST TAP."]
    #[inline(always)]
    pub fn jtagsel(&self) -> JtagselR {
        JtagselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit field specify the status of syspwrupack from pmctl."]
    #[inline(always)]
    pub fn syspwrupack(&self) -> SyspwrupackR {
        SyspwrupackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit field specifies the status of dbgpwrupack from pmctl."]
    #[inline(always)]
    pub fn dbgpwrupack(&self) -> DbgpwrupackR {
        DbgpwrupackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is used to enable connection between SWD pads and IceMelter"]
    #[inline(always)]
    pub fn swdcen(&self) -> SwdcenR {
        SwdcenR::new(((self.bits >> 5) & 1) != 0)
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
This bit is used for connecting to IO pads to SWCLK/IO on SW-DP through a software request and establish SWD connection without icemelter trigger for debug purpose."]
    #[inline(always)]
    #[must_use]
    pub fn swdovr(&mut self) -> SwdovrW<DbgctlSpec> {
        SwdovrW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is used to enable connection between SWD pads and IceMelter"]
    #[inline(always)]
    #[must_use]
    pub fn swdcen(&mut self) -> SwdcenW<DbgctlSpec> {
        SwdcenW::new(self, 5)
    }
}
#[doc = "Debug control register. This register is used for controlling debug connection and read out debug status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctlSpec;
impl crate::RegisterSpec for DbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgctl::R`](R) reader structure"]
impl crate::Readable for DbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctl::W`](W) writer structure"]
impl crate::Writable for DbgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGCTL to value 0x20"]
impl crate::Resettable for DbgctlSpec {
    const RESET_VALUE: u32 = 0x20;
}
