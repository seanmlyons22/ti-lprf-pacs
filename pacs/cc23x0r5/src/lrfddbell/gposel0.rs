#[doc = "Register `GPOSEL0` reader"]
pub type R = crate::R<Gposel0Spec>;
#[doc = "Register `GPOSEL0` writer"]
pub type W = crate::W<Gposel0Spec>;
#[doc = "4:0\\]
Select source of radio GPO line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src0 {
    #[doc = "25: Select RFCTRC GPO line 0"]
    Rfctrc = 25,
    #[doc = "24: Select RFE GPO line 7"]
    Rfegpo7 = 24,
    #[doc = "23: Select RFE GPO line 6"]
    Rfegpo6 = 23,
    #[doc = "22: Select RFE GPO line 5"]
    Rfegpo5 = 22,
    #[doc = "21: Select RFE GPO line 4"]
    Rfegpo4 = 21,
    #[doc = "20: Select RFE GPO line 3"]
    Rfegpo3 = 20,
    #[doc = "19: Select RFE GPO line 2"]
    Rfegpo2 = 19,
    #[doc = "18: Select RFE GPO line 1"]
    Rfegpo1 = 18,
    #[doc = "17: Select RFE GPO line 0"]
    Rfegpo0 = 17,
    #[doc = "16: Select MCE GPO line 7"]
    Mcegpo7 = 16,
    #[doc = "15: Select MCE GPO line 6"]
    Mcegpo6 = 15,
    #[doc = "14: Select MCE GPO line 5"]
    Mcegpo5 = 14,
    #[doc = "13: Select MCE GPO line 4"]
    Mcegpo4 = 13,
    #[doc = "12: Select MCE GPO line 3"]
    Mcegpo3 = 12,
    #[doc = "11: Select MCE GPO line 2"]
    Mcegpo2 = 11,
    #[doc = "10: Select MCE GPO line 1"]
    Mcegpo1 = 10,
    #[doc = "9: Select MCE GPO line 0"]
    Mcegpo0 = 9,
    #[doc = "8: Select PBE GPO line 7"]
    Pbegpo7 = 8,
    #[doc = "7: Select PBE GPO line 6"]
    Pbegpo6 = 7,
    #[doc = "6: Select PBE GPO line 5"]
    Pbegpo5 = 6,
    #[doc = "5: Select PBE GPO line 4"]
    Pbegpo4 = 5,
    #[doc = "4: Select PBE GPO line 3"]
    Pbegpo3 = 4,
    #[doc = "3: Select PBE GPO line 2"]
    Pbegpo2 = 3,
    #[doc = "2: Select PBE GPO line 1"]
    Pbegpo1 = 2,
    #[doc = "1: Select PBE GPO line 0"]
    Pbegpo0 = 1,
    #[doc = "0: Output not enabled"]
    Dis = 0,
}
impl From<Src0> for u8 {
    #[inline(always)]
    fn from(variant: Src0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src0 {
    type Ux = u8;
}
impl crate::IsEnum for Src0 {}
#[doc = "Field `SRC0` reader - 4:0\\]
Select source of radio GPO line 0"]
pub type Src0R = crate::FieldReader<Src0>;
impl Src0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src0> {
        match self.bits {
            25 => Some(Src0::Rfctrc),
            24 => Some(Src0::Rfegpo7),
            23 => Some(Src0::Rfegpo6),
            22 => Some(Src0::Rfegpo5),
            21 => Some(Src0::Rfegpo4),
            20 => Some(Src0::Rfegpo3),
            19 => Some(Src0::Rfegpo2),
            18 => Some(Src0::Rfegpo1),
            17 => Some(Src0::Rfegpo0),
            16 => Some(Src0::Mcegpo7),
            15 => Some(Src0::Mcegpo6),
            14 => Some(Src0::Mcegpo5),
            13 => Some(Src0::Mcegpo4),
            12 => Some(Src0::Mcegpo3),
            11 => Some(Src0::Mcegpo2),
            10 => Some(Src0::Mcegpo1),
            9 => Some(Src0::Mcegpo0),
            8 => Some(Src0::Pbegpo7),
            7 => Some(Src0::Pbegpo6),
            6 => Some(Src0::Pbegpo5),
            5 => Some(Src0::Pbegpo4),
            4 => Some(Src0::Pbegpo3),
            3 => Some(Src0::Pbegpo2),
            2 => Some(Src0::Pbegpo1),
            1 => Some(Src0::Pbegpo0),
            0 => Some(Src0::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 0"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src0::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src0::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src0::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src0::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src0::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src0::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src0::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src0::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src0::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src0::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src0::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src0::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src0::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src0::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src0::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src0::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src0::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src0::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src0::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src0::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src0::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src0::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src0::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src0::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src0::Pbegpo0
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src0::Dis
    }
}
#[doc = "Field `SRC0` writer - 4:0\\]
Select source of radio GPO line 0"]
pub type Src0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src0>;
impl<'a, REG> Src0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 0"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbegpo0)
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Dis)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "12:8\\]
Select source of radio GPO line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src1 {
    #[doc = "25: Select RFCTRC GPO line 1"]
    Rfctrc = 25,
    #[doc = "24: Select RFE GPO line 7"]
    Rfegpo7 = 24,
    #[doc = "23: Select RFE GPO line 6"]
    Rfegpo6 = 23,
    #[doc = "22: Select RFE GPO line 5"]
    Rfegpo5 = 22,
    #[doc = "21: Select RFE GPO line 4"]
    Rfegpo4 = 21,
    #[doc = "20: Select RFE GPO line 3"]
    Rfegpo3 = 20,
    #[doc = "19: Select RFE GPO line 2"]
    Rfegpo2 = 19,
    #[doc = "18: Select RFE GPO line 1"]
    Rfegpo1 = 18,
    #[doc = "17: Select RFE GPO line 0"]
    Rfegpo0 = 17,
    #[doc = "16: Select MCE GPO line 7"]
    Mcegpo7 = 16,
    #[doc = "15: Select MCE GPO line 6"]
    Mcegpo6 = 15,
    #[doc = "14: Select MCE GPO line 5"]
    Mcegpo5 = 14,
    #[doc = "13: Select MCE GPO line 4"]
    Mcegpo4 = 13,
    #[doc = "12: Select MCE GPO line 3"]
    Mcegpo3 = 12,
    #[doc = "11: Select MCE GPO line 2"]
    Mcegpo2 = 11,
    #[doc = "10: Select MCE GPO line 1"]
    Mcegpo1 = 10,
    #[doc = "9: Select MCE GPO line 0"]
    Mcegpo0 = 9,
    #[doc = "8: Select PBE GPO line 7"]
    Pbegpo7 = 8,
    #[doc = "7: Select PBE GPO line 6"]
    Pbegpo6 = 7,
    #[doc = "6: Select PBE GPO line 5"]
    Pbegpo5 = 6,
    #[doc = "5: Select PBE GPO line 4"]
    Pbegpo4 = 5,
    #[doc = "4: Select PBE GPO line 3"]
    Pbegpo3 = 4,
    #[doc = "3: Select PBE GPO line 2"]
    Pbegpo2 = 3,
    #[doc = "2: Select PBE GPO line 1"]
    Pbegpo1 = 2,
    #[doc = "1: Select PBE GPO line 0"]
    Pbegpo0 = 1,
    #[doc = "0: Output not enabled"]
    Dis = 0,
}
impl From<Src1> for u8 {
    #[inline(always)]
    fn from(variant: Src1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src1 {
    type Ux = u8;
}
impl crate::IsEnum for Src1 {}
#[doc = "Field `SRC1` reader - 12:8\\]
Select source of radio GPO line 1"]
pub type Src1R = crate::FieldReader<Src1>;
impl Src1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src1> {
        match self.bits {
            25 => Some(Src1::Rfctrc),
            24 => Some(Src1::Rfegpo7),
            23 => Some(Src1::Rfegpo6),
            22 => Some(Src1::Rfegpo5),
            21 => Some(Src1::Rfegpo4),
            20 => Some(Src1::Rfegpo3),
            19 => Some(Src1::Rfegpo2),
            18 => Some(Src1::Rfegpo1),
            17 => Some(Src1::Rfegpo0),
            16 => Some(Src1::Mcegpo7),
            15 => Some(Src1::Mcegpo6),
            14 => Some(Src1::Mcegpo5),
            13 => Some(Src1::Mcegpo4),
            12 => Some(Src1::Mcegpo3),
            11 => Some(Src1::Mcegpo2),
            10 => Some(Src1::Mcegpo1),
            9 => Some(Src1::Mcegpo0),
            8 => Some(Src1::Pbegpo7),
            7 => Some(Src1::Pbegpo6),
            6 => Some(Src1::Pbegpo5),
            5 => Some(Src1::Pbegpo4),
            4 => Some(Src1::Pbegpo3),
            3 => Some(Src1::Pbegpo2),
            2 => Some(Src1::Pbegpo1),
            1 => Some(Src1::Pbegpo0),
            0 => Some(Src1::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 1"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src1::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src1::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src1::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src1::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src1::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src1::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src1::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src1::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src1::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src1::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src1::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src1::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src1::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src1::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src1::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src1::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src1::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src1::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src1::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src1::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src1::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src1::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src1::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src1::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src1::Pbegpo0
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src1::Dis
    }
}
#[doc = "Field `SRC1` writer - 12:8\\]
Select source of radio GPO line 1"]
pub type Src1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src1>;
impl<'a, REG> Src1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 1"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbegpo0)
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Dis)
    }
}
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "20:16\\]
Select source of radio GPO line 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src2 {
    #[doc = "25: Select RFCTRC GPO line 2"]
    Rfctrc = 25,
    #[doc = "24: Select RFE GPO line 7"]
    Rfegpo7 = 24,
    #[doc = "23: Select RFE GPO line 6"]
    Rfegpo6 = 23,
    #[doc = "22: Select RFE GPO line 5"]
    Rfegpo5 = 22,
    #[doc = "21: Select RFE GPO line 4"]
    Rfegpo4 = 21,
    #[doc = "20: Select RFE GPO line 3"]
    Rfegpo3 = 20,
    #[doc = "19: Select RFE GPO line 2"]
    Rfegpo2 = 19,
    #[doc = "18: Select RFE GPO line 1"]
    Rfegpo1 = 18,
    #[doc = "17: Select RFE GPO line 0"]
    Rfegpo0 = 17,
    #[doc = "16: Select MCE GPO line 7"]
    Mcegpo7 = 16,
    #[doc = "15: Select MCE GPO line 6"]
    Mcegpo6 = 15,
    #[doc = "14: Select MCE GPO line 5"]
    Mcegpo5 = 14,
    #[doc = "13: Select MCE GPO line 4"]
    Mcegpo4 = 13,
    #[doc = "12: Select MCE GPO line 3"]
    Mcegpo3 = 12,
    #[doc = "11: Select MCE GPO line 2"]
    Mcegpo2 = 11,
    #[doc = "10: Select MCE GPO line 1"]
    Mcegpo1 = 10,
    #[doc = "9: Select MCE GPO line 0"]
    Mcegpo0 = 9,
    #[doc = "8: Select PBE GPO line 7"]
    Pbegpo7 = 8,
    #[doc = "7: Select PBE GPO line 6"]
    Pbegpo6 = 7,
    #[doc = "6: Select PBE GPO line 5"]
    Pbegpo5 = 6,
    #[doc = "5: Select PBE GPO line 4"]
    Pbegpo4 = 5,
    #[doc = "4: Select PBE GPO line 3"]
    Pbegpo3 = 4,
    #[doc = "3: Select PBE GPO line 2"]
    Pbegpo2 = 3,
    #[doc = "2: Select PBE GPO line 1"]
    Pbegpo1 = 2,
    #[doc = "1: Select PBE GPO line 0"]
    Pbegpo0 = 1,
    #[doc = "0: Output not enabled"]
    Dis = 0,
}
impl From<Src2> for u8 {
    #[inline(always)]
    fn from(variant: Src2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src2 {
    type Ux = u8;
}
impl crate::IsEnum for Src2 {}
#[doc = "Field `SRC2` reader - 20:16\\]
Select source of radio GPO line 2"]
pub type Src2R = crate::FieldReader<Src2>;
impl Src2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src2> {
        match self.bits {
            25 => Some(Src2::Rfctrc),
            24 => Some(Src2::Rfegpo7),
            23 => Some(Src2::Rfegpo6),
            22 => Some(Src2::Rfegpo5),
            21 => Some(Src2::Rfegpo4),
            20 => Some(Src2::Rfegpo3),
            19 => Some(Src2::Rfegpo2),
            18 => Some(Src2::Rfegpo1),
            17 => Some(Src2::Rfegpo0),
            16 => Some(Src2::Mcegpo7),
            15 => Some(Src2::Mcegpo6),
            14 => Some(Src2::Mcegpo5),
            13 => Some(Src2::Mcegpo4),
            12 => Some(Src2::Mcegpo3),
            11 => Some(Src2::Mcegpo2),
            10 => Some(Src2::Mcegpo1),
            9 => Some(Src2::Mcegpo0),
            8 => Some(Src2::Pbegpo7),
            7 => Some(Src2::Pbegpo6),
            6 => Some(Src2::Pbegpo5),
            5 => Some(Src2::Pbegpo4),
            4 => Some(Src2::Pbegpo3),
            3 => Some(Src2::Pbegpo2),
            2 => Some(Src2::Pbegpo1),
            1 => Some(Src2::Pbegpo0),
            0 => Some(Src2::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 2"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src2::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src2::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src2::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src2::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src2::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src2::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src2::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src2::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src2::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src2::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src2::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src2::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src2::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src2::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src2::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src2::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src2::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src2::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src2::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src2::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src2::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src2::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src2::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src2::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src2::Pbegpo0
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src2::Dis
    }
}
#[doc = "Field `SRC2` writer - 20:16\\]
Select source of radio GPO line 2"]
pub type Src2W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src2>;
impl<'a, REG> Src2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 2"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbegpo0)
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Dis)
    }
}
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "28:24\\]
Select source of radio GPO line 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src3 {
    #[doc = "25: Select RFCTRC GPO line 3"]
    Rfctrc = 25,
    #[doc = "24: Select RFE GPO line 7"]
    Rfegpo7 = 24,
    #[doc = "23: Select RFE GPO line 6"]
    Rfegpo6 = 23,
    #[doc = "22: Select RFE GPO line 5"]
    Rfegpo5 = 22,
    #[doc = "21: Select RFE GPO line 4"]
    Rfegpo4 = 21,
    #[doc = "20: Select RFE GPO line 3"]
    Rfegpo3 = 20,
    #[doc = "19: Select RFE GPO line 2"]
    Rfegpo2 = 19,
    #[doc = "18: Select RFE GPO line 1"]
    Rfegpo1 = 18,
    #[doc = "17: Select RFE GPO line 0"]
    Rfegpo0 = 17,
    #[doc = "16: Select MCE GPO line 7"]
    Mcegpo7 = 16,
    #[doc = "15: Select MCE GPO line 6"]
    Mcegpo6 = 15,
    #[doc = "14: Select MCE GPO line 5"]
    Mcegpo5 = 14,
    #[doc = "13: Select MCE GPO line 4"]
    Mcegpo4 = 13,
    #[doc = "12: Select MCE GPO line 3"]
    Mcegpo3 = 12,
    #[doc = "11: Select MCE GPO line 2"]
    Mcegpo2 = 11,
    #[doc = "10: Select MCE GPO line 1"]
    Mcegpo1 = 10,
    #[doc = "9: Select MCE GPO line 0"]
    Mcegpo0 = 9,
    #[doc = "8: Select PBE GPO line 7"]
    Pbegpo7 = 8,
    #[doc = "7: Select PBE GPO line 6"]
    Pbegpo6 = 7,
    #[doc = "6: Select PBE GPO line 5"]
    Pbegpo5 = 6,
    #[doc = "5: Select PBE GPO line 4"]
    Pbegpo4 = 5,
    #[doc = "4: Select PBE GPO line 3"]
    Pbegpo3 = 4,
    #[doc = "3: Select PBE GPO line 2"]
    Pbegpo2 = 3,
    #[doc = "2: Select PBE GPO line 1"]
    Pbegpo1 = 2,
    #[doc = "1: Select PBE GPO line 0"]
    Pbegpo0 = 1,
    #[doc = "0: Output not enabled"]
    Dis = 0,
}
impl From<Src3> for u8 {
    #[inline(always)]
    fn from(variant: Src3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src3 {
    type Ux = u8;
}
impl crate::IsEnum for Src3 {}
#[doc = "Field `SRC3` reader - 28:24\\]
Select source of radio GPO line 3"]
pub type Src3R = crate::FieldReader<Src3>;
impl Src3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src3> {
        match self.bits {
            25 => Some(Src3::Rfctrc),
            24 => Some(Src3::Rfegpo7),
            23 => Some(Src3::Rfegpo6),
            22 => Some(Src3::Rfegpo5),
            21 => Some(Src3::Rfegpo4),
            20 => Some(Src3::Rfegpo3),
            19 => Some(Src3::Rfegpo2),
            18 => Some(Src3::Rfegpo1),
            17 => Some(Src3::Rfegpo0),
            16 => Some(Src3::Mcegpo7),
            15 => Some(Src3::Mcegpo6),
            14 => Some(Src3::Mcegpo5),
            13 => Some(Src3::Mcegpo4),
            12 => Some(Src3::Mcegpo3),
            11 => Some(Src3::Mcegpo2),
            10 => Some(Src3::Mcegpo1),
            9 => Some(Src3::Mcegpo0),
            8 => Some(Src3::Pbegpo7),
            7 => Some(Src3::Pbegpo6),
            6 => Some(Src3::Pbegpo5),
            5 => Some(Src3::Pbegpo4),
            4 => Some(Src3::Pbegpo3),
            3 => Some(Src3::Pbegpo2),
            2 => Some(Src3::Pbegpo1),
            1 => Some(Src3::Pbegpo0),
            0 => Some(Src3::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 3"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src3::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src3::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src3::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src3::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src3::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src3::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src3::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src3::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src3::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src3::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src3::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src3::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src3::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src3::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src3::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src3::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src3::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src3::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src3::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src3::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src3::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src3::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src3::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src3::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src3::Pbegpo0
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src3::Dis
    }
}
#[doc = "Field `SRC3` writer - 28:24\\]
Select source of radio GPO line 3"]
pub type Src3W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src3>;
impl<'a, REG> Src3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 3"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Pbegpo0)
    }
    #[doc = "Output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src3::Dis)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Select source of radio GPO line 0"]
    #[inline(always)]
    pub fn src0(&self) -> Src0R {
        Src0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select source of radio GPO line 1"]
    #[inline(always)]
    pub fn src1(&self) -> Src1R {
        Src1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Select source of radio GPO line 2"]
    #[inline(always)]
    pub fn src2(&self) -> Src2R {
        Src2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Select source of radio GPO line 3"]
    #[inline(always)]
    pub fn src3(&self) -> Src3R {
        Src3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Select source of radio GPO line 0"]
    #[inline(always)]
    #[must_use]
    pub fn src0(&mut self) -> Src0W<Gposel0Spec> {
        Src0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select source of radio GPO line 1"]
    #[inline(always)]
    #[must_use]
    pub fn src1(&mut self) -> Src1W<Gposel0Spec> {
        Src1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Select source of radio GPO line 2"]
    #[inline(always)]
    #[must_use]
    pub fn src2(&mut self) -> Src2W<Gposel0Spec> {
        Src2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Select source of radio GPO line 3"]
    #[inline(always)]
    #[must_use]
    pub fn src3(&mut self) -> Src3W<Gposel0Spec> {
        Src3W::new(self, 24)
    }
}
#[doc = "Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gposel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gposel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gposel0Spec;
impl crate::RegisterSpec for Gposel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gposel0::R`](R) reader structure"]
impl crate::Readable for Gposel0Spec {}
#[doc = "`write(|w| ..)` method takes [`gposel0::W`](W) writer structure"]
impl crate::Writable for Gposel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPOSEL0 to value 0"]
impl crate::Resettable for Gposel0Spec {
    const RESET_VALUE: u32 = 0;
}
