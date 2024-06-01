#[doc = "Register `SYSGPOCTL` reader"]
pub type R = crate::R<SysgpoctlSpec>;
#[doc = "Register `SYSGPOCTL` writer"]
pub type W = crate::W<SysgpoctlSpec>;
#[doc = "3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpoctl0 {
    #[doc = "15: RAT GPO line 3"]
    Ratgpo3 = 15,
    #[doc = "14: RAT GPO line 2"]
    Ratgpo2 = 14,
    #[doc = "13: RAT GPO line 1"]
    Ratgpo1 = 13,
    #[doc = "12: RAT GPO line 0"]
    Ratgpo0 = 12,
    #[doc = "11: RFE GPO line 3"]
    Rfegpo3 = 11,
    #[doc = "10: RFE GPO line 2"]
    Rfegpo2 = 10,
    #[doc = "9: RFE GPO line 1"]
    Rfegpo1 = 9,
    #[doc = "8: RFE GPO line 0"]
    Rfegpo0 = 8,
    #[doc = "7: MCE GPO line 3"]
    Mcegpo3 = 7,
    #[doc = "6: MCE GPO line 2"]
    Mcegpo2 = 6,
    #[doc = "5: MCE GPO line 1"]
    Mcegpo1 = 5,
    #[doc = "4: MCE GPO line 0"]
    Mcegpo0 = 4,
    #[doc = "3: CPE GPO line 3"]
    Cpegpo3 = 3,
    #[doc = "2: CPE GPO line 2"]
    Cpegpo2 = 2,
    #[doc = "1: CPE GPO line 1"]
    Cpegpo1 = 1,
    #[doc = "0: CPE GPO line 0"]
    Cpegpo0 = 0,
}
impl From<Gpoctl0> for u8 {
    #[inline(always)]
    fn from(variant: Gpoctl0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpoctl0 {
    type Ux = u8;
}
impl crate::IsEnum for Gpoctl0 {}
#[doc = "Field `GPOCTL0` reader - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
pub type Gpoctl0R = crate::FieldReader<Gpoctl0>;
impl Gpoctl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpoctl0 {
        match self.bits {
            15 => Gpoctl0::Ratgpo3,
            14 => Gpoctl0::Ratgpo2,
            13 => Gpoctl0::Ratgpo1,
            12 => Gpoctl0::Ratgpo0,
            11 => Gpoctl0::Rfegpo3,
            10 => Gpoctl0::Rfegpo2,
            9 => Gpoctl0::Rfegpo1,
            8 => Gpoctl0::Rfegpo0,
            7 => Gpoctl0::Mcegpo3,
            6 => Gpoctl0::Mcegpo2,
            5 => Gpoctl0::Mcegpo1,
            4 => Gpoctl0::Mcegpo0,
            3 => Gpoctl0::Cpegpo3,
            2 => Gpoctl0::Cpegpo2,
            1 => Gpoctl0::Cpegpo1,
            0 => Gpoctl0::Cpegpo0,
            _ => unreachable!(),
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == Gpoctl0::Ratgpo3
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == Gpoctl0::Ratgpo2
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == Gpoctl0::Ratgpo1
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == Gpoctl0::Ratgpo0
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Gpoctl0::Rfegpo3
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Gpoctl0::Rfegpo2
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Gpoctl0::Rfegpo1
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Gpoctl0::Rfegpo0
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Gpoctl0::Mcegpo3
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Gpoctl0::Mcegpo2
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Gpoctl0::Mcegpo1
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Gpoctl0::Mcegpo0
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == Gpoctl0::Cpegpo3
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == Gpoctl0::Cpegpo2
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == Gpoctl0::Cpegpo1
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == Gpoctl0::Cpegpo0
    }
}
#[doc = "Field `GPOCTL0` writer - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
pub type Gpoctl0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpoctl0, crate::Safe>;
impl<'a, REG> Gpoctl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Ratgpo3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Ratgpo2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Ratgpo1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Ratgpo0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Rfegpo3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Rfegpo2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Rfegpo1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Rfegpo0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Mcegpo3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Mcegpo2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Mcegpo1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Mcegpo0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Cpegpo3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Cpegpo2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Cpegpo1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl0::Cpegpo0)
    }
}
#[doc = "7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpoctl1 {
    #[doc = "15: RAT GPO line 3"]
    Ratgpo3 = 15,
    #[doc = "14: RAT GPO line 2"]
    Ratgpo2 = 14,
    #[doc = "13: RAT GPO line 1"]
    Ratgpo1 = 13,
    #[doc = "12: RAT GPO line 0"]
    Ratgpo0 = 12,
    #[doc = "11: RFE GPO line 3"]
    Rfegpo3 = 11,
    #[doc = "10: RFE GPO line 2"]
    Rfegpo2 = 10,
    #[doc = "9: RFE GPO line 1"]
    Rfegpo1 = 9,
    #[doc = "8: RFE GPO line 0"]
    Rfegpo0 = 8,
    #[doc = "7: MCE GPO line 3"]
    Mcegpo3 = 7,
    #[doc = "6: MCE GPO line 2"]
    Mcegpo2 = 6,
    #[doc = "5: MCE GPO line 1"]
    Mcegpo1 = 5,
    #[doc = "4: MCE GPO line 0"]
    Mcegpo0 = 4,
    #[doc = "3: CPE GPO line 3"]
    Cpegpo3 = 3,
    #[doc = "2: CPE GPO line 2"]
    Cpegpo2 = 2,
    #[doc = "1: CPE GPO line 1"]
    Cpegpo1 = 1,
    #[doc = "0: CPE GPO line 0"]
    Cpegpo0 = 0,
}
impl From<Gpoctl1> for u8 {
    #[inline(always)]
    fn from(variant: Gpoctl1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpoctl1 {
    type Ux = u8;
}
impl crate::IsEnum for Gpoctl1 {}
#[doc = "Field `GPOCTL1` reader - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
pub type Gpoctl1R = crate::FieldReader<Gpoctl1>;
impl Gpoctl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpoctl1 {
        match self.bits {
            15 => Gpoctl1::Ratgpo3,
            14 => Gpoctl1::Ratgpo2,
            13 => Gpoctl1::Ratgpo1,
            12 => Gpoctl1::Ratgpo0,
            11 => Gpoctl1::Rfegpo3,
            10 => Gpoctl1::Rfegpo2,
            9 => Gpoctl1::Rfegpo1,
            8 => Gpoctl1::Rfegpo0,
            7 => Gpoctl1::Mcegpo3,
            6 => Gpoctl1::Mcegpo2,
            5 => Gpoctl1::Mcegpo1,
            4 => Gpoctl1::Mcegpo0,
            3 => Gpoctl1::Cpegpo3,
            2 => Gpoctl1::Cpegpo2,
            1 => Gpoctl1::Cpegpo1,
            0 => Gpoctl1::Cpegpo0,
            _ => unreachable!(),
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == Gpoctl1::Ratgpo3
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == Gpoctl1::Ratgpo2
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == Gpoctl1::Ratgpo1
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == Gpoctl1::Ratgpo0
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Gpoctl1::Rfegpo3
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Gpoctl1::Rfegpo2
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Gpoctl1::Rfegpo1
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Gpoctl1::Rfegpo0
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Gpoctl1::Mcegpo3
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Gpoctl1::Mcegpo2
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Gpoctl1::Mcegpo1
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Gpoctl1::Mcegpo0
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == Gpoctl1::Cpegpo3
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == Gpoctl1::Cpegpo2
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == Gpoctl1::Cpegpo1
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == Gpoctl1::Cpegpo0
    }
}
#[doc = "Field `GPOCTL1` writer - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
pub type Gpoctl1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpoctl1, crate::Safe>;
impl<'a, REG> Gpoctl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Ratgpo3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Ratgpo2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Ratgpo1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Ratgpo0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Rfegpo3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Rfegpo2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Rfegpo1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Rfegpo0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Mcegpo3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Mcegpo2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Mcegpo1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Mcegpo0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Cpegpo3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Cpegpo2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Cpegpo1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl1::Cpegpo0)
    }
}
#[doc = "11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpoctl2 {
    #[doc = "15: RAT GPO line 3"]
    Ratgpo3 = 15,
    #[doc = "14: RAT GPO line 2"]
    Ratgpo2 = 14,
    #[doc = "13: RAT GPO line 1"]
    Ratgpo1 = 13,
    #[doc = "12: RAT GPO line 0"]
    Ratgpo0 = 12,
    #[doc = "11: RFE GPO line 3"]
    Rfegpo3 = 11,
    #[doc = "10: RFE GPO line 2"]
    Rfegpo2 = 10,
    #[doc = "9: RFE GPO line 1"]
    Rfegpo1 = 9,
    #[doc = "8: RFE GPO line 0"]
    Rfegpo0 = 8,
    #[doc = "7: MCE GPO line 3"]
    Mcegpo3 = 7,
    #[doc = "6: MCE GPO line 2"]
    Mcegpo2 = 6,
    #[doc = "5: MCE GPO line 1"]
    Mcegpo1 = 5,
    #[doc = "4: MCE GPO line 0"]
    Mcegpo0 = 4,
    #[doc = "3: CPE GPO line 3"]
    Cpegpo3 = 3,
    #[doc = "2: CPE GPO line 2"]
    Cpegpo2 = 2,
    #[doc = "1: CPE GPO line 1"]
    Cpegpo1 = 1,
    #[doc = "0: CPE GPO line 0"]
    Cpegpo0 = 0,
}
impl From<Gpoctl2> for u8 {
    #[inline(always)]
    fn from(variant: Gpoctl2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpoctl2 {
    type Ux = u8;
}
impl crate::IsEnum for Gpoctl2 {}
#[doc = "Field `GPOCTL2` reader - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
pub type Gpoctl2R = crate::FieldReader<Gpoctl2>;
impl Gpoctl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpoctl2 {
        match self.bits {
            15 => Gpoctl2::Ratgpo3,
            14 => Gpoctl2::Ratgpo2,
            13 => Gpoctl2::Ratgpo1,
            12 => Gpoctl2::Ratgpo0,
            11 => Gpoctl2::Rfegpo3,
            10 => Gpoctl2::Rfegpo2,
            9 => Gpoctl2::Rfegpo1,
            8 => Gpoctl2::Rfegpo0,
            7 => Gpoctl2::Mcegpo3,
            6 => Gpoctl2::Mcegpo2,
            5 => Gpoctl2::Mcegpo1,
            4 => Gpoctl2::Mcegpo0,
            3 => Gpoctl2::Cpegpo3,
            2 => Gpoctl2::Cpegpo2,
            1 => Gpoctl2::Cpegpo1,
            0 => Gpoctl2::Cpegpo0,
            _ => unreachable!(),
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == Gpoctl2::Ratgpo3
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == Gpoctl2::Ratgpo2
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == Gpoctl2::Ratgpo1
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == Gpoctl2::Ratgpo0
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Gpoctl2::Rfegpo3
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Gpoctl2::Rfegpo2
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Gpoctl2::Rfegpo1
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Gpoctl2::Rfegpo0
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Gpoctl2::Mcegpo3
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Gpoctl2::Mcegpo2
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Gpoctl2::Mcegpo1
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Gpoctl2::Mcegpo0
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == Gpoctl2::Cpegpo3
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == Gpoctl2::Cpegpo2
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == Gpoctl2::Cpegpo1
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == Gpoctl2::Cpegpo0
    }
}
#[doc = "Field `GPOCTL2` writer - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
pub type Gpoctl2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpoctl2, crate::Safe>;
impl<'a, REG> Gpoctl2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Ratgpo3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Ratgpo2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Ratgpo1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Ratgpo0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Rfegpo3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Rfegpo2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Rfegpo1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Rfegpo0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Mcegpo3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Mcegpo2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Mcegpo1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Mcegpo0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Cpegpo3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Cpegpo2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Cpegpo1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl2::Cpegpo0)
    }
}
#[doc = "15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpoctl3 {
    #[doc = "15: RAT GPO line 3"]
    Ratgpo3 = 15,
    #[doc = "14: RAT GPO line 2"]
    Ratgpo2 = 14,
    #[doc = "13: RAT GPO line 1"]
    Ratgpo1 = 13,
    #[doc = "12: RAT GPO line 0"]
    Ratgpo0 = 12,
    #[doc = "11: RFE GPO line 3"]
    Rfegpo3 = 11,
    #[doc = "10: RFE GPO line 2"]
    Rfegpo2 = 10,
    #[doc = "9: RFE GPO line 1"]
    Rfegpo1 = 9,
    #[doc = "8: RFE GPO line 0"]
    Rfegpo0 = 8,
    #[doc = "7: MCE GPO line 3"]
    Mcegpo3 = 7,
    #[doc = "6: MCE GPO line 2"]
    Mcegpo2 = 6,
    #[doc = "5: MCE GPO line 1"]
    Mcegpo1 = 5,
    #[doc = "4: MCE GPO line 0"]
    Mcegpo0 = 4,
    #[doc = "3: CPE GPO line 3"]
    Cpegpo3 = 3,
    #[doc = "2: CPE GPO line 2"]
    Cpegpo2 = 2,
    #[doc = "1: CPE GPO line 1"]
    Cpegpo1 = 1,
    #[doc = "0: CPE GPO line 0"]
    Cpegpo0 = 0,
}
impl From<Gpoctl3> for u8 {
    #[inline(always)]
    fn from(variant: Gpoctl3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpoctl3 {
    type Ux = u8;
}
impl crate::IsEnum for Gpoctl3 {}
#[doc = "Field `GPOCTL3` reader - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
pub type Gpoctl3R = crate::FieldReader<Gpoctl3>;
impl Gpoctl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpoctl3 {
        match self.bits {
            15 => Gpoctl3::Ratgpo3,
            14 => Gpoctl3::Ratgpo2,
            13 => Gpoctl3::Ratgpo1,
            12 => Gpoctl3::Ratgpo0,
            11 => Gpoctl3::Rfegpo3,
            10 => Gpoctl3::Rfegpo2,
            9 => Gpoctl3::Rfegpo1,
            8 => Gpoctl3::Rfegpo0,
            7 => Gpoctl3::Mcegpo3,
            6 => Gpoctl3::Mcegpo2,
            5 => Gpoctl3::Mcegpo1,
            4 => Gpoctl3::Mcegpo0,
            3 => Gpoctl3::Cpegpo3,
            2 => Gpoctl3::Cpegpo2,
            1 => Gpoctl3::Cpegpo1,
            0 => Gpoctl3::Cpegpo0,
            _ => unreachable!(),
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn is_ratgpo3(&self) -> bool {
        *self == Gpoctl3::Ratgpo3
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn is_ratgpo2(&self) -> bool {
        *self == Gpoctl3::Ratgpo2
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn is_ratgpo1(&self) -> bool {
        *self == Gpoctl3::Ratgpo1
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn is_ratgpo0(&self) -> bool {
        *self == Gpoctl3::Ratgpo0
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Gpoctl3::Rfegpo3
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Gpoctl3::Rfegpo2
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Gpoctl3::Rfegpo1
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Gpoctl3::Rfegpo0
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Gpoctl3::Mcegpo3
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Gpoctl3::Mcegpo2
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Gpoctl3::Mcegpo1
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Gpoctl3::Mcegpo0
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn is_cpegpo3(&self) -> bool {
        *self == Gpoctl3::Cpegpo3
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn is_cpegpo2(&self) -> bool {
        *self == Gpoctl3::Cpegpo2
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn is_cpegpo1(&self) -> bool {
        *self == Gpoctl3::Cpegpo1
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn is_cpegpo0(&self) -> bool {
        *self == Gpoctl3::Cpegpo0
    }
}
#[doc = "Field `GPOCTL3` writer - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
pub type Gpoctl3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpoctl3, crate::Safe>;
impl<'a, REG> Gpoctl3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAT GPO line 3"]
    #[inline(always)]
    pub fn ratgpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Ratgpo3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline(always)]
    pub fn ratgpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Ratgpo2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline(always)]
    pub fn ratgpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Ratgpo1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline(always)]
    pub fn ratgpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Ratgpo0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Rfegpo3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Rfegpo2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Rfegpo1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Rfegpo0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Mcegpo3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Mcegpo2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Mcegpo1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Mcegpo0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline(always)]
    pub fn cpegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Cpegpo3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline(always)]
    pub fn cpegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Cpegpo2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline(always)]
    pub fn cpegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Cpegpo1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline(always)]
    pub fn cpegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpoctl3::Cpegpo0)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline(always)]
    pub fn gpoctl0(&self) -> Gpoctl0R {
        Gpoctl0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline(always)]
    pub fn gpoctl1(&self) -> Gpoctl1R {
        Gpoctl1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline(always)]
    pub fn gpoctl2(&self) -> Gpoctl2R {
        Gpoctl2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline(always)]
    pub fn gpoctl3(&self) -> Gpoctl3R {
        Gpoctl3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl0(&mut self) -> Gpoctl0W<SysgpoctlSpec> {
        Gpoctl0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl1(&mut self) -> Gpoctl1W<SysgpoctlSpec> {
        Gpoctl1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl2(&mut self) -> Gpoctl2W<SysgpoctlSpec> {
        Gpoctl2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline(always)]
    #[must_use]
    pub fn gpoctl3(&mut self) -> Gpoctl3W<SysgpoctlSpec> {
        Gpoctl3W::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<SysgpoctlSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "RF Core General Purpose Output Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysgpoctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysgpoctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysgpoctlSpec;
impl crate::RegisterSpec for SysgpoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysgpoctl::R`](R) reader structure"]
impl crate::Readable for SysgpoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysgpoctl::W`](W) writer structure"]
impl crate::Writable for SysgpoctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSGPOCTL to value 0"]
impl crate::Resettable for SysgpoctlSpec {
    const RESET_VALUE: u32 = 0;
}
