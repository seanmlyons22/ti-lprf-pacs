#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "0:0\\]
Clock enable to AHB bridge. The bridge will request it's own clock, this bit it to override that feature to have a free running clock.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bridge {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Bridge> for bool {
    #[inline(always)]
    fn from(variant: Bridge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRIDGE` reader - 0:0\\]
Clock enable to AHB bridge. The bridge will request it's own clock, this bit it to override that feature to have a free running clock."]
pub type BridgeR = crate::BitReader<Bridge>;
impl BridgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bridge {
        match self.bits {
            true => Bridge::En,
            false => Bridge::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bridge::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bridge::Dis
    }
}
#[doc = "Field `BRIDGE` writer - 0:0\\]
Clock enable to AHB bridge. The bridge will request it's own clock, this bit it to override that feature to have a free running clock."]
pub type BridgeW<'a, REG> = crate::BitWriter<'a, REG, Bridge>;
impl<'a, REG> BridgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bridge::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bridge::Dis)
    }
}
#[doc = "1:1\\]
Enable the clock to the PBE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbe {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Pbe> for bool {
    #[inline(always)]
    fn from(variant: Pbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBE` reader - 1:1\\]
Enable the clock to the PBE"]
pub type PbeR = crate::BitReader<Pbe>;
impl PbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbe {
        match self.bits {
            true => Pbe::En,
            false => Pbe::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pbe::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pbe::Dis
    }
}
#[doc = "Field `PBE` writer - 1:1\\]
Enable the clock to the PBE"]
pub type PbeW<'a, REG> = crate::BitWriter<'a, REG, Pbe>;
impl<'a, REG> PbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pbe::Dis)
    }
}
#[doc = "2:2\\]
Enable the clock to the Modem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdm {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Mdm> for bool {
    #[inline(always)]
    fn from(variant: Mdm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDM` reader - 2:2\\]
Enable the clock to the Modem"]
pub type MdmR = crate::BitReader<Mdm>;
impl MdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdm {
        match self.bits {
            true => Mdm::En,
            false => Mdm::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mdm::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mdm::Dis
    }
}
#[doc = "Field `MDM` writer - 2:2\\]
Enable the clock to the Modem"]
pub type MdmW<'a, REG> = crate::BitWriter<'a, REG, Mdm>;
impl<'a, REG> MdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mdm::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mdm::Dis)
    }
}
#[doc = "3:3\\]
Enable the clock to the RFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - 3:3\\]
Enable the clock to the RFE"]
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            true => Rfe::En,
            false => Rfe::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rfe::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rfe::Dis
    }
}
#[doc = "Field `RFE` writer - 3:3\\]
Enable the clock to the RFE"]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG, Rfe>;
impl<'a, REG> RfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rfe::Dis)
    }
}
#[doc = "4:4\\]
Enable the clock to Samples2RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2r {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<S2r> for bool {
    #[inline(always)]
    fn from(variant: S2r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2R` reader - 4:4\\]
Enable the clock to Samples2RAM"]
pub type S2rR = crate::BitReader<S2r>;
impl S2rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2r {
        match self.bits {
            true => S2r::En,
            false => S2r::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == S2r::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == S2r::Dis
    }
}
#[doc = "Field `S2R` writer - 4:4\\]
Enable the clock to Samples2RAM"]
pub type S2rW<'a, REG> = crate::BitWriter<'a, REG, S2r>;
impl<'a, REG> S2rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(S2r::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(S2r::Dis)
    }
}
#[doc = "5:5\\]
Enable the clock to the Tracer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trc {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Trc> for bool {
    #[inline(always)]
    fn from(variant: Trc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRC` reader - 5:5\\]
Enable the clock to the Tracer"]
pub type TrcR = crate::BitReader<Trc>;
impl TrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trc {
        match self.bits {
            true => Trc::En,
            false => Trc::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Trc::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Trc::Dis
    }
}
#[doc = "Field `TRC` writer - 5:5\\]
Enable the clock to the Tracer"]
pub type TrcW<'a, REG> = crate::BitWriter<'a, REG, Trc>;
impl<'a, REG> TrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Trc::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Trc::Dis)
    }
}
#[doc = "6:6\\]
Enable the clock to the PBE RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pberam {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Pberam> for bool {
    #[inline(always)]
    fn from(variant: Pberam) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBERAM` reader - 6:6\\]
Enable the clock to the PBE RAM"]
pub type PberamR = crate::BitReader<Pberam>;
impl PberamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pberam {
        match self.bits {
            true => Pberam::En,
            false => Pberam::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pberam::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pberam::Dis
    }
}
#[doc = "Field `PBERAM` writer - 6:6\\]
Enable the clock to the PBE RAM"]
pub type PberamW<'a, REG> = crate::BitWriter<'a, REG, Pberam>;
impl<'a, REG> PberamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pberam::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pberam::Dis)
    }
}
#[doc = "7:7\\]
Enable the clock to the MCE RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mceram {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Mceram> for bool {
    #[inline(always)]
    fn from(variant: Mceram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCERAM` reader - 7:7\\]
Enable the clock to the MCE RAM"]
pub type MceramR = crate::BitReader<Mceram>;
impl MceramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mceram {
        match self.bits {
            true => Mceram::En,
            false => Mceram::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mceram::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mceram::Dis
    }
}
#[doc = "Field `MCERAM` writer - 7:7\\]
Enable the clock to the MCE RAM"]
pub type MceramW<'a, REG> = crate::BitWriter<'a, REG, Mceram>;
impl<'a, REG> MceramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mceram::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mceram::Dis)
    }
}
#[doc = "8:8\\]
Enable the clock to the RFE RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rferam {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Rferam> for bool {
    #[inline(always)]
    fn from(variant: Rferam) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFERAM` reader - 8:8\\]
Enable the clock to the RFE RAM"]
pub type RferamR = crate::BitReader<Rferam>;
impl RferamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rferam {
        match self.bits {
            true => Rferam::En,
            false => Rferam::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rferam::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rferam::Dis
    }
}
#[doc = "Field `RFERAM` writer - 8:8\\]
Enable the clock to the RFE RAM"]
pub type RferamW<'a, REG> = crate::BitWriter<'a, REG, Rferam>;
impl<'a, REG> RferamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rferam::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rferam::Dis)
    }
}
#[doc = "9:9\\]
Enable the clock to the DSB RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsbram {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Dsbram> for bool {
    #[inline(always)]
    fn from(variant: Dsbram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSBRAM` reader - 9:9\\]
Enable the clock to the DSB RAM"]
pub type DsbramR = crate::BitReader<Dsbram>;
impl DsbramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsbram {
        match self.bits {
            true => Dsbram::En,
            false => Dsbram::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dsbram::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dsbram::Dis
    }
}
#[doc = "Field `DSBRAM` writer - 9:9\\]
Enable the clock to the DSB RAM"]
pub type DsbramW<'a, REG> = crate::BitWriter<'a, REG, Dsbram>;
impl<'a, REG> DsbramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dsbram::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dsbram::Dis)
    }
}
#[doc = "10:10\\]
Enable the clock to the BUFRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufram {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Bufram> for bool {
    #[inline(always)]
    fn from(variant: Bufram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFRAM` reader - 10:10\\]
Enable the clock to the BUFRAM"]
pub type BuframR = crate::BitReader<Bufram>;
impl BuframR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufram {
        match self.bits {
            true => Bufram::En,
            false => Bufram::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bufram::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bufram::Dis
    }
}
#[doc = "Field `BUFRAM` writer - 10:10\\]
Enable the clock to the BUFRAM"]
pub type BuframW<'a, REG> = crate::BitWriter<'a, REG, Bufram>;
impl<'a, REG> BuframW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bufram::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bufram::Dis)
    }
}
#[doc = "11:11\\]
Enable the clock to the S2R RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2rram {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<S2rram> for bool {
    #[inline(always)]
    fn from(variant: S2rram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RRAM` reader - 11:11\\]
Enable the clock to the S2R RAM"]
pub type S2rramR = crate::BitReader<S2rram>;
impl S2rramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2rram {
        match self.bits {
            true => S2rram::En,
            false => S2rram::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == S2rram::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == S2rram::Dis
    }
}
#[doc = "Field `S2RRAM` writer - 11:11\\]
Enable the clock to the S2R RAM"]
pub type S2rramW<'a, REG> = crate::BitWriter<'a, REG, S2rram>;
impl<'a, REG> S2rramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(S2rram::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(S2rram::Dis)
    }
}
#[doc = "12:12\\]
Enable the clock to the modulator. Modem will request this clock automatically, this bit is to force the modulator clock to be free running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mod {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Mod> for bool {
    #[inline(always)]
    fn from(variant: Mod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOD` reader - 12:12\\]
Enable the clock to the modulator. Modem will request this clock automatically, this bit is to force the modulator clock to be free running."]
pub type ModR = crate::BitReader<Mod>;
impl ModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mod {
        match self.bits {
            true => Mod::En,
            false => Mod::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Mod::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mod::Dis
    }
}
#[doc = "Field `MOD` writer - 12:12\\]
Enable the clock to the modulator. Modem will request this clock automatically, this bit is to force the modulator clock to be free running."]
pub type ModW<'a, REG> = crate::BitWriter<'a, REG, Mod>;
impl<'a, REG> ModW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::Dis)
    }
}
#[doc = "13:13\\]
Enable the clock to the demodulator. The modem will request this clock automatically. This bit is to force the clock to be free running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dem {
    #[doc = "1: Clock is requested"]
    En = 1,
    #[doc = "0: Clock not requested"]
    Dis = 0,
}
impl From<Dem> for bool {
    #[inline(always)]
    fn from(variant: Dem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - 13:13\\]
Enable the clock to the demodulator. The modem will request this clock automatically. This bit is to force the clock to be free running"]
pub type DemR = crate::BitReader<Dem>;
impl DemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dem {
        match self.bits {
            true => Dem::En,
            false => Dem::Dis,
        }
    }
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dem::En
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dem::Dis
    }
}
#[doc = "Field `DEM` writer - 13:13\\]
Enable the clock to the demodulator. The modem will request this clock automatically. This bit is to force the clock to be free running"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG, Dem>;
impl<'a, REG> DemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is requested"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dem::En)
    }
    #[doc = "Clock not requested"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dem::Dis)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock enable to AHB bridge. The bridge will request it's own clock, this bit it to override that feature to have a free running clock."]
    #[inline(always)]
    pub fn bridge(&self) -> BridgeR {
        BridgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable the clock to the PBE"]
    #[inline(always)]
    pub fn pbe(&self) -> PbeR {
        PbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable the clock to the Modem"]
    #[inline(always)]
    pub fn mdm(&self) -> MdmR {
        MdmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable the clock to the RFE"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable the clock to Samples2RAM"]
    #[inline(always)]
    pub fn s2r(&self) -> S2rR {
        S2rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable the clock to the Tracer"]
    #[inline(always)]
    pub fn trc(&self) -> TrcR {
        TrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable the clock to the PBE RAM"]
    #[inline(always)]
    pub fn pberam(&self) -> PberamR {
        PberamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable the clock to the MCE RAM"]
    #[inline(always)]
    pub fn mceram(&self) -> MceramR {
        MceramR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable the clock to the RFE RAM"]
    #[inline(always)]
    pub fn rferam(&self) -> RferamR {
        RferamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable the clock to the DSB RAM"]
    #[inline(always)]
    pub fn dsbram(&self) -> DsbramR {
        DsbramR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable the clock to the BUFRAM"]
    #[inline(always)]
    pub fn bufram(&self) -> BuframR {
        BuframR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable the clock to the S2R RAM"]
    #[inline(always)]
    pub fn s2rram(&self) -> S2rramR {
        S2rramR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable the clock to the modulator. Modem will request this clock automatically, this bit is to force the modulator clock to be free running."]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable the clock to the demodulator. The modem will request this clock automatically. This bit is to force the clock to be free running"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clock enable to AHB bridge. The bridge will request it's own clock, this bit it to override that feature to have a free running clock."]
    #[inline(always)]
    #[must_use]
    pub fn bridge(&mut self) -> BridgeW<ClkctlSpec> {
        BridgeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable the clock to the PBE"]
    #[inline(always)]
    #[must_use]
    pub fn pbe(&mut self) -> PbeW<ClkctlSpec> {
        PbeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable the clock to the Modem"]
    #[inline(always)]
    #[must_use]
    pub fn mdm(&mut self) -> MdmW<ClkctlSpec> {
        MdmW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable the clock to the RFE"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<ClkctlSpec> {
        RfeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable the clock to Samples2RAM"]
    #[inline(always)]
    #[must_use]
    pub fn s2r(&mut self) -> S2rW<ClkctlSpec> {
        S2rW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable the clock to the Tracer"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TrcW<ClkctlSpec> {
        TrcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable the clock to the PBE RAM"]
    #[inline(always)]
    #[must_use]
    pub fn pberam(&mut self) -> PberamW<ClkctlSpec> {
        PberamW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable the clock to the MCE RAM"]
    #[inline(always)]
    #[must_use]
    pub fn mceram(&mut self) -> MceramW<ClkctlSpec> {
        MceramW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable the clock to the RFE RAM"]
    #[inline(always)]
    #[must_use]
    pub fn rferam(&mut self) -> RferamW<ClkctlSpec> {
        RferamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable the clock to the DSB RAM"]
    #[inline(always)]
    #[must_use]
    pub fn dsbram(&mut self) -> DsbramW<ClkctlSpec> {
        DsbramW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable the clock to the BUFRAM"]
    #[inline(always)]
    #[must_use]
    pub fn bufram(&mut self) -> BuframW<ClkctlSpec> {
        BuframW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable the clock to the S2R RAM"]
    #[inline(always)]
    #[must_use]
    pub fn s2rram(&mut self) -> S2rramW<ClkctlSpec> {
        S2rramW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable the clock to the modulator. Modem will request this clock automatically, this bit is to force the modulator clock to be free running."]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> ModW<ClkctlSpec> {
        ModW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable the clock to the demodulator. The modem will request this clock automatically. This bit is to force the clock to be free running"]
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DemW<ClkctlSpec> {
        DemW::new(self, 13)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<ClkctlSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Controls the functional clock gates for the individual sub-modules. Writing a bit to zero does not necessarily switch off the corresponding clock. It can also be requested internally. A clock will only be switched off if internal and external requests are removed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTL to value 0x01"]
impl crate::Resettable for ClkctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
