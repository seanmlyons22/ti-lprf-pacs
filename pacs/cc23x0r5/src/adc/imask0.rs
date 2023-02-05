#[doc = "Register `IMASK0` reader"]
pub type R = crate::R<Imask0Spec>;
#[doc = "Register `IMASK0` writer"]
pub type W = crate::W<Imask0Spec>;
#[doc = "0:0\\]
Conversion overflow interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovifg {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Ovifg> for bool {
    #[inline(always)]
    fn from(variant: Ovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVIFG` reader - 0:0\\]
Conversion overflow interrupt mask"]
pub type OvifgR = crate::BitReader<Ovifg>;
impl OvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovifg {
        match self.bits {
            true => Ovifg::Ena,
            false => Ovifg::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Ovifg::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ovifg::Dis
    }
}
#[doc = "Field `OVIFG` writer - 0:0\\]
Conversion overflow interrupt mask"]
pub type OvifgW<'a, REG> = crate::BitWriter<'a, REG, Ovifg>;
impl<'a, REG> OvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Ovifg::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ovifg::Dis)
    }
}
#[doc = "1:1\\]
Sequence conversion time overflow interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tovifg {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Tovifg> for bool {
    #[inline(always)]
    fn from(variant: Tovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVIFG` reader - 1:1\\]
Sequence conversion time overflow interrupt mask"]
pub type TovifgR = crate::BitReader<Tovifg>;
impl TovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tovifg {
        match self.bits {
            true => Tovifg::Ena,
            false => Tovifg::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Tovifg::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tovifg::Dis
    }
}
#[doc = "Field `TOVIFG` writer - 1:1\\]
Sequence conversion time overflow interrupt mask"]
pub type TovifgW<'a, REG> = crate::BitWriter<'a, REG, Tovifg>;
impl<'a, REG> TovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Tovifg::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tovifg::Dis)
    }
}
#[doc = "2:2\\]
High threshold compare interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highifg {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Highifg> for bool {
    #[inline(always)]
    fn from(variant: Highifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHIFG` reader - 2:2\\]
High threshold compare interrupt mask"]
pub type HighifgR = crate::BitReader<Highifg>;
impl HighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highifg {
        match self.bits {
            true => Highifg::Ena,
            false => Highifg::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Highifg::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Highifg::Dis
    }
}
#[doc = "Field `HIGHIFG` writer - 2:2\\]
High threshold compare interrupt mask"]
pub type HighifgW<'a, REG> = crate::BitWriter<'a, REG, Highifg>;
impl<'a, REG> HighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Highifg::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Highifg::Dis)
    }
}
#[doc = "3:3\\]
Low threshold compare interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowifg {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Lowifg> for bool {
    #[inline(always)]
    fn from(variant: Lowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWIFG` reader - 3:3\\]
Low threshold compare interrupt mask"]
pub type LowifgR = crate::BitReader<Lowifg>;
impl LowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowifg {
        match self.bits {
            true => Lowifg::Ena,
            false => Lowifg::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Lowifg::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lowifg::Dis
    }
}
#[doc = "Field `LOWIFG` writer - 3:3\\]
Low threshold compare interrupt mask"]
pub type LowifgW<'a, REG> = crate::BitWriter<'a, REG, Lowifg>;
impl<'a, REG> LowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Lowifg::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lowifg::Dis)
    }
}
#[doc = "4:4\\]
In-range comparator interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inifg {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Inifg> for bool {
    #[inline(always)]
    fn from(variant: Inifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIFG` reader - 4:4\\]
In-range comparator interrupt mask."]
pub type InifgR = crate::BitReader<Inifg>;
impl InifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inifg {
        match self.bits {
            true => Inifg::Ena,
            false => Inifg::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Inifg::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Inifg::Dis
    }
}
#[doc = "Field `INIFG` writer - 4:4\\]
In-range comparator interrupt mask."]
pub type InifgW<'a, REG> = crate::BitWriter<'a, REG, Inifg>;
impl<'a, REG> InifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Inifg::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Inifg::Dis)
    }
}
#[doc = "Field `RESERVED5` reader - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::BitReader;
#[doc = "6:6\\]
Conversion underflow interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uvifg {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Uvifg> for bool {
    #[inline(always)]
    fn from(variant: Uvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVIFG` reader - 6:6\\]
Conversion underflow interrupt mask"]
pub type UvifgR = crate::BitReader<Uvifg>;
impl UvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uvifg {
        match self.bits {
            true => Uvifg::Ena,
            false => Uvifg::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Uvifg::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Uvifg::Dis
    }
}
#[doc = "Field `UVIFG` writer - 6:6\\]
Conversion underflow interrupt mask"]
pub type UvifgW<'a, REG> = crate::BitWriter<'a, REG, Uvifg>;
impl<'a, REG> UvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Uvifg::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Uvifg::Dis)
    }
}
#[doc = "7:7\\]
Mask for ASC done raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascdone {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Ascdone> for bool {
    #[inline(always)]
    fn from(variant: Ascdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCDONE` reader - 7:7\\]
Mask for ASC done raw interrupt flag"]
pub type AscdoneR = crate::BitReader<Ascdone>;
impl AscdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascdone {
        match self.bits {
            true => Ascdone::Ena,
            false => Ascdone::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Ascdone::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ascdone::Dis
    }
}
#[doc = "Field `ASCDONE` writer - 7:7\\]
Mask for ASC done raw interrupt flag"]
pub type AscdoneW<'a, REG> = crate::BitWriter<'a, REG, Ascdone>;
impl<'a, REG> AscdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Ascdone::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ascdone::Dis)
    }
}
#[doc = "8:8\\]
MEMRES0 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg0 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg0> for bool {
    #[inline(always)]
    fn from(variant: Memresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG0` reader - 8:8\\]
MEMRES0 conversion result interrupt mask."]
pub type Memresifg0R = crate::BitReader<Memresifg0>;
impl Memresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg0 {
        match self.bits {
            true => Memresifg0::Ena,
            false => Memresifg0::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg0::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg0::Dis
    }
}
#[doc = "Field `MEMRESIFG0` writer - 8:8\\]
MEMRES0 conversion result interrupt mask."]
pub type Memresifg0W<'a, REG> = crate::BitWriter<'a, REG, Memresifg0>;
impl<'a, REG> Memresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Dis)
    }
}
#[doc = "9:9\\]
MEMRES1 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg1 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg1> for bool {
    #[inline(always)]
    fn from(variant: Memresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG1` reader - 9:9\\]
MEMRES1 conversion result interrupt mask."]
pub type Memresifg1R = crate::BitReader<Memresifg1>;
impl Memresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg1 {
        match self.bits {
            true => Memresifg1::Ena,
            false => Memresifg1::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg1::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg1::Dis
    }
}
#[doc = "Field `MEMRESIFG1` writer - 9:9\\]
MEMRES1 conversion result interrupt mask."]
pub type Memresifg1W<'a, REG> = crate::BitWriter<'a, REG, Memresifg1>;
impl<'a, REG> Memresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg1::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg1::Dis)
    }
}
#[doc = "10:10\\]
MEMRES2 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg2 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg2> for bool {
    #[inline(always)]
    fn from(variant: Memresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG2` reader - 10:10\\]
MEMRES2 conversion result interrupt mask."]
pub type Memresifg2R = crate::BitReader<Memresifg2>;
impl Memresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg2 {
        match self.bits {
            true => Memresifg2::Ena,
            false => Memresifg2::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg2::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg2::Dis
    }
}
#[doc = "Field `MEMRESIFG2` writer - 10:10\\]
MEMRES2 conversion result interrupt mask."]
pub type Memresifg2W<'a, REG> = crate::BitWriter<'a, REG, Memresifg2>;
impl<'a, REG> Memresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg2::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg2::Dis)
    }
}
#[doc = "11:11\\]
MEMRES3 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg3 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg3> for bool {
    #[inline(always)]
    fn from(variant: Memresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG3` reader - 11:11\\]
MEMRES3 conversion result interrupt mask."]
pub type Memresifg3R = crate::BitReader<Memresifg3>;
impl Memresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg3 {
        match self.bits {
            true => Memresifg3::Ena,
            false => Memresifg3::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg3::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg3::Dis
    }
}
#[doc = "Field `MEMRESIFG3` writer - 11:11\\]
MEMRES3 conversion result interrupt mask."]
pub type Memresifg3W<'a, REG> = crate::BitWriter<'a, REG, Memresifg3>;
impl<'a, REG> Memresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg3::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg3::Dis)
    }
}
#[doc = "12:12\\]
MEMRES4 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg4 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg4> for bool {
    #[inline(always)]
    fn from(variant: Memresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG4` reader - 12:12\\]
MEMRES4 conversion result interrupt mask."]
pub type Memresifg4R = crate::BitReader<Memresifg4>;
impl Memresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg4 {
        match self.bits {
            true => Memresifg4::Ena,
            false => Memresifg4::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg4::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg4::Dis
    }
}
#[doc = "Field `MEMRESIFG4` writer - 12:12\\]
MEMRES4 conversion result interrupt mask."]
pub type Memresifg4W<'a, REG> = crate::BitWriter<'a, REG, Memresifg4>;
impl<'a, REG> Memresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg4::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg4::Dis)
    }
}
#[doc = "13:13\\]
MEMRES5 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg5 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg5> for bool {
    #[inline(always)]
    fn from(variant: Memresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG5` reader - 13:13\\]
MEMRES5 conversion result interrupt mask."]
pub type Memresifg5R = crate::BitReader<Memresifg5>;
impl Memresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg5 {
        match self.bits {
            true => Memresifg5::Ena,
            false => Memresifg5::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg5::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg5::Dis
    }
}
#[doc = "Field `MEMRESIFG5` writer - 13:13\\]
MEMRES5 conversion result interrupt mask."]
pub type Memresifg5W<'a, REG> = crate::BitWriter<'a, REG, Memresifg5>;
impl<'a, REG> Memresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg5::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg5::Dis)
    }
}
#[doc = "14:14\\]
MEMRES6 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg6 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg6> for bool {
    #[inline(always)]
    fn from(variant: Memresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG6` reader - 14:14\\]
MEMRES6 conversion result interrupt mask."]
pub type Memresifg6R = crate::BitReader<Memresifg6>;
impl Memresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg6 {
        match self.bits {
            true => Memresifg6::Ena,
            false => Memresifg6::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg6::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg6::Dis
    }
}
#[doc = "Field `MEMRESIFG6` writer - 14:14\\]
MEMRES6 conversion result interrupt mask."]
pub type Memresifg6W<'a, REG> = crate::BitWriter<'a, REG, Memresifg6>;
impl<'a, REG> Memresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg6::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg6::Dis)
    }
}
#[doc = "15:15\\]
MEMRES7 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg7 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg7> for bool {
    #[inline(always)]
    fn from(variant: Memresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG7` reader - 15:15\\]
MEMRES7 conversion result interrupt mask."]
pub type Memresifg7R = crate::BitReader<Memresifg7>;
impl Memresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg7 {
        match self.bits {
            true => Memresifg7::Ena,
            false => Memresifg7::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg7::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg7::Dis
    }
}
#[doc = "Field `MEMRESIFG7` writer - 15:15\\]
MEMRES7 conversion result interrupt mask."]
pub type Memresifg7W<'a, REG> = crate::BitWriter<'a, REG, Memresifg7>;
impl<'a, REG> Memresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg7::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg7::Dis)
    }
}
#[doc = "16:16\\]
MEMRES8 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg8 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg8> for bool {
    #[inline(always)]
    fn from(variant: Memresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG8` reader - 16:16\\]
MEMRES8 conversion result interrupt mask."]
pub type Memresifg8R = crate::BitReader<Memresifg8>;
impl Memresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg8 {
        match self.bits {
            true => Memresifg8::Ena,
            false => Memresifg8::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg8::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg8::Dis
    }
}
#[doc = "Field `MEMRESIFG8` writer - 16:16\\]
MEMRES8 conversion result interrupt mask."]
pub type Memresifg8W<'a, REG> = crate::BitWriter<'a, REG, Memresifg8>;
impl<'a, REG> Memresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg8::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg8::Dis)
    }
}
#[doc = "17:17\\]
MEMRES9 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg9 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg9> for bool {
    #[inline(always)]
    fn from(variant: Memresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG9` reader - 17:17\\]
MEMRES9 conversion result interrupt mask."]
pub type Memresifg9R = crate::BitReader<Memresifg9>;
impl Memresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg9 {
        match self.bits {
            true => Memresifg9::Ena,
            false => Memresifg9::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg9::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg9::Dis
    }
}
#[doc = "Field `MEMRESIFG9` writer - 17:17\\]
MEMRES9 conversion result interrupt mask."]
pub type Memresifg9W<'a, REG> = crate::BitWriter<'a, REG, Memresifg9>;
impl<'a, REG> Memresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg9::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg9::Dis)
    }
}
#[doc = "18:18\\]
MEMRES10 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg10 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg10> for bool {
    #[inline(always)]
    fn from(variant: Memresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG10` reader - 18:18\\]
MEMRES10 conversion result interrupt mask."]
pub type Memresifg10R = crate::BitReader<Memresifg10>;
impl Memresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg10 {
        match self.bits {
            true => Memresifg10::Ena,
            false => Memresifg10::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg10::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg10::Dis
    }
}
#[doc = "Field `MEMRESIFG10` writer - 18:18\\]
MEMRES10 conversion result interrupt mask."]
pub type Memresifg10W<'a, REG> = crate::BitWriter<'a, REG, Memresifg10>;
impl<'a, REG> Memresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg10::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg10::Dis)
    }
}
#[doc = "19:19\\]
MEMRES11 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg11 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg11> for bool {
    #[inline(always)]
    fn from(variant: Memresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG11` reader - 19:19\\]
MEMRES11 conversion result interrupt mask."]
pub type Memresifg11R = crate::BitReader<Memresifg11>;
impl Memresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg11 {
        match self.bits {
            true => Memresifg11::Ena,
            false => Memresifg11::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg11::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg11::Dis
    }
}
#[doc = "Field `MEMRESIFG11` writer - 19:19\\]
MEMRES11 conversion result interrupt mask."]
pub type Memresifg11W<'a, REG> = crate::BitWriter<'a, REG, Memresifg11>;
impl<'a, REG> Memresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg11::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg11::Dis)
    }
}
#[doc = "20:20\\]
MEMRES12 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg12 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg12> for bool {
    #[inline(always)]
    fn from(variant: Memresifg12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG12` reader - 20:20\\]
MEMRES12 conversion result interrupt mask."]
pub type Memresifg12R = crate::BitReader<Memresifg12>;
impl Memresifg12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg12 {
        match self.bits {
            true => Memresifg12::Ena,
            false => Memresifg12::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg12::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg12::Dis
    }
}
#[doc = "Field `MEMRESIFG12` writer - 20:20\\]
MEMRES12 conversion result interrupt mask."]
pub type Memresifg12W<'a, REG> = crate::BitWriter<'a, REG, Memresifg12>;
impl<'a, REG> Memresifg12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg12::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg12::Dis)
    }
}
#[doc = "21:21\\]
MEMRES13 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg13 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg13> for bool {
    #[inline(always)]
    fn from(variant: Memresifg13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG13` reader - 21:21\\]
MEMRES13 conversion result interrupt mask."]
pub type Memresifg13R = crate::BitReader<Memresifg13>;
impl Memresifg13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg13 {
        match self.bits {
            true => Memresifg13::Ena,
            false => Memresifg13::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg13::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg13::Dis
    }
}
#[doc = "Field `MEMRESIFG13` writer - 21:21\\]
MEMRES13 conversion result interrupt mask."]
pub type Memresifg13W<'a, REG> = crate::BitWriter<'a, REG, Memresifg13>;
impl<'a, REG> Memresifg13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg13::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg13::Dis)
    }
}
#[doc = "22:22\\]
MEMRES14 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg14 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg14> for bool {
    #[inline(always)]
    fn from(variant: Memresifg14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG14` reader - 22:22\\]
MEMRES14 conversion result interrupt mask."]
pub type Memresifg14R = crate::BitReader<Memresifg14>;
impl Memresifg14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg14 {
        match self.bits {
            true => Memresifg14::Ena,
            false => Memresifg14::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg14::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg14::Dis
    }
}
#[doc = "Field `MEMRESIFG14` writer - 22:22\\]
MEMRES14 conversion result interrupt mask."]
pub type Memresifg14W<'a, REG> = crate::BitWriter<'a, REG, Memresifg14>;
impl<'a, REG> Memresifg14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg14::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg14::Dis)
    }
}
#[doc = "23:23\\]
MEMRES15 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg15 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg15> for bool {
    #[inline(always)]
    fn from(variant: Memresifg15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG15` reader - 23:23\\]
MEMRES15 conversion result interrupt mask."]
pub type Memresifg15R = crate::BitReader<Memresifg15>;
impl Memresifg15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg15 {
        match self.bits {
            true => Memresifg15::Ena,
            false => Memresifg15::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg15::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg15::Dis
    }
}
#[doc = "Field `MEMRESIFG15` writer - 23:23\\]
MEMRES15 conversion result interrupt mask."]
pub type Memresifg15W<'a, REG> = crate::BitWriter<'a, REG, Memresifg15>;
impl<'a, REG> Memresifg15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg15::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg15::Dis)
    }
}
#[doc = "24:24\\]
MEMRES16 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg16 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg16> for bool {
    #[inline(always)]
    fn from(variant: Memresifg16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG16` reader - 24:24\\]
MEMRES16 conversion result interrupt mask."]
pub type Memresifg16R = crate::BitReader<Memresifg16>;
impl Memresifg16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg16 {
        match self.bits {
            true => Memresifg16::Ena,
            false => Memresifg16::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg16::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg16::Dis
    }
}
#[doc = "Field `MEMRESIFG16` writer - 24:24\\]
MEMRES16 conversion result interrupt mask."]
pub type Memresifg16W<'a, REG> = crate::BitWriter<'a, REG, Memresifg16>;
impl<'a, REG> Memresifg16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg16::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg16::Dis)
    }
}
#[doc = "25:25\\]
MEMRES17 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg17 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg17> for bool {
    #[inline(always)]
    fn from(variant: Memresifg17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG17` reader - 25:25\\]
MEMRES17 conversion result interrupt mask."]
pub type Memresifg17R = crate::BitReader<Memresifg17>;
impl Memresifg17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg17 {
        match self.bits {
            true => Memresifg17::Ena,
            false => Memresifg17::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg17::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg17::Dis
    }
}
#[doc = "Field `MEMRESIFG17` writer - 25:25\\]
MEMRES17 conversion result interrupt mask."]
pub type Memresifg17W<'a, REG> = crate::BitWriter<'a, REG, Memresifg17>;
impl<'a, REG> Memresifg17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg17::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg17::Dis)
    }
}
#[doc = "26:26\\]
MEMRES18 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg18 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg18> for bool {
    #[inline(always)]
    fn from(variant: Memresifg18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG18` reader - 26:26\\]
MEMRES18 conversion result interrupt mask."]
pub type Memresifg18R = crate::BitReader<Memresifg18>;
impl Memresifg18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg18 {
        match self.bits {
            true => Memresifg18::Ena,
            false => Memresifg18::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg18::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg18::Dis
    }
}
#[doc = "Field `MEMRESIFG18` writer - 26:26\\]
MEMRES18 conversion result interrupt mask."]
pub type Memresifg18W<'a, REG> = crate::BitWriter<'a, REG, Memresifg18>;
impl<'a, REG> Memresifg18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg18::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg18::Dis)
    }
}
#[doc = "27:27\\]
MEMRES19 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg19 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg19> for bool {
    #[inline(always)]
    fn from(variant: Memresifg19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG19` reader - 27:27\\]
MEMRES19 conversion result interrupt mask."]
pub type Memresifg19R = crate::BitReader<Memresifg19>;
impl Memresifg19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg19 {
        match self.bits {
            true => Memresifg19::Ena,
            false => Memresifg19::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg19::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg19::Dis
    }
}
#[doc = "Field `MEMRESIFG19` writer - 27:27\\]
MEMRES19 conversion result interrupt mask."]
pub type Memresifg19W<'a, REG> = crate::BitWriter<'a, REG, Memresifg19>;
impl<'a, REG> Memresifg19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg19::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg19::Dis)
    }
}
#[doc = "28:28\\]
MEMRES20 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg20 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg20> for bool {
    #[inline(always)]
    fn from(variant: Memresifg20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG20` reader - 28:28\\]
MEMRES20 conversion result interrupt mask."]
pub type Memresifg20R = crate::BitReader<Memresifg20>;
impl Memresifg20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg20 {
        match self.bits {
            true => Memresifg20::Ena,
            false => Memresifg20::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg20::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg20::Dis
    }
}
#[doc = "Field `MEMRESIFG20` writer - 28:28\\]
MEMRES20 conversion result interrupt mask."]
pub type Memresifg20W<'a, REG> = crate::BitWriter<'a, REG, Memresifg20>;
impl<'a, REG> Memresifg20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg20::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg20::Dis)
    }
}
#[doc = "29:29\\]
MEMRES21 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg21 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg21> for bool {
    #[inline(always)]
    fn from(variant: Memresifg21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG21` reader - 29:29\\]
MEMRES21 conversion result interrupt mask."]
pub type Memresifg21R = crate::BitReader<Memresifg21>;
impl Memresifg21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg21 {
        match self.bits {
            true => Memresifg21::Ena,
            false => Memresifg21::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg21::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg21::Dis
    }
}
#[doc = "Field `MEMRESIFG21` writer - 29:29\\]
MEMRES21 conversion result interrupt mask."]
pub type Memresifg21W<'a, REG> = crate::BitWriter<'a, REG, Memresifg21>;
impl<'a, REG> Memresifg21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg21::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg21::Dis)
    }
}
#[doc = "30:30\\]
MEMRES22 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg22 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg22> for bool {
    #[inline(always)]
    fn from(variant: Memresifg22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG22` reader - 30:30\\]
MEMRES22 conversion result interrupt mask."]
pub type Memresifg22R = crate::BitReader<Memresifg22>;
impl Memresifg22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg22 {
        match self.bits {
            true => Memresifg22::Ena,
            false => Memresifg22::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg22::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg22::Dis
    }
}
#[doc = "Field `MEMRESIFG22` writer - 30:30\\]
MEMRES22 conversion result interrupt mask."]
pub type Memresifg22W<'a, REG> = crate::BitWriter<'a, REG, Memresifg22>;
impl<'a, REG> Memresifg22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg22::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg22::Dis)
    }
}
#[doc = "31:31\\]
MEMRES23 conversion result interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg23 {
    #[doc = "1: Enable interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable interrupt mask"]
    Dis = 0,
}
impl From<Memresifg23> for bool {
    #[inline(always)]
    fn from(variant: Memresifg23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG23` reader - 31:31\\]
MEMRES23 conversion result interrupt mask."]
pub type Memresifg23R = crate::BitReader<Memresifg23>;
impl Memresifg23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg23 {
        match self.bits {
            true => Memresifg23::Ena,
            false => Memresifg23::Dis,
        }
    }
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Memresifg23::Ena
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Memresifg23::Dis
    }
}
#[doc = "Field `MEMRESIFG23` writer - 31:31\\]
MEMRES23 conversion result interrupt mask."]
pub type Memresifg23W<'a, REG> = crate::BitWriter<'a, REG, Memresifg23>;
impl<'a, REG> Memresifg23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg23::Ena)
    }
    #[doc = "Disable interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg23::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Conversion overflow interrupt mask"]
    #[inline(always)]
    pub fn ovifg(&self) -> OvifgR {
        OvifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sequence conversion time overflow interrupt mask"]
    #[inline(always)]
    pub fn tovifg(&self) -> TovifgR {
        TovifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
High threshold compare interrupt mask"]
    #[inline(always)]
    pub fn highifg(&self) -> HighifgR {
        HighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Low threshold compare interrupt mask"]
    #[inline(always)]
    pub fn lowifg(&self) -> LowifgR {
        LowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
In-range comparator interrupt mask."]
    #[inline(always)]
    pub fn inifg(&self) -> InifgR {
        InifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Conversion underflow interrupt mask"]
    #[inline(always)]
    pub fn uvifg(&self) -> UvifgR {
        UvifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Mask for ASC done raw interrupt flag"]
    #[inline(always)]
    pub fn ascdone(&self) -> AscdoneR {
        AscdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MEMRES0 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg0(&self) -> Memresifg0R {
        Memresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
MEMRES1 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg1(&self) -> Memresifg1R {
        Memresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
MEMRES2 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg2(&self) -> Memresifg2R {
        Memresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
MEMRES3 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg3(&self) -> Memresifg3R {
        Memresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
MEMRES4 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg4(&self) -> Memresifg4R {
        Memresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MEMRES5 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg5(&self) -> Memresifg5R {
        Memresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
MEMRES6 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg6(&self) -> Memresifg6R {
        Memresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
MEMRES7 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg7(&self) -> Memresifg7R {
        Memresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MEMRES8 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg8(&self) -> Memresifg8R {
        Memresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
MEMRES9 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg9(&self) -> Memresifg9R {
        Memresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
MEMRES10 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg10(&self) -> Memresifg10R {
        Memresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
MEMRES11 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg11(&self) -> Memresifg11R {
        Memresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
MEMRES12 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg12(&self) -> Memresifg12R {
        Memresifg12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
MEMRES13 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg13(&self) -> Memresifg13R {
        Memresifg13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
MEMRES14 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg14(&self) -> Memresifg14R {
        Memresifg14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
MEMRES15 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg15(&self) -> Memresifg15R {
        Memresifg15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MEMRES16 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg16(&self) -> Memresifg16R {
        Memresifg16R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
MEMRES17 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg17(&self) -> Memresifg17R {
        Memresifg17R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
MEMRES18 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg18(&self) -> Memresifg18R {
        Memresifg18R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
MEMRES19 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg19(&self) -> Memresifg19R {
        Memresifg19R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
MEMRES20 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg20(&self) -> Memresifg20R {
        Memresifg20R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
MEMRES21 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg21(&self) -> Memresifg21R {
        Memresifg21R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
MEMRES22 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg22(&self) -> Memresifg22R {
        Memresifg22R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MEMRES23 conversion result interrupt mask."]
    #[inline(always)]
    pub fn memresifg23(&self) -> Memresifg23R {
        Memresifg23R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Conversion overflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ovifg(&mut self) -> OvifgW<Imask0Spec> {
        OvifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sequence conversion time overflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tovifg(&mut self) -> TovifgW<Imask0Spec> {
        TovifgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
High threshold compare interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn highifg(&mut self) -> HighifgW<Imask0Spec> {
        HighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Low threshold compare interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn lowifg(&mut self) -> LowifgW<Imask0Spec> {
        LowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
In-range comparator interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn inifg(&mut self) -> InifgW<Imask0Spec> {
        InifgW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Conversion underflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn uvifg(&mut self) -> UvifgW<Imask0Spec> {
        UvifgW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Mask for ASC done raw interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ascdone(&mut self) -> AscdoneW<Imask0Spec> {
        AscdoneW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
MEMRES0 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg0(&mut self) -> Memresifg0W<Imask0Spec> {
        Memresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
MEMRES1 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg1(&mut self) -> Memresifg1W<Imask0Spec> {
        Memresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
MEMRES2 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg2(&mut self) -> Memresifg2W<Imask0Spec> {
        Memresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
MEMRES3 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg3(&mut self) -> Memresifg3W<Imask0Spec> {
        Memresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
MEMRES4 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg4(&mut self) -> Memresifg4W<Imask0Spec> {
        Memresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
MEMRES5 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg5(&mut self) -> Memresifg5W<Imask0Spec> {
        Memresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
MEMRES6 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg6(&mut self) -> Memresifg6W<Imask0Spec> {
        Memresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
MEMRES7 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg7(&mut self) -> Memresifg7W<Imask0Spec> {
        Memresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
MEMRES8 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg8(&mut self) -> Memresifg8W<Imask0Spec> {
        Memresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
MEMRES9 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg9(&mut self) -> Memresifg9W<Imask0Spec> {
        Memresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
MEMRES10 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg10(&mut self) -> Memresifg10W<Imask0Spec> {
        Memresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
MEMRES11 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg11(&mut self) -> Memresifg11W<Imask0Spec> {
        Memresifg11W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
MEMRES12 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg12(&mut self) -> Memresifg12W<Imask0Spec> {
        Memresifg12W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
MEMRES13 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg13(&mut self) -> Memresifg13W<Imask0Spec> {
        Memresifg13W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
MEMRES14 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg14(&mut self) -> Memresifg14W<Imask0Spec> {
        Memresifg14W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
MEMRES15 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg15(&mut self) -> Memresifg15W<Imask0Spec> {
        Memresifg15W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
MEMRES16 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg16(&mut self) -> Memresifg16W<Imask0Spec> {
        Memresifg16W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
MEMRES17 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg17(&mut self) -> Memresifg17W<Imask0Spec> {
        Memresifg17W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
MEMRES18 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg18(&mut self) -> Memresifg18W<Imask0Spec> {
        Memresifg18W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
MEMRES19 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg19(&mut self) -> Memresifg19W<Imask0Spec> {
        Memresifg19W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
MEMRES20 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg20(&mut self) -> Memresifg20W<Imask0Spec> {
        Memresifg20W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
MEMRES21 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg21(&mut self) -> Memresifg21W<Imask0Spec> {
        Memresifg21W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
MEMRES22 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg22(&mut self) -> Memresifg22W<Imask0Spec> {
        Memresifg22W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
MEMRES23 conversion result interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn memresifg23(&mut self) -> Memresifg23W<Imask0Spec> {
        Memresifg23W::new(self, 31)
    }
}
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS0 to MIS0 when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imask0Spec;
impl crate::RegisterSpec for Imask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask0::R`](R) reader structure"]
impl crate::Readable for Imask0Spec {}
#[doc = "`write(|w| ..)` method takes [`imask0::W`](W) writer structure"]
impl crate::Writable for Imask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK0 to value 0"]
impl crate::Resettable for Imask0Spec {
    const RESET_VALUE: u32 = 0;
}
