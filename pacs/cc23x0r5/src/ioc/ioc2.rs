#[doc = "Register `IOC2` reader"]
pub type R = crate::R<Ioc2Spec>;
#[doc = "Register `IOC2` writer"]
pub type W = crate::W<Ioc2Spec>;
#[doc = "2:0\\]
Port configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Portcfg {
    #[doc = "7: Digital Test Bus function"]
    Dtb = 7,
    #[doc = "6: Analog function"]
    Ana = 6,
    #[doc = "5: Digital peripheral function-5"]
    Pfunc5 = 5,
    #[doc = "4: Digital peripheral function-4"]
    Pfunc4 = 4,
    #[doc = "3: Digital peripheral function-3"]
    Pfunc3 = 3,
    #[doc = "2: Digital peripheral function-2"]
    Pfunc2 = 2,
    #[doc = "1: Digital peripheral function-1"]
    Pfunc1 = 1,
    #[doc = "0: Base function"]
    Base = 0,
}
impl From<Portcfg> for u8 {
    #[inline(always)]
    fn from(variant: Portcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Portcfg {
    type Ux = u8;
}
impl crate::IsEnum for Portcfg {}
#[doc = "Field `PORTCFG` reader - 2:0\\]
Port configuration."]
pub type PortcfgR = crate::FieldReader<Portcfg>;
impl PortcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portcfg {
        match self.bits {
            7 => Portcfg::Dtb,
            6 => Portcfg::Ana,
            5 => Portcfg::Pfunc5,
            4 => Portcfg::Pfunc4,
            3 => Portcfg::Pfunc3,
            2 => Portcfg::Pfunc2,
            1 => Portcfg::Pfunc1,
            0 => Portcfg::Base,
            _ => unreachable!(),
        }
    }
    #[doc = "Digital Test Bus function"]
    #[inline(always)]
    pub fn is_dtb(&self) -> bool {
        *self == Portcfg::Dtb
    }
    #[doc = "Analog function"]
    #[inline(always)]
    pub fn is_ana(&self) -> bool {
        *self == Portcfg::Ana
    }
    #[doc = "Digital peripheral function-5"]
    #[inline(always)]
    pub fn is_pfunc5(&self) -> bool {
        *self == Portcfg::Pfunc5
    }
    #[doc = "Digital peripheral function-4"]
    #[inline(always)]
    pub fn is_pfunc4(&self) -> bool {
        *self == Portcfg::Pfunc4
    }
    #[doc = "Digital peripheral function-3"]
    #[inline(always)]
    pub fn is_pfunc3(&self) -> bool {
        *self == Portcfg::Pfunc3
    }
    #[doc = "Digital peripheral function-2"]
    #[inline(always)]
    pub fn is_pfunc2(&self) -> bool {
        *self == Portcfg::Pfunc2
    }
    #[doc = "Digital peripheral function-1"]
    #[inline(always)]
    pub fn is_pfunc1(&self) -> bool {
        *self == Portcfg::Pfunc1
    }
    #[doc = "Base function"]
    #[inline(always)]
    pub fn is_base(&self) -> bool {
        *self == Portcfg::Base
    }
}
#[doc = "Field `PORTCFG` writer - 2:0\\]
Port configuration."]
pub type PortcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Portcfg, crate::Safe>;
impl<'a, REG> PortcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital Test Bus function"]
    #[inline(always)]
    pub fn dtb(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Dtb)
    }
    #[doc = "Analog function"]
    #[inline(always)]
    pub fn ana(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Ana)
    }
    #[doc = "Digital peripheral function-5"]
    #[inline(always)]
    pub fn pfunc5(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Pfunc5)
    }
    #[doc = "Digital peripheral function-4"]
    #[inline(always)]
    pub fn pfunc4(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Pfunc4)
    }
    #[doc = "Digital peripheral function-3"]
    #[inline(always)]
    pub fn pfunc3(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Pfunc3)
    }
    #[doc = "Digital peripheral function-2"]
    #[inline(always)]
    pub fn pfunc2(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Pfunc2)
    }
    #[doc = "Digital peripheral function-1"]
    #[inline(always)]
    pub fn pfunc1(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Pfunc1)
    }
    #[doc = "Base function"]
    #[inline(always)]
    pub fn base(self) -> &'a mut crate::W<REG> {
        self.variant(Portcfg::Base)
    }
}
#[doc = "Field `RESERVED3` reader - 12:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED3` writer - 12:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "14:13\\]
Pull control. Setting this to value 0x3 disables pull.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pullctl {
    #[doc = "2: Pull up enabled"]
    PullUp = 2,
    #[doc = "1: Pull down enabled"]
    PullDown = 1,
    #[doc = "0: No pull"]
    PullDis = 0,
}
impl From<Pullctl> for u8 {
    #[inline(always)]
    fn from(variant: Pullctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pullctl {
    type Ux = u8;
}
impl crate::IsEnum for Pullctl {}
#[doc = "Field `PULLCTL` reader - 14:13\\]
Pull control. Setting this to value 0x3 disables pull."]
pub type PullctlR = crate::FieldReader<Pullctl>;
impl PullctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pullctl> {
        match self.bits {
            2 => Some(Pullctl::PullUp),
            1 => Some(Pullctl::PullDown),
            0 => Some(Pullctl::PullDis),
            _ => None,
        }
    }
    #[doc = "Pull up enabled"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Pullctl::PullUp
    }
    #[doc = "Pull down enabled"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Pullctl::PullDown
    }
    #[doc = "No pull"]
    #[inline(always)]
    pub fn is_pull_dis(&self) -> bool {
        *self == Pullctl::PullDis
    }
}
#[doc = "Field `PULLCTL` writer - 14:13\\]
Pull control. Setting this to value 0x3 disables pull."]
pub type PullctlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pullctl>;
impl<'a, REG> PullctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull up enabled"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Pullctl::PullUp)
    }
    #[doc = "Pull down enabled"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pullctl::PullDown)
    }
    #[doc = "No pull"]
    #[inline(always)]
    pub fn pull_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pullctl::PullDis)
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "17:16\\]
Edge detect configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgedet {
    #[doc = "3: Positive and negative edge detection"]
    EdgeBoth = 3,
    #[doc = "2: Positive edge detection"]
    EdgePos = 2,
    #[doc = "1: Negative edge detection"]
    EdgeNeg = 1,
    #[doc = "0: No edge detection"]
    EdgeDis = 0,
}
impl From<Edgedet> for u8 {
    #[inline(always)]
    fn from(variant: Edgedet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgedet {
    type Ux = u8;
}
impl crate::IsEnum for Edgedet {}
#[doc = "Field `EDGEDET` reader - 17:16\\]
Edge detect configuration"]
pub type EdgedetR = crate::FieldReader<Edgedet>;
impl EdgedetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edgedet {
        match self.bits {
            3 => Edgedet::EdgeBoth,
            2 => Edgedet::EdgePos,
            1 => Edgedet::EdgeNeg,
            0 => Edgedet::EdgeDis,
            _ => unreachable!(),
        }
    }
    #[doc = "Positive and negative edge detection"]
    #[inline(always)]
    pub fn is_edge_both(&self) -> bool {
        *self == Edgedet::EdgeBoth
    }
    #[doc = "Positive edge detection"]
    #[inline(always)]
    pub fn is_edge_pos(&self) -> bool {
        *self == Edgedet::EdgePos
    }
    #[doc = "Negative edge detection"]
    #[inline(always)]
    pub fn is_edge_neg(&self) -> bool {
        *self == Edgedet::EdgeNeg
    }
    #[doc = "No edge detection"]
    #[inline(always)]
    pub fn is_edge_dis(&self) -> bool {
        *self == Edgedet::EdgeDis
    }
}
#[doc = "Field `EDGEDET` writer - 17:16\\]
Edge detect configuration"]
pub type EdgedetW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgedet, crate::Safe>;
impl<'a, REG> EdgedetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive and negative edge detection"]
    #[inline(always)]
    pub fn edge_both(self) -> &'a mut crate::W<REG> {
        self.variant(Edgedet::EdgeBoth)
    }
    #[doc = "Positive edge detection"]
    #[inline(always)]
    pub fn edge_pos(self) -> &'a mut crate::W<REG> {
        self.variant(Edgedet::EdgePos)
    }
    #[doc = "Negative edge detection"]
    #[inline(always)]
    pub fn edge_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Edgedet::EdgeNeg)
    }
    #[doc = "No edge detection"]
    #[inline(always)]
    pub fn edge_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Edgedet::EdgeDis)
    }
}
#[doc = "18:18\\]
Wakeup enable from standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuensb {
    #[doc = "1: wakeup enabled (effective only if EDGEDET is enabled)"]
    En = 1,
    #[doc = "0: Wakeup disabled"]
    Dis = 0,
}
impl From<Wuensb> for bool {
    #[inline(always)]
    fn from(variant: Wuensb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUENSB` reader - 18:18\\]
Wakeup enable from standby"]
pub type WuensbR = crate::BitReader<Wuensb>;
impl WuensbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuensb {
        match self.bits {
            true => Wuensb::En,
            false => Wuensb::Dis,
        }
    }
    #[doc = "wakeup enabled (effective only if EDGEDET is enabled)"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wuensb::En
    }
    #[doc = "Wakeup disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wuensb::Dis
    }
}
#[doc = "Field `WUENSB` writer - 18:18\\]
Wakeup enable from standby"]
pub type WuensbW<'a, REG> = crate::BitWriter<'a, REG, Wuensb>;
impl<'a, REG> WuensbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wakeup enabled (effective only if EDGEDET is enabled)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Wuensb::En)
    }
    #[doc = "Wakeup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Wuensb::Dis)
    }
}
#[doc = "Field `RESERVED19` reader - 19:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::BitReader;
#[doc = "Field `RESERVED19` writer - 19:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "21:20\\]
Wakeup configuration from shutdown\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wucfgsd {
    #[doc = "3: Wakeup triggered when pad level is high"]
    WakeHigh = 3,
    #[doc = "2: Wakeup triggered when pad level is low"]
    WakeLow = 2,
    #[doc = "1: Wakeup disabled"]
    Dis1 = 1,
    #[doc = "0: Wakeup disabled"]
    Dis0 = 0,
}
impl From<Wucfgsd> for u8 {
    #[inline(always)]
    fn from(variant: Wucfgsd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wucfgsd {
    type Ux = u8;
}
impl crate::IsEnum for Wucfgsd {}
#[doc = "Field `WUCFGSD` reader - 21:20\\]
Wakeup configuration from shutdown"]
pub type WucfgsdR = crate::FieldReader<Wucfgsd>;
impl WucfgsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wucfgsd {
        match self.bits {
            3 => Wucfgsd::WakeHigh,
            2 => Wucfgsd::WakeLow,
            1 => Wucfgsd::Dis1,
            0 => Wucfgsd::Dis0,
            _ => unreachable!(),
        }
    }
    #[doc = "Wakeup triggered when pad level is high"]
    #[inline(always)]
    pub fn is_wake_high(&self) -> bool {
        *self == Wucfgsd::WakeHigh
    }
    #[doc = "Wakeup triggered when pad level is low"]
    #[inline(always)]
    pub fn is_wake_low(&self) -> bool {
        *self == Wucfgsd::WakeLow
    }
    #[doc = "Wakeup disabled"]
    #[inline(always)]
    pub fn is_dis_1(&self) -> bool {
        *self == Wucfgsd::Dis1
    }
    #[doc = "Wakeup disabled"]
    #[inline(always)]
    pub fn is_dis_0(&self) -> bool {
        *self == Wucfgsd::Dis0
    }
}
#[doc = "Field `WUCFGSD` writer - 21:20\\]
Wakeup configuration from shutdown"]
pub type WucfgsdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wucfgsd, crate::Safe>;
impl<'a, REG> WucfgsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wakeup triggered when pad level is high"]
    #[inline(always)]
    pub fn wake_high(self) -> &'a mut crate::W<REG> {
        self.variant(Wucfgsd::WakeHigh)
    }
    #[doc = "Wakeup triggered when pad level is low"]
    #[inline(always)]
    pub fn wake_low(self) -> &'a mut crate::W<REG> {
        self.variant(Wucfgsd::WakeLow)
    }
    #[doc = "Wakeup disabled"]
    #[inline(always)]
    pub fn dis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wucfgsd::Dis1)
    }
    #[doc = "Wakeup disabled"]
    #[inline(always)]
    pub fn dis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wucfgsd::Dis0)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "26:24\\]
IO Mode. Setting this to value 0x6 or 0x7 will default to normal IO behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iomode {
    #[doc = "5: Open Source, inverted IO"]
    OpensInv = 5,
    #[doc = "4: Open Source, normal IO"]
    Opens = 4,
    #[doc = "3: Open Drain, inverted IO"]
    OpendInv = 3,
    #[doc = "2: Open Drain, normal IO"]
    Opend = 2,
    #[doc = "1: Inverted IO"]
    Inverted = 1,
    #[doc = "0: Normal IO"]
    Normal = 0,
}
impl From<Iomode> for u8 {
    #[inline(always)]
    fn from(variant: Iomode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iomode {
    type Ux = u8;
}
impl crate::IsEnum for Iomode {}
#[doc = "Field `IOMODE` reader - 26:24\\]
IO Mode. Setting this to value 0x6 or 0x7 will default to normal IO behavior."]
pub type IomodeR = crate::FieldReader<Iomode>;
impl IomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iomode> {
        match self.bits {
            5 => Some(Iomode::OpensInv),
            4 => Some(Iomode::Opens),
            3 => Some(Iomode::OpendInv),
            2 => Some(Iomode::Opend),
            1 => Some(Iomode::Inverted),
            0 => Some(Iomode::Normal),
            _ => None,
        }
    }
    #[doc = "Open Source, inverted IO"]
    #[inline(always)]
    pub fn is_opens_inv(&self) -> bool {
        *self == Iomode::OpensInv
    }
    #[doc = "Open Source, normal IO"]
    #[inline(always)]
    pub fn is_opens(&self) -> bool {
        *self == Iomode::Opens
    }
    #[doc = "Open Drain, inverted IO"]
    #[inline(always)]
    pub fn is_opend_inv(&self) -> bool {
        *self == Iomode::OpendInv
    }
    #[doc = "Open Drain, normal IO"]
    #[inline(always)]
    pub fn is_opend(&self) -> bool {
        *self == Iomode::Opend
    }
    #[doc = "Inverted IO"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Iomode::Inverted
    }
    #[doc = "Normal IO"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Iomode::Normal
    }
}
#[doc = "Field `IOMODE` writer - 26:24\\]
IO Mode. Setting this to value 0x6 or 0x7 will default to normal IO behavior."]
pub type IomodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Iomode>;
impl<'a, REG> IomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Open Source, inverted IO"]
    #[inline(always)]
    pub fn opens_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::OpensInv)
    }
    #[doc = "Open Source, normal IO"]
    #[inline(always)]
    pub fn opens(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Opens)
    }
    #[doc = "Open Drain, inverted IO"]
    #[inline(always)]
    pub fn opend_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::OpendInv)
    }
    #[doc = "Open Drain, normal IO"]
    #[inline(always)]
    pub fn opend(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Opend)
    }
    #[doc = "Inverted IO"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Inverted)
    }
    #[doc = "Normal IO"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::Normal)
    }
}
#[doc = "Field `RESERVED27` reader - 28:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27R = crate::FieldReader;
#[doc = "Field `RESERVED27` writer - 28:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "29:29\\]
Input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inpen {
    #[doc = "1: Input enabled"]
    En = 1,
    #[doc = "0: Input disabled"]
    Dis = 0,
}
impl From<Inpen> for bool {
    #[inline(always)]
    fn from(variant: Inpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPEN` reader - 29:29\\]
Input enable"]
pub type InpenR = crate::BitReader<Inpen>;
impl InpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inpen {
        match self.bits {
            true => Inpen::En,
            false => Inpen::Dis,
        }
    }
    #[doc = "Input enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Inpen::En
    }
    #[doc = "Input disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Inpen::Dis
    }
}
#[doc = "Field `INPEN` writer - 29:29\\]
Input enable"]
pub type InpenW<'a, REG> = crate::BitWriter<'a, REG, Inpen>;
impl<'a, REG> InpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Inpen::En)
    }
    #[doc = "Input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Inpen::Dis)
    }
}
#[doc = "30:30\\]
This field controls input hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hysten {
    #[doc = "1: Input hysteresis enabled"]
    En = 1,
    #[doc = "0: Input hysteresis disabled"]
    Dis = 0,
}
impl From<Hysten> for bool {
    #[inline(always)]
    fn from(variant: Hysten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYSTEN` reader - 30:30\\]
This field controls input hysteresis"]
pub type HystenR = crate::BitReader<Hysten>;
impl HystenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hysten {
        match self.bits {
            true => Hysten::En,
            false => Hysten::Dis,
        }
    }
    #[doc = "Input hysteresis enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hysten::En
    }
    #[doc = "Input hysteresis disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hysten::Dis
    }
}
#[doc = "Field `HYSTEN` writer - 30:30\\]
This field controls input hysteresis"]
pub type HystenW<'a, REG> = crate::BitWriter<'a, REG, Hysten>;
impl<'a, REG> HystenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input hysteresis enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hysten::En)
    }
    #[doc = "Input hysteresis disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hysten::Dis)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Port configuration."]
    #[inline(always)]
    pub fn portcfg(&self) -> PortcfgR {
        PortcfgR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:12 - 12:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Pull control. Setting this to value 0x3 disables pull."]
    #[inline(always)]
    pub fn pullctl(&self) -> PullctlR {
        PullctlR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Edge detect configuration"]
    #[inline(always)]
    pub fn edgedet(&self) -> EdgedetR {
        EdgedetR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Wakeup enable from standby"]
    #[inline(always)]
    pub fn wuensb(&self) -> WuensbR {
        WuensbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Wakeup configuration from shutdown"]
    #[inline(always)]
    pub fn wucfgsd(&self) -> WucfgsdR {
        WucfgsdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode. Setting this to value 0x6 or 0x7 will default to normal IO behavior."]
    #[inline(always)]
    pub fn iomode(&self) -> IomodeR {
        IomodeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Input enable"]
    #[inline(always)]
    pub fn inpen(&self) -> InpenR {
        InpenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
This field controls input hysteresis"]
    #[inline(always)]
    pub fn hysten(&self) -> HystenR {
        HystenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Port configuration."]
    #[inline(always)]
    #[must_use]
    pub fn portcfg(&mut self) -> PortcfgW<Ioc2Spec> {
        PortcfgW::new(self, 0)
    }
    #[doc = "Bits 3:12 - 12:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Ioc2Spec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Pull control. Setting this to value 0x3 disables pull."]
    #[inline(always)]
    #[must_use]
    pub fn pullctl(&mut self) -> PullctlW<Ioc2Spec> {
        PullctlW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> Reserved15W<Ioc2Spec> {
        Reserved15W::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Edge detect configuration"]
    #[inline(always)]
    #[must_use]
    pub fn edgedet(&mut self) -> EdgedetW<Ioc2Spec> {
        EdgedetW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Wakeup enable from standby"]
    #[inline(always)]
    #[must_use]
    pub fn wuensb(&mut self) -> WuensbW<Ioc2Spec> {
        WuensbW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<Ioc2Spec> {
        Reserved19W::new(self, 19)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Wakeup configuration from shutdown"]
    #[inline(always)]
    #[must_use]
    pub fn wucfgsd(&mut self) -> WucfgsdW<Ioc2Spec> {
        WucfgsdW::new(self, 20)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<Ioc2Spec> {
        Reserved22W::new(self, 22)
    }
    #[doc = "Bits 24:26 - 26:24\\]
IO Mode. Setting this to value 0x6 or 0x7 will default to normal IO behavior."]
    #[inline(always)]
    #[must_use]
    pub fn iomode(&mut self) -> IomodeW<Ioc2Spec> {
        IomodeW::new(self, 24)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> Reserved27W<Ioc2Spec> {
        Reserved27W::new(self, 27)
    }
    #[doc = "Bit 29 - 29:29\\]
Input enable"]
    #[inline(always)]
    #[must_use]
    pub fn inpen(&mut self) -> InpenW<Ioc2Spec> {
        InpenW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
This field controls input hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hysten(&mut self) -> HystenW<Ioc2Spec> {
        HystenW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Ioc2Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Configuration of DIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioc2Spec;
impl crate::RegisterSpec for Ioc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioc2::R`](R) reader structure"]
impl crate::Readable for Ioc2Spec {}
#[doc = "`write(|w| ..)` method takes [`ioc2::W`](W) writer structure"]
impl crate::Writable for Ioc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOC2 to value 0"]
impl crate::Resettable for Ioc2Spec {
    const RESET_VALUE: u32 = 0;
}
