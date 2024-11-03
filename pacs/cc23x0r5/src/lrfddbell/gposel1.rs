#[doc = "Register `GPOSEL1` reader"]
pub type R = crate::R<Gposel1Spec>;
#[doc = "Register `GPOSEL1` writer"]
pub type W = crate::W<Gposel1Spec>;
#[doc = "4:0\\]
Select source of radio GPO line 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src4 {
    #[doc = "25: Select RFCTRC GPO line 4"]
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
    #[doc = "0: No output not enabled"]
    Dis = 0,
}
impl From<Src4> for u8 {
    #[inline(always)]
    fn from(variant: Src4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src4 {
    type Ux = u8;
}
impl crate::IsEnum for Src4 {}
#[doc = "Field `SRC4` reader - 4:0\\]
Select source of radio GPO line 4"]
pub type Src4R = crate::FieldReader<Src4>;
impl Src4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src4> {
        match self.bits {
            25 => Some(Src4::Rfctrc),
            24 => Some(Src4::Rfegpo7),
            23 => Some(Src4::Rfegpo6),
            22 => Some(Src4::Rfegpo5),
            21 => Some(Src4::Rfegpo4),
            20 => Some(Src4::Rfegpo3),
            19 => Some(Src4::Rfegpo2),
            18 => Some(Src4::Rfegpo1),
            17 => Some(Src4::Rfegpo0),
            16 => Some(Src4::Mcegpo7),
            15 => Some(Src4::Mcegpo6),
            14 => Some(Src4::Mcegpo5),
            13 => Some(Src4::Mcegpo4),
            12 => Some(Src4::Mcegpo3),
            11 => Some(Src4::Mcegpo2),
            10 => Some(Src4::Mcegpo1),
            9 => Some(Src4::Mcegpo0),
            8 => Some(Src4::Pbegpo7),
            7 => Some(Src4::Pbegpo6),
            6 => Some(Src4::Pbegpo5),
            5 => Some(Src4::Pbegpo4),
            4 => Some(Src4::Pbegpo3),
            3 => Some(Src4::Pbegpo2),
            2 => Some(Src4::Pbegpo1),
            1 => Some(Src4::Pbegpo0),
            0 => Some(Src4::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 4"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src4::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src4::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src4::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src4::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src4::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src4::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src4::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src4::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src4::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src4::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src4::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src4::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src4::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src4::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src4::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src4::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src4::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src4::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src4::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src4::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src4::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src4::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src4::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src4::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src4::Pbegpo0
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src4::Dis
    }
}
#[doc = "Field `SRC4` writer - 4:0\\]
Select source of radio GPO line 4"]
pub type Src4W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src4>;
impl<'a, REG> Src4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 4"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Pbegpo0)
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src4::Dis)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "12:8\\]
Select source of radio GPO line 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src5 {
    #[doc = "25: Select RFCTRC GPO line 5"]
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
    #[doc = "0: No output not enabled"]
    Dis = 0,
}
impl From<Src5> for u8 {
    #[inline(always)]
    fn from(variant: Src5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src5 {
    type Ux = u8;
}
impl crate::IsEnum for Src5 {}
#[doc = "Field `SRC5` reader - 12:8\\]
Select source of radio GPO line 5"]
pub type Src5R = crate::FieldReader<Src5>;
impl Src5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src5> {
        match self.bits {
            25 => Some(Src5::Rfctrc),
            24 => Some(Src5::Rfegpo7),
            23 => Some(Src5::Rfegpo6),
            22 => Some(Src5::Rfegpo5),
            21 => Some(Src5::Rfegpo4),
            20 => Some(Src5::Rfegpo3),
            19 => Some(Src5::Rfegpo2),
            18 => Some(Src5::Rfegpo1),
            17 => Some(Src5::Rfegpo0),
            16 => Some(Src5::Mcegpo7),
            15 => Some(Src5::Mcegpo6),
            14 => Some(Src5::Mcegpo5),
            13 => Some(Src5::Mcegpo4),
            12 => Some(Src5::Mcegpo3),
            11 => Some(Src5::Mcegpo2),
            10 => Some(Src5::Mcegpo1),
            9 => Some(Src5::Mcegpo0),
            8 => Some(Src5::Pbegpo7),
            7 => Some(Src5::Pbegpo6),
            6 => Some(Src5::Pbegpo5),
            5 => Some(Src5::Pbegpo4),
            4 => Some(Src5::Pbegpo3),
            3 => Some(Src5::Pbegpo2),
            2 => Some(Src5::Pbegpo1),
            1 => Some(Src5::Pbegpo0),
            0 => Some(Src5::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 5"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src5::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src5::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src5::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src5::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src5::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src5::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src5::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src5::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src5::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src5::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src5::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src5::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src5::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src5::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src5::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src5::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src5::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src5::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src5::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src5::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src5::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src5::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src5::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src5::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src5::Pbegpo0
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src5::Dis
    }
}
#[doc = "Field `SRC5` writer - 12:8\\]
Select source of radio GPO line 5"]
pub type Src5W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src5>;
impl<'a, REG> Src5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 5"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Pbegpo0)
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src5::Dis)
    }
}
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `RESERVED13` writer - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "20:16\\]
Select source of radio GPO line 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src6 {
    #[doc = "25: Select RFCTRC GPO line 6"]
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
    #[doc = "19: Selevt RFE GPO line 2"]
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
    #[doc = "0: No output not enabled"]
    Dis = 0,
}
impl From<Src6> for u8 {
    #[inline(always)]
    fn from(variant: Src6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src6 {
    type Ux = u8;
}
impl crate::IsEnum for Src6 {}
#[doc = "Field `SRC6` reader - 20:16\\]
Select source of radio GPO line 6"]
pub type Src6R = crate::FieldReader<Src6>;
impl Src6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src6> {
        match self.bits {
            25 => Some(Src6::Rfctrc),
            24 => Some(Src6::Rfegpo7),
            23 => Some(Src6::Rfegpo6),
            22 => Some(Src6::Rfegpo5),
            21 => Some(Src6::Rfegpo4),
            20 => Some(Src6::Rfegpo3),
            19 => Some(Src6::Rfegpo2),
            18 => Some(Src6::Rfegpo1),
            17 => Some(Src6::Rfegpo0),
            16 => Some(Src6::Mcegpo7),
            15 => Some(Src6::Mcegpo6),
            14 => Some(Src6::Mcegpo5),
            13 => Some(Src6::Mcegpo4),
            12 => Some(Src6::Mcegpo3),
            11 => Some(Src6::Mcegpo2),
            10 => Some(Src6::Mcegpo1),
            9 => Some(Src6::Mcegpo0),
            8 => Some(Src6::Pbegpo7),
            7 => Some(Src6::Pbegpo6),
            6 => Some(Src6::Pbegpo5),
            5 => Some(Src6::Pbegpo4),
            4 => Some(Src6::Pbegpo3),
            3 => Some(Src6::Pbegpo2),
            2 => Some(Src6::Pbegpo1),
            1 => Some(Src6::Pbegpo0),
            0 => Some(Src6::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 6"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src6::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src6::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src6::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src6::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src6::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src6::Rfegpo3
    }
    #[doc = "Selevt RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src6::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src6::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src6::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src6::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src6::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src6::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src6::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src6::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src6::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src6::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src6::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src6::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src6::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src6::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src6::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src6::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src6::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src6::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src6::Pbegpo0
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src6::Dis
    }
}
#[doc = "Field `SRC6` writer - 20:16\\]
Select source of radio GPO line 6"]
pub type Src6W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src6>;
impl<'a, REG> Src6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 6"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo3)
    }
    #[doc = "Selevt RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Pbegpo0)
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src6::Dis)
    }
}
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "28:24\\]
Select source of radio GPO line 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src7 {
    #[doc = "25: Select RFCTRC GPO line 7"]
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
    #[doc = "0: No output not enabled"]
    Dis = 0,
}
impl From<Src7> for u8 {
    #[inline(always)]
    fn from(variant: Src7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src7 {
    type Ux = u8;
}
impl crate::IsEnum for Src7 {}
#[doc = "Field `SRC7` reader - 28:24\\]
Select source of radio GPO line 7"]
pub type Src7R = crate::FieldReader<Src7>;
impl Src7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src7> {
        match self.bits {
            25 => Some(Src7::Rfctrc),
            24 => Some(Src7::Rfegpo7),
            23 => Some(Src7::Rfegpo6),
            22 => Some(Src7::Rfegpo5),
            21 => Some(Src7::Rfegpo4),
            20 => Some(Src7::Rfegpo3),
            19 => Some(Src7::Rfegpo2),
            18 => Some(Src7::Rfegpo1),
            17 => Some(Src7::Rfegpo0),
            16 => Some(Src7::Mcegpo7),
            15 => Some(Src7::Mcegpo6),
            14 => Some(Src7::Mcegpo5),
            13 => Some(Src7::Mcegpo4),
            12 => Some(Src7::Mcegpo3),
            11 => Some(Src7::Mcegpo2),
            10 => Some(Src7::Mcegpo1),
            9 => Some(Src7::Mcegpo0),
            8 => Some(Src7::Pbegpo7),
            7 => Some(Src7::Pbegpo6),
            6 => Some(Src7::Pbegpo5),
            5 => Some(Src7::Pbegpo4),
            4 => Some(Src7::Pbegpo3),
            3 => Some(Src7::Pbegpo2),
            2 => Some(Src7::Pbegpo1),
            1 => Some(Src7::Pbegpo0),
            0 => Some(Src7::Dis),
            _ => None,
        }
    }
    #[doc = "Select RFCTRC GPO line 7"]
    #[inline(always)]
    pub fn is_rfctrc(&self) -> bool {
        *self == Src7::Rfctrc
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn is_rfegpo7(&self) -> bool {
        *self == Src7::Rfegpo7
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn is_rfegpo6(&self) -> bool {
        *self == Src7::Rfegpo6
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn is_rfegpo5(&self) -> bool {
        *self == Src7::Rfegpo5
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn is_rfegpo4(&self) -> bool {
        *self == Src7::Rfegpo4
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn is_rfegpo3(&self) -> bool {
        *self == Src7::Rfegpo3
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn is_rfegpo2(&self) -> bool {
        *self == Src7::Rfegpo2
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn is_rfegpo1(&self) -> bool {
        *self == Src7::Rfegpo1
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn is_rfegpo0(&self) -> bool {
        *self == Src7::Rfegpo0
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn is_mcegpo7(&self) -> bool {
        *self == Src7::Mcegpo7
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn is_mcegpo6(&self) -> bool {
        *self == Src7::Mcegpo6
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn is_mcegpo5(&self) -> bool {
        *self == Src7::Mcegpo5
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn is_mcegpo4(&self) -> bool {
        *self == Src7::Mcegpo4
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn is_mcegpo3(&self) -> bool {
        *self == Src7::Mcegpo3
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn is_mcegpo2(&self) -> bool {
        *self == Src7::Mcegpo2
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn is_mcegpo1(&self) -> bool {
        *self == Src7::Mcegpo1
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn is_mcegpo0(&self) -> bool {
        *self == Src7::Mcegpo0
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn is_pbegpo7(&self) -> bool {
        *self == Src7::Pbegpo7
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn is_pbegpo6(&self) -> bool {
        *self == Src7::Pbegpo6
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn is_pbegpo5(&self) -> bool {
        *self == Src7::Pbegpo5
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn is_pbegpo4(&self) -> bool {
        *self == Src7::Pbegpo4
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn is_pbegpo3(&self) -> bool {
        *self == Src7::Pbegpo3
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn is_pbegpo2(&self) -> bool {
        *self == Src7::Pbegpo2
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn is_pbegpo1(&self) -> bool {
        *self == Src7::Pbegpo1
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn is_pbegpo0(&self) -> bool {
        *self == Src7::Pbegpo0
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src7::Dis
    }
}
#[doc = "Field `SRC7` writer - 28:24\\]
Select source of radio GPO line 7"]
pub type Src7W<'a, REG> = crate::FieldWriter<'a, REG, 5, Src7>;
impl<'a, REG> Src7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select RFCTRC GPO line 7"]
    #[inline(always)]
    pub fn rfctrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfctrc)
    }
    #[doc = "Select RFE GPO line 7"]
    #[inline(always)]
    pub fn rfegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo7)
    }
    #[doc = "Select RFE GPO line 6"]
    #[inline(always)]
    pub fn rfegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo6)
    }
    #[doc = "Select RFE GPO line 5"]
    #[inline(always)]
    pub fn rfegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo5)
    }
    #[doc = "Select RFE GPO line 4"]
    #[inline(always)]
    pub fn rfegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo4)
    }
    #[doc = "Select RFE GPO line 3"]
    #[inline(always)]
    pub fn rfegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo3)
    }
    #[doc = "Select RFE GPO line 2"]
    #[inline(always)]
    pub fn rfegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo2)
    }
    #[doc = "Select RFE GPO line 1"]
    #[inline(always)]
    pub fn rfegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo1)
    }
    #[doc = "Select RFE GPO line 0"]
    #[inline(always)]
    pub fn rfegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Rfegpo0)
    }
    #[doc = "Select MCE GPO line 7"]
    #[inline(always)]
    pub fn mcegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo7)
    }
    #[doc = "Select MCE GPO line 6"]
    #[inline(always)]
    pub fn mcegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo6)
    }
    #[doc = "Select MCE GPO line 5"]
    #[inline(always)]
    pub fn mcegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo5)
    }
    #[doc = "Select MCE GPO line 4"]
    #[inline(always)]
    pub fn mcegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo4)
    }
    #[doc = "Select MCE GPO line 3"]
    #[inline(always)]
    pub fn mcegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo3)
    }
    #[doc = "Select MCE GPO line 2"]
    #[inline(always)]
    pub fn mcegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo2)
    }
    #[doc = "Select MCE GPO line 1"]
    #[inline(always)]
    pub fn mcegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo1)
    }
    #[doc = "Select MCE GPO line 0"]
    #[inline(always)]
    pub fn mcegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Mcegpo0)
    }
    #[doc = "Select PBE GPO line 7"]
    #[inline(always)]
    pub fn pbegpo7(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo7)
    }
    #[doc = "Select PBE GPO line 6"]
    #[inline(always)]
    pub fn pbegpo6(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo6)
    }
    #[doc = "Select PBE GPO line 5"]
    #[inline(always)]
    pub fn pbegpo5(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo5)
    }
    #[doc = "Select PBE GPO line 4"]
    #[inline(always)]
    pub fn pbegpo4(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo4)
    }
    #[doc = "Select PBE GPO line 3"]
    #[inline(always)]
    pub fn pbegpo3(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo3)
    }
    #[doc = "Select PBE GPO line 2"]
    #[inline(always)]
    pub fn pbegpo2(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo2)
    }
    #[doc = "Select PBE GPO line 1"]
    #[inline(always)]
    pub fn pbegpo1(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo1)
    }
    #[doc = "Select PBE GPO line 0"]
    #[inline(always)]
    pub fn pbegpo0(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Pbegpo0)
    }
    #[doc = "No output not enabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src7::Dis)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Select source of radio GPO line 4"]
    #[inline(always)]
    pub fn src4(&self) -> Src4R {
        Src4R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select source of radio GPO line 5"]
    #[inline(always)]
    pub fn src5(&self) -> Src5R {
        Src5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Select source of radio GPO line 6"]
    #[inline(always)]
    pub fn src6(&self) -> Src6R {
        Src6R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Select source of radio GPO line 7"]
    #[inline(always)]
    pub fn src7(&self) -> Src7R {
        Src7R::new(((self.bits >> 24) & 0x1f) as u8)
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
Select source of radio GPO line 4"]
    #[inline(always)]
    #[must_use]
    pub fn src4(&mut self) -> Src4W<Gposel1Spec> {
        Src4W::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Gposel1Spec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Select source of radio GPO line 5"]
    #[inline(always)]
    #[must_use]
    pub fn src5(&mut self) -> Src5W<Gposel1Spec> {
        Src5W::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<Gposel1Spec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Select source of radio GPO line 6"]
    #[inline(always)]
    #[must_use]
    pub fn src6(&mut self) -> Src6W<Gposel1Spec> {
        Src6W::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<Gposel1Spec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Select source of radio GPO line 7"]
    #[inline(always)]
    #[must_use]
    pub fn src7(&mut self) -> Src7W<Gposel1Spec> {
        Src7W::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<Gposel1Spec> {
        Reserved29W::new(self, 29)
    }
}
#[doc = "Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gposel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gposel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gposel1Spec;
impl crate::RegisterSpec for Gposel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gposel1::R`](R) reader structure"]
impl crate::Readable for Gposel1Spec {}
#[doc = "`write(|w| ..)` method takes [`gposel1::W`](W) writer structure"]
impl crate::Writable for Gposel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPOSEL1 to value 0"]
impl crate::Resettable for Gposel1Spec {
    const RESET_VALUE: u32 = 0;
}
