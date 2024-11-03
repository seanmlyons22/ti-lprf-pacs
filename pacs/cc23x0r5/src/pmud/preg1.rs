#[doc = "Register `PREG1` reader"]
pub type R = crate::R<Preg1Spec>;
#[doc = "Register `PREG1` writer"]
pub type W = crate::W<Preg1Spec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:1\\]
GLDO ATB selection bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GldoAtbsel {
    #[doc = "8: VDDR_OK connected to test bus"]
    Vddrok = 8,
    #[doc = "4: GLDO 1uA bias current connected to test bus"]
    Ib1u = 4,
    #[doc = "2: GLDO pass transistor gate voltage connected to test bus"]
    Passgate = 2,
    #[doc = "1: GLDO error amp output voltage connected to test bus"]
    ErrampOut = 1,
    #[doc = "0: No GLDO signal connected to test bus"]
    Nc = 0,
}
impl From<GldoAtbsel> for u8 {
    #[inline(always)]
    fn from(variant: GldoAtbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GldoAtbsel {
    type Ux = u8;
}
impl crate::IsEnum for GldoAtbsel {}
#[doc = "Field `GLDO_ATBSEL` reader - 4:1\\]
GLDO ATB selection bits."]
pub type GldoAtbselR = crate::FieldReader<GldoAtbsel>;
impl GldoAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GldoAtbsel> {
        match self.bits {
            8 => Some(GldoAtbsel::Vddrok),
            4 => Some(GldoAtbsel::Ib1u),
            2 => Some(GldoAtbsel::Passgate),
            1 => Some(GldoAtbsel::ErrampOut),
            0 => Some(GldoAtbsel::Nc),
            _ => None,
        }
    }
    #[doc = "VDDR_OK connected to test bus"]
    #[inline(always)]
    pub fn is_vddrok(&self) -> bool {
        *self == GldoAtbsel::Vddrok
    }
    #[doc = "GLDO 1uA bias current connected to test bus"]
    #[inline(always)]
    pub fn is_ib1u(&self) -> bool {
        *self == GldoAtbsel::Ib1u
    }
    #[doc = "GLDO pass transistor gate voltage connected to test bus"]
    #[inline(always)]
    pub fn is_passgate(&self) -> bool {
        *self == GldoAtbsel::Passgate
    }
    #[doc = "GLDO error amp output voltage connected to test bus"]
    #[inline(always)]
    pub fn is_erramp_out(&self) -> bool {
        *self == GldoAtbsel::ErrampOut
    }
    #[doc = "No GLDO signal connected to test bus"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == GldoAtbsel::Nc
    }
}
#[doc = "Field `GLDO_ATBSEL` writer - 4:1\\]
GLDO ATB selection bits."]
pub type GldoAtbselW<'a, REG> = crate::FieldWriter<'a, REG, 4, GldoAtbsel>;
impl<'a, REG> GldoAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDDR_OK connected to test bus"]
    #[inline(always)]
    pub fn vddrok(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAtbsel::Vddrok)
    }
    #[doc = "GLDO 1uA bias current connected to test bus"]
    #[inline(always)]
    pub fn ib1u(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAtbsel::Ib1u)
    }
    #[doc = "GLDO pass transistor gate voltage connected to test bus"]
    #[inline(always)]
    pub fn passgate(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAtbsel::Passgate)
    }
    #[doc = "GLDO error amp output voltage connected to test bus"]
    #[inline(always)]
    pub fn erramp_out(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAtbsel::ErrampOut)
    }
    #[doc = "No GLDO signal connected to test bus"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAtbsel::Nc)
    }
}
#[doc = "5:5\\]
Disable GLDO error amplifier bias current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GldoEaBiasDis {
    #[doc = "1: Bias current is turned off when GLDO is disabled"]
    Off = 1,
    #[doc = "0: Bias current remains ON"]
    On = 0,
}
impl From<GldoEaBiasDis> for bool {
    #[inline(always)]
    fn from(variant: GldoEaBiasDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLDO_EA_BIAS_DIS` reader - 5:5\\]
Disable GLDO error amplifier bias current"]
pub type GldoEaBiasDisR = crate::BitReader<GldoEaBiasDis>;
impl GldoEaBiasDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GldoEaBiasDis {
        match self.bits {
            true => GldoEaBiasDis::Off,
            false => GldoEaBiasDis::On,
        }
    }
    #[doc = "Bias current is turned off when GLDO is disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == GldoEaBiasDis::Off
    }
    #[doc = "Bias current remains ON"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == GldoEaBiasDis::On
    }
}
#[doc = "Field `GLDO_EA_BIAS_DIS` writer - 5:5\\]
Disable GLDO error amplifier bias current"]
pub type GldoEaBiasDisW<'a, REG> = crate::BitWriter<'a, REG, GldoEaBiasDis>;
impl<'a, REG> GldoEaBiasDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bias current is turned off when GLDO is disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(GldoEaBiasDis::Off)
    }
    #[doc = "Bias current remains ON"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(GldoEaBiasDis::On)
    }
}
#[doc = "6:6\\]
VDDR ATB selection bit. This is used to connect VDDR to ATEST bus. Set GLDO_ATBSEL = 0x0 first before setting this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VddrAtbsel {
    #[doc = "1: Connected"]
    En = 1,
    #[doc = "0: Not connected"]
    Dis = 0,
}
impl From<VddrAtbsel> for bool {
    #[inline(always)]
    fn from(variant: VddrAtbsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDR_ATBSEL` reader - 6:6\\]
VDDR ATB selection bit. This is used to connect VDDR to ATEST bus. Set GLDO_ATBSEL = 0x0 first before setting this bit."]
pub type VddrAtbselR = crate::BitReader<VddrAtbsel>;
impl VddrAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VddrAtbsel {
        match self.bits {
            true => VddrAtbsel::En,
            false => VddrAtbsel::Dis,
        }
    }
    #[doc = "Connected"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == VddrAtbsel::En
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VddrAtbsel::Dis
    }
}
#[doc = "Field `VDDR_ATBSEL` writer - 6:6\\]
VDDR ATB selection bit. This is used to connect VDDR to ATEST bus. Set GLDO_ATBSEL = 0x0 first before setting this bit."]
pub type VddrAtbselW<'a, REG> = crate::BitWriter<'a, REG, VddrAtbsel>;
impl<'a, REG> VddrAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connected"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(VddrAtbsel::En)
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(VddrAtbsel::Dis)
    }
}
#[doc = "Field `SPARE` reader - 7:7\\]
Spare bit."]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - 7:7\\]
Spare bit."]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `RESERVED8` writer - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "12:12\\]
Enable/Disable the 32 kHz clock to the recharge comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RchgCompClkDis {
    #[doc = "1: Disable the clock"]
    Dis = 1,
    #[doc = "0: Enable the clock"]
    En = 0,
}
impl From<RchgCompClkDis> for bool {
    #[inline(always)]
    fn from(variant: RchgCompClkDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCHG_COMP_CLK_DIS` reader - 12:12\\]
Enable/Disable the 32 kHz clock to the recharge comparator."]
pub type RchgCompClkDisR = crate::BitReader<RchgCompClkDis>;
impl RchgCompClkDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RchgCompClkDis {
        match self.bits {
            true => RchgCompClkDis::Dis,
            false => RchgCompClkDis::En,
        }
    }
    #[doc = "Disable the clock"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RchgCompClkDis::Dis
    }
    #[doc = "Enable the clock"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RchgCompClkDis::En
    }
}
#[doc = "Field `RCHG_COMP_CLK_DIS` writer - 12:12\\]
Enable/Disable the 32 kHz clock to the recharge comparator."]
pub type RchgCompClkDisW<'a, REG> = crate::BitWriter<'a, REG, RchgCompClkDis>;
impl<'a, REG> RchgCompClkDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RchgCompClkDis::Dis)
    }
    #[doc = "Enable the clock"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RchgCompClkDis::En)
    }
}
#[doc = "13:13\\]
Force sample of VREF on sample capacitor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RchgForceSampVref {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<RchgForceSampVref> for bool {
    #[inline(always)]
    fn from(variant: RchgForceSampVref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCHG_FORCE_SAMP_VREF` reader - 13:13\\]
Force sample of VREF on sample capacitor."]
pub type RchgForceSampVrefR = crate::BitReader<RchgForceSampVref>;
impl RchgForceSampVrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RchgForceSampVref {
        match self.bits {
            true => RchgForceSampVref::En,
            false => RchgForceSampVref::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RchgForceSampVref::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RchgForceSampVref::Dis
    }
}
#[doc = "Field `RCHG_FORCE_SAMP_VREF` writer - 13:13\\]
Force sample of VREF on sample capacitor."]
pub type RchgForceSampVrefW<'a, REG> = crate::BitWriter<'a, REG, RchgForceSampVref>;
impl<'a, REG> RchgForceSampVrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RchgForceSampVref::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RchgForceSampVref::Dis)
    }
}
#[doc = "14:14\\]
Enable/Disable test outputs to recharge comparator block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RchgBlkAtestEn {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<RchgBlkAtestEn> for bool {
    #[inline(always)]
    fn from(variant: RchgBlkAtestEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCHG_BLK_ATEST_EN` reader - 14:14\\]
Enable/Disable test outputs to recharge comparator block."]
pub type RchgBlkAtestEnR = crate::BitReader<RchgBlkAtestEn>;
impl RchgBlkAtestEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RchgBlkAtestEn {
        match self.bits {
            true => RchgBlkAtestEn::En,
            false => RchgBlkAtestEn::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RchgBlkAtestEn::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RchgBlkAtestEn::Dis
    }
}
#[doc = "Field `RCHG_BLK_ATEST_EN` writer - 14:14\\]
Enable/Disable test outputs to recharge comparator block."]
pub type RchgBlkAtestEnW<'a, REG> = crate::BitWriter<'a, REG, RchgBlkAtestEn>;
impl<'a, REG> RchgBlkAtestEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RchgBlkAtestEn::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RchgBlkAtestEn::Dis)
    }
}
#[doc = "15:15\\]
Enable/Disable ATEST input to VDDR input of recharge comparator. Used for trimming the recharge voltage reference level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RchgBlkVtrigEn {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<RchgBlkVtrigEn> for bool {
    #[inline(always)]
    fn from(variant: RchgBlkVtrigEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCHG_BLK_VTRIG_EN` reader - 15:15\\]
Enable/Disable ATEST input to VDDR input of recharge comparator. Used for trimming the recharge voltage reference level."]
pub type RchgBlkVtrigEnR = crate::BitReader<RchgBlkVtrigEn>;
impl RchgBlkVtrigEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RchgBlkVtrigEn {
        match self.bits {
            true => RchgBlkVtrigEn::En,
            false => RchgBlkVtrigEn::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RchgBlkVtrigEn::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RchgBlkVtrigEn::Dis
    }
}
#[doc = "Field `RCHG_BLK_VTRIG_EN` writer - 15:15\\]
Enable/Disable ATEST input to VDDR input of recharge comparator. Used for trimming the recharge voltage reference level."]
pub type RchgBlkVtrigEnW<'a, REG> = crate::BitWriter<'a, REG, RchgBlkVtrigEn>;
impl<'a, REG> RchgBlkVtrigEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RchgBlkVtrigEn::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RchgBlkVtrigEn::Dis)
    }
}
#[doc = "16:16\\]
Keep GLDO always on when enabled. PMUDLC cannot disable GLDO pass gate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GldoAon {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<GldoAon> for bool {
    #[inline(always)]
    fn from(variant: GldoAon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLDO_AON` reader - 16:16\\]
Keep GLDO always on when enabled. PMUDLC cannot disable GLDO pass gate."]
pub type GldoAonR = crate::BitReader<GldoAon>;
impl GldoAonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GldoAon {
        match self.bits {
            true => GldoAon::En,
            false => GldoAon::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GldoAon::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GldoAon::Dis
    }
}
#[doc = "Field `GLDO_AON` writer - 16:16\\]
Keep GLDO always on when enabled. PMUDLC cannot disable GLDO pass gate."]
pub type GldoAonW<'a, REG> = crate::BitWriter<'a, REG, GldoAon>;
impl<'a, REG> GldoAonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAon::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GldoAon::Dis)
    }
}
#[doc = "17:17\\]
Enable switching frequency randomizer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DitherEn {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<DitherEn> for bool {
    #[inline(always)]
    fn from(variant: DitherEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DITHER_EN` reader - 17:17\\]
Enable switching frequency randomizer."]
pub type DitherEnR = crate::BitReader<DitherEn>;
impl DitherEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DitherEn {
        match self.bits {
            true => DitherEn::En,
            false => DitherEn::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DitherEn::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DitherEn::Dis
    }
}
#[doc = "Field `DITHER_EN` writer - 17:17\\]
Enable switching frequency randomizer."]
pub type DitherEnW<'a, REG> = crate::BitWriter<'a, REG, DitherEn>;
impl<'a, REG> DitherEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DitherEn::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DitherEn::Dis)
    }
}
#[doc = "18:18\\]
This bit is used to turn on DCDC PMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestDcdcPmos {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<TestDcdcPmos> for bool {
    #[inline(always)]
    fn from(variant: TestDcdcPmos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST_DCDC_PMOS` reader - 18:18\\]
This bit is used to turn on DCDC PMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
pub type TestDcdcPmosR = crate::BitReader<TestDcdcPmos>;
impl TestDcdcPmosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestDcdcPmos {
        match self.bits {
            true => TestDcdcPmos::En,
            false => TestDcdcPmos::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TestDcdcPmos::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TestDcdcPmos::Dis
    }
}
#[doc = "Field `TEST_DCDC_PMOS` writer - 18:18\\]
This bit is used to turn on DCDC PMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
pub type TestDcdcPmosW<'a, REG> = crate::BitWriter<'a, REG, TestDcdcPmos>;
impl<'a, REG> TestDcdcPmosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TestDcdcPmos::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TestDcdcPmos::Dis)
    }
}
#[doc = "19:19\\]
This bit is used to turn on DCDC NMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestDcdcNmos {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<TestDcdcNmos> for bool {
    #[inline(always)]
    fn from(variant: TestDcdcNmos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST_DCDC_NMOS` reader - 19:19\\]
This bit is used to turn on DCDC NMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
pub type TestDcdcNmosR = crate::BitReader<TestDcdcNmos>;
impl TestDcdcNmosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestDcdcNmos {
        match self.bits {
            true => TestDcdcNmos::En,
            false => TestDcdcNmos::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TestDcdcNmos::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TestDcdcNmos::Dis
    }
}
#[doc = "Field `TEST_DCDC_NMOS` writer - 19:19\\]
This bit is used to turn on DCDC NMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
pub type TestDcdcNmosW<'a, REG> = crate::BitWriter<'a, REG, TestDcdcNmos>;
impl<'a, REG> TestDcdcNmosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TestDcdcNmos::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TestDcdcNmos::Dis)
    }
}
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
GLDO ATB selection bits."]
    #[inline(always)]
    pub fn gldo_atbsel(&self) -> GldoAtbselR {
        GldoAtbselR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Disable GLDO error amplifier bias current"]
    #[inline(always)]
    pub fn gldo_ea_bias_dis(&self) -> GldoEaBiasDisR {
        GldoEaBiasDisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
VDDR ATB selection bit. This is used to connect VDDR to ATEST bus. Set GLDO_ATBSEL = 0x0 first before setting this bit."]
    #[inline(always)]
    pub fn vddr_atbsel(&self) -> VddrAtbselR {
        VddrAtbselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Spare bit."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable/Disable the 32 kHz clock to the recharge comparator."]
    #[inline(always)]
    pub fn rchg_comp_clk_dis(&self) -> RchgCompClkDisR {
        RchgCompClkDisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Force sample of VREF on sample capacitor."]
    #[inline(always)]
    pub fn rchg_force_samp_vref(&self) -> RchgForceSampVrefR {
        RchgForceSampVrefR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable/Disable test outputs to recharge comparator block."]
    #[inline(always)]
    pub fn rchg_blk_atest_en(&self) -> RchgBlkAtestEnR {
        RchgBlkAtestEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable/Disable ATEST input to VDDR input of recharge comparator. Used for trimming the recharge voltage reference level."]
    #[inline(always)]
    pub fn rchg_blk_vtrig_en(&self) -> RchgBlkVtrigEnR {
        RchgBlkVtrigEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Keep GLDO always on when enabled. PMUDLC cannot disable GLDO pass gate."]
    #[inline(always)]
    pub fn gldo_aon(&self) -> GldoAonR {
        GldoAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable switching frequency randomizer."]
    #[inline(always)]
    pub fn dither_en(&self) -> DitherEnR {
        DitherEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
This bit is used to turn on DCDC PMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
    #[inline(always)]
    pub fn test_dcdc_pmos(&self) -> TestDcdcPmosR {
        TestDcdcPmosR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
This bit is used to turn on DCDC NMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
    #[inline(always)]
    pub fn test_dcdc_nmos(&self) -> TestDcdcNmosR {
        TestDcdcNmosR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Preg1Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
GLDO ATB selection bits."]
    #[inline(always)]
    #[must_use]
    pub fn gldo_atbsel(&mut self) -> GldoAtbselW<Preg1Spec> {
        GldoAtbselW::new(self, 1)
    }
    #[doc = "Bit 5 - 5:5\\]
Disable GLDO error amplifier bias current"]
    #[inline(always)]
    #[must_use]
    pub fn gldo_ea_bias_dis(&mut self) -> GldoEaBiasDisW<Preg1Spec> {
        GldoEaBiasDisW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
VDDR ATB selection bit. This is used to connect VDDR to ATEST bus. Set GLDO_ATBSEL = 0x0 first before setting this bit."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_atbsel(&mut self) -> VddrAtbselW<Preg1Spec> {
        VddrAtbselW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Spare bit."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<Preg1Spec> {
        SpareW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Preg1Spec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable/Disable the 32 kHz clock to the recharge comparator."]
    #[inline(always)]
    #[must_use]
    pub fn rchg_comp_clk_dis(&mut self) -> RchgCompClkDisW<Preg1Spec> {
        RchgCompClkDisW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Force sample of VREF on sample capacitor."]
    #[inline(always)]
    #[must_use]
    pub fn rchg_force_samp_vref(&mut self) -> RchgForceSampVrefW<Preg1Spec> {
        RchgForceSampVrefW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Enable/Disable test outputs to recharge comparator block."]
    #[inline(always)]
    #[must_use]
    pub fn rchg_blk_atest_en(&mut self) -> RchgBlkAtestEnW<Preg1Spec> {
        RchgBlkAtestEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable/Disable ATEST input to VDDR input of recharge comparator. Used for trimming the recharge voltage reference level."]
    #[inline(always)]
    #[must_use]
    pub fn rchg_blk_vtrig_en(&mut self) -> RchgBlkVtrigEnW<Preg1Spec> {
        RchgBlkVtrigEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Keep GLDO always on when enabled. PMUDLC cannot disable GLDO pass gate."]
    #[inline(always)]
    #[must_use]
    pub fn gldo_aon(&mut self) -> GldoAonW<Preg1Spec> {
        GldoAonW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable switching frequency randomizer."]
    #[inline(always)]
    #[must_use]
    pub fn dither_en(&mut self) -> DitherEnW<Preg1Spec> {
        DitherEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
This bit is used to turn on DCDC PMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
    #[inline(always)]
    #[must_use]
    pub fn test_dcdc_pmos(&mut self) -> TestDcdcPmosW<Preg1Spec> {
        TestDcdcPmosW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
This bit is used to turn on DCDC NMOS switch. TEST_DCDC_PMOS and TEST_DCDC_NMOS bits should not be set together."]
    #[inline(always)]
    #[must_use]
    pub fn test_dcdc_nmos(&mut self) -> TestDcdcNmosW<Preg1Spec> {
        TestDcdcNmosW::new(self, 19)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<Preg1Spec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "PMU REG 1 register. Note: All bits in this register except DITHER_EN are write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preg1Spec;
impl crate::RegisterSpec for Preg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preg1::R`](R) reader structure"]
impl crate::Readable for Preg1Spec {}
#[doc = "`write(|w| ..)` method takes [`preg1::W`](W) writer structure"]
impl crate::Writable for Preg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREG1 to value 0"]
impl crate::Resettable for Preg1Spec {
    const RESET_VALUE: u32 = 0;
}
