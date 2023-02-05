#[doc = "Register `SYSTIMOEV` reader"]
pub type R = crate::R<SystimoevSpec>;
#[doc = "Register `SYSTIMOEV` writer"]
pub type W = crate::W<SystimoevSpec>;
#[doc = "3:0\\]
Select source of systimer output event 0 (capture source)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src0 {
    #[doc = "12: PBE FW systimer capture event 2"]
    Pbesystim2 = 12,
    #[doc = "11: PBE FW systimer capture event 1"]
    Pbesystim1 = 11,
    #[doc = "10: PBE FW systimer capture event 0"]
    Pbesystim0 = 10,
    #[doc = "9: MDM HW event 2"]
    Mdmhw2 = 9,
    #[doc = "8: MDM HW event 1"]
    Mdmhw1 = 8,
    #[doc = "7: MDM HW event 0"]
    Mdmhw0 = 7,
    #[doc = "6: MCE FW systimer capture event 2"]
    Mcesystim2 = 6,
    #[doc = "5: MCE FW systimer capture event 1"]
    Mcesystim1 = 5,
    #[doc = "4: MCE FW systimer capture event 0"]
    Mcesystim0 = 4,
    #[doc = "3: RFE FW systimer capture event 2"]
    Rfesystim2 = 3,
    #[doc = "2: RFE FW systimer capture event 1"]
    Rfesystim1 = 2,
    #[doc = "1: RFE FW systimer capture event 0"]
    Rfesystim0 = 1,
    #[doc = "0: Output not enabled, always 0."]
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
#[doc = "Field `SRC0` reader - 3:0\\]
Select source of systimer output event 0 (capture source)"]
pub type Src0R = crate::FieldReader<Src0>;
impl Src0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src0> {
        match self.bits {
            12 => Some(Src0::Pbesystim2),
            11 => Some(Src0::Pbesystim1),
            10 => Some(Src0::Pbesystim0),
            9 => Some(Src0::Mdmhw2),
            8 => Some(Src0::Mdmhw1),
            7 => Some(Src0::Mdmhw0),
            6 => Some(Src0::Mcesystim2),
            5 => Some(Src0::Mcesystim1),
            4 => Some(Src0::Mcesystim0),
            3 => Some(Src0::Rfesystim2),
            2 => Some(Src0::Rfesystim1),
            1 => Some(Src0::Rfesystim0),
            0 => Some(Src0::Dis),
            _ => None,
        }
    }
    #[doc = "PBE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_pbesystim2(&self) -> bool {
        *self == Src0::Pbesystim2
    }
    #[doc = "PBE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_pbesystim1(&self) -> bool {
        *self == Src0::Pbesystim1
    }
    #[doc = "PBE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_pbesystim0(&self) -> bool {
        *self == Src0::Pbesystim0
    }
    #[doc = "MDM HW event 2"]
    #[inline(always)]
    pub fn is_mdmhw2(&self) -> bool {
        *self == Src0::Mdmhw2
    }
    #[doc = "MDM HW event 1"]
    #[inline(always)]
    pub fn is_mdmhw1(&self) -> bool {
        *self == Src0::Mdmhw1
    }
    #[doc = "MDM HW event 0"]
    #[inline(always)]
    pub fn is_mdmhw0(&self) -> bool {
        *self == Src0::Mdmhw0
    }
    #[doc = "MCE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_mcesystim2(&self) -> bool {
        *self == Src0::Mcesystim2
    }
    #[doc = "MCE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_mcesystim1(&self) -> bool {
        *self == Src0::Mcesystim1
    }
    #[doc = "MCE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_mcesystim0(&self) -> bool {
        *self == Src0::Mcesystim0
    }
    #[doc = "RFE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_rfesystim2(&self) -> bool {
        *self == Src0::Rfesystim2
    }
    #[doc = "RFE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_rfesystim1(&self) -> bool {
        *self == Src0::Rfesystim1
    }
    #[doc = "RFE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_rfesystim0(&self) -> bool {
        *self == Src0::Rfesystim0
    }
    #[doc = "Output not enabled, always 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src0::Dis
    }
}
#[doc = "Field `SRC0` writer - 3:0\\]
Select source of systimer output event 0 (capture source)"]
pub type Src0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Src0>;
impl<'a, REG> Src0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PBE FW systimer capture event 2"]
    #[inline(always)]
    pub fn pbesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbesystim2)
    }
    #[doc = "PBE FW systimer capture event 1"]
    #[inline(always)]
    pub fn pbesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbesystim1)
    }
    #[doc = "PBE FW systimer capture event 0"]
    #[inline(always)]
    pub fn pbesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Pbesystim0)
    }
    #[doc = "MDM HW event 2"]
    #[inline(always)]
    pub fn mdmhw2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mdmhw2)
    }
    #[doc = "MDM HW event 1"]
    #[inline(always)]
    pub fn mdmhw1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mdmhw1)
    }
    #[doc = "MDM HW event 0"]
    #[inline(always)]
    pub fn mdmhw0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mdmhw0)
    }
    #[doc = "MCE FW systimer capture event 2"]
    #[inline(always)]
    pub fn mcesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcesystim2)
    }
    #[doc = "MCE FW systimer capture event 1"]
    #[inline(always)]
    pub fn mcesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcesystim1)
    }
    #[doc = "MCE FW systimer capture event 0"]
    #[inline(always)]
    pub fn mcesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Mcesystim0)
    }
    #[doc = "RFE FW systimer capture event 2"]
    #[inline(always)]
    pub fn rfesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfesystim2)
    }
    #[doc = "RFE FW systimer capture event 1"]
    #[inline(always)]
    pub fn rfesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfesystim1)
    }
    #[doc = "RFE FW systimer capture event 0"]
    #[inline(always)]
    pub fn rfesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Rfesystim0)
    }
    #[doc = "Output not enabled, always 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src0::Dis)
    }
}
#[doc = "7:4\\]
Select source of systimer output event 1 (capture source)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src1 {
    #[doc = "12: PBE FW systimer capture event 2"]
    Pbesystim2 = 12,
    #[doc = "11: PBE FW systimer capture event 1"]
    Pbesystim1 = 11,
    #[doc = "10: PBE FW systimer capture event 0"]
    Pbesystim0 = 10,
    #[doc = "9: MDM HW event 2"]
    Mdmhw2 = 9,
    #[doc = "8: MDM HW event 1"]
    Mdmhw1 = 8,
    #[doc = "7: MDM HW event 0"]
    Mdmhw0 = 7,
    #[doc = "6: MCE FW systimer capture event 2"]
    Mcesystim2 = 6,
    #[doc = "5: MCE FW systimer capture event 1"]
    Mcesystim1 = 5,
    #[doc = "4: MCE FW systimer capture event 0"]
    Mcesystim0 = 4,
    #[doc = "3: RFE FW systimer capture event 2"]
    Rfesystim2 = 3,
    #[doc = "2: RFE FW systimer capture event 1"]
    Rfesystim1 = 2,
    #[doc = "1: RFE FW systimer capture event 0"]
    Rfesystim0 = 1,
    #[doc = "0: Output not enabled, always 0."]
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
#[doc = "Field `SRC1` reader - 7:4\\]
Select source of systimer output event 1 (capture source)"]
pub type Src1R = crate::FieldReader<Src1>;
impl Src1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src1> {
        match self.bits {
            12 => Some(Src1::Pbesystim2),
            11 => Some(Src1::Pbesystim1),
            10 => Some(Src1::Pbesystim0),
            9 => Some(Src1::Mdmhw2),
            8 => Some(Src1::Mdmhw1),
            7 => Some(Src1::Mdmhw0),
            6 => Some(Src1::Mcesystim2),
            5 => Some(Src1::Mcesystim1),
            4 => Some(Src1::Mcesystim0),
            3 => Some(Src1::Rfesystim2),
            2 => Some(Src1::Rfesystim1),
            1 => Some(Src1::Rfesystim0),
            0 => Some(Src1::Dis),
            _ => None,
        }
    }
    #[doc = "PBE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_pbesystim2(&self) -> bool {
        *self == Src1::Pbesystim2
    }
    #[doc = "PBE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_pbesystim1(&self) -> bool {
        *self == Src1::Pbesystim1
    }
    #[doc = "PBE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_pbesystim0(&self) -> bool {
        *self == Src1::Pbesystim0
    }
    #[doc = "MDM HW event 2"]
    #[inline(always)]
    pub fn is_mdmhw2(&self) -> bool {
        *self == Src1::Mdmhw2
    }
    #[doc = "MDM HW event 1"]
    #[inline(always)]
    pub fn is_mdmhw1(&self) -> bool {
        *self == Src1::Mdmhw1
    }
    #[doc = "MDM HW event 0"]
    #[inline(always)]
    pub fn is_mdmhw0(&self) -> bool {
        *self == Src1::Mdmhw0
    }
    #[doc = "MCE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_mcesystim2(&self) -> bool {
        *self == Src1::Mcesystim2
    }
    #[doc = "MCE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_mcesystim1(&self) -> bool {
        *self == Src1::Mcesystim1
    }
    #[doc = "MCE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_mcesystim0(&self) -> bool {
        *self == Src1::Mcesystim0
    }
    #[doc = "RFE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_rfesystim2(&self) -> bool {
        *self == Src1::Rfesystim2
    }
    #[doc = "RFE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_rfesystim1(&self) -> bool {
        *self == Src1::Rfesystim1
    }
    #[doc = "RFE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_rfesystim0(&self) -> bool {
        *self == Src1::Rfesystim0
    }
    #[doc = "Output not enabled, always 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src1::Dis
    }
}
#[doc = "Field `SRC1` writer - 7:4\\]
Select source of systimer output event 1 (capture source)"]
pub type Src1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Src1>;
impl<'a, REG> Src1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PBE FW systimer capture event 2"]
    #[inline(always)]
    pub fn pbesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbesystim2)
    }
    #[doc = "PBE FW systimer capture event 1"]
    #[inline(always)]
    pub fn pbesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbesystim1)
    }
    #[doc = "PBE FW systimer capture event 0"]
    #[inline(always)]
    pub fn pbesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Pbesystim0)
    }
    #[doc = "MDM HW event 2"]
    #[inline(always)]
    pub fn mdmhw2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mdmhw2)
    }
    #[doc = "MDM HW event 1"]
    #[inline(always)]
    pub fn mdmhw1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mdmhw1)
    }
    #[doc = "MDM HW event 0"]
    #[inline(always)]
    pub fn mdmhw0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mdmhw0)
    }
    #[doc = "MCE FW systimer capture event 2"]
    #[inline(always)]
    pub fn mcesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcesystim2)
    }
    #[doc = "MCE FW systimer capture event 1"]
    #[inline(always)]
    pub fn mcesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcesystim1)
    }
    #[doc = "MCE FW systimer capture event 0"]
    #[inline(always)]
    pub fn mcesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Mcesystim0)
    }
    #[doc = "RFE FW systimer capture event 2"]
    #[inline(always)]
    pub fn rfesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfesystim2)
    }
    #[doc = "RFE FW systimer capture event 1"]
    #[inline(always)]
    pub fn rfesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfesystim1)
    }
    #[doc = "RFE FW systimer capture event 0"]
    #[inline(always)]
    pub fn rfesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Rfesystim0)
    }
    #[doc = "Output not enabled, always 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src1::Dis)
    }
}
#[doc = "11:8\\]
Select source of systimer output event 2 (capture source)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src2 {
    #[doc = "12: PBE FW systimer capture event 2"]
    Pbesystim2 = 12,
    #[doc = "11: PBE FW systimer capture event 1"]
    Pbesystim1 = 11,
    #[doc = "10: PBE FW systimer capture event 0"]
    Pbesystim0 = 10,
    #[doc = "9: MDM HW event 2"]
    Mdmhw2 = 9,
    #[doc = "8: MDM HW event 1"]
    Mdmhw1 = 8,
    #[doc = "7: MDM HW event 0"]
    Mdmhw0 = 7,
    #[doc = "6: MCE FW systimer capture event 2"]
    Mcesystim2 = 6,
    #[doc = "5: MCE FW systimer capture event 1"]
    Mcesystim1 = 5,
    #[doc = "4: MCE FW systimer capture event 0"]
    Mcesystim0 = 4,
    #[doc = "3: RFE FW systimer capture event 2"]
    Rfesystim2 = 3,
    #[doc = "2: RFE FW systimer capture event 1"]
    Rfesystim1 = 2,
    #[doc = "1: RFE FW systimer capture event 0"]
    Rfesystim0 = 1,
    #[doc = "0: Output not enabled, always 0."]
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
#[doc = "Field `SRC2` reader - 11:8\\]
Select source of systimer output event 2 (capture source)"]
pub type Src2R = crate::FieldReader<Src2>;
impl Src2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src2> {
        match self.bits {
            12 => Some(Src2::Pbesystim2),
            11 => Some(Src2::Pbesystim1),
            10 => Some(Src2::Pbesystim0),
            9 => Some(Src2::Mdmhw2),
            8 => Some(Src2::Mdmhw1),
            7 => Some(Src2::Mdmhw0),
            6 => Some(Src2::Mcesystim2),
            5 => Some(Src2::Mcesystim1),
            4 => Some(Src2::Mcesystim0),
            3 => Some(Src2::Rfesystim2),
            2 => Some(Src2::Rfesystim1),
            1 => Some(Src2::Rfesystim0),
            0 => Some(Src2::Dis),
            _ => None,
        }
    }
    #[doc = "PBE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_pbesystim2(&self) -> bool {
        *self == Src2::Pbesystim2
    }
    #[doc = "PBE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_pbesystim1(&self) -> bool {
        *self == Src2::Pbesystim1
    }
    #[doc = "PBE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_pbesystim0(&self) -> bool {
        *self == Src2::Pbesystim0
    }
    #[doc = "MDM HW event 2"]
    #[inline(always)]
    pub fn is_mdmhw2(&self) -> bool {
        *self == Src2::Mdmhw2
    }
    #[doc = "MDM HW event 1"]
    #[inline(always)]
    pub fn is_mdmhw1(&self) -> bool {
        *self == Src2::Mdmhw1
    }
    #[doc = "MDM HW event 0"]
    #[inline(always)]
    pub fn is_mdmhw0(&self) -> bool {
        *self == Src2::Mdmhw0
    }
    #[doc = "MCE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_mcesystim2(&self) -> bool {
        *self == Src2::Mcesystim2
    }
    #[doc = "MCE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_mcesystim1(&self) -> bool {
        *self == Src2::Mcesystim1
    }
    #[doc = "MCE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_mcesystim0(&self) -> bool {
        *self == Src2::Mcesystim0
    }
    #[doc = "RFE FW systimer capture event 2"]
    #[inline(always)]
    pub fn is_rfesystim2(&self) -> bool {
        *self == Src2::Rfesystim2
    }
    #[doc = "RFE FW systimer capture event 1"]
    #[inline(always)]
    pub fn is_rfesystim1(&self) -> bool {
        *self == Src2::Rfesystim1
    }
    #[doc = "RFE FW systimer capture event 0"]
    #[inline(always)]
    pub fn is_rfesystim0(&self) -> bool {
        *self == Src2::Rfesystim0
    }
    #[doc = "Output not enabled, always 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src2::Dis
    }
}
#[doc = "Field `SRC2` writer - 11:8\\]
Select source of systimer output event 2 (capture source)"]
pub type Src2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Src2>;
impl<'a, REG> Src2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PBE FW systimer capture event 2"]
    #[inline(always)]
    pub fn pbesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbesystim2)
    }
    #[doc = "PBE FW systimer capture event 1"]
    #[inline(always)]
    pub fn pbesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbesystim1)
    }
    #[doc = "PBE FW systimer capture event 0"]
    #[inline(always)]
    pub fn pbesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Pbesystim0)
    }
    #[doc = "MDM HW event 2"]
    #[inline(always)]
    pub fn mdmhw2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mdmhw2)
    }
    #[doc = "MDM HW event 1"]
    #[inline(always)]
    pub fn mdmhw1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mdmhw1)
    }
    #[doc = "MDM HW event 0"]
    #[inline(always)]
    pub fn mdmhw0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mdmhw0)
    }
    #[doc = "MCE FW systimer capture event 2"]
    #[inline(always)]
    pub fn mcesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcesystim2)
    }
    #[doc = "MCE FW systimer capture event 1"]
    #[inline(always)]
    pub fn mcesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcesystim1)
    }
    #[doc = "MCE FW systimer capture event 0"]
    #[inline(always)]
    pub fn mcesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Mcesystim0)
    }
    #[doc = "RFE FW systimer capture event 2"]
    #[inline(always)]
    pub fn rfesystim2(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfesystim2)
    }
    #[doc = "RFE FW systimer capture event 1"]
    #[inline(always)]
    pub fn rfesystim1(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfesystim1)
    }
    #[doc = "RFE FW systimer capture event 0"]
    #[inline(always)]
    pub fn rfesystim0(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Rfesystim0)
    }
    #[doc = "Output not enabled, always 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src2::Dis)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Select source of systimer output event 0 (capture source)"]
    #[inline(always)]
    pub fn src0(&self) -> Src0R {
        Src0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Select source of systimer output event 1 (capture source)"]
    #[inline(always)]
    pub fn src1(&self) -> Src1R {
        Src1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Select source of systimer output event 2 (capture source)"]
    #[inline(always)]
    pub fn src2(&self) -> Src2R {
        Src2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Select source of systimer output event 0 (capture source)"]
    #[inline(always)]
    #[must_use]
    pub fn src0(&mut self) -> Src0W<SystimoevSpec> {
        Src0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Select source of systimer output event 1 (capture source)"]
    #[inline(always)]
    #[must_use]
    pub fn src1(&mut self) -> Src1W<SystimoevSpec> {
        Src1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Select source of systimer output event 2 (capture source)"]
    #[inline(always)]
    #[must_use]
    pub fn src2(&mut self) -> Src2W<SystimoevSpec> {
        Src2W::new(self, 8)
    }
}
#[doc = "Systimer Output Event Control Register. Controls routing of internal events to the three systimer output events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimoev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimoev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystimoevSpec;
impl crate::RegisterSpec for SystimoevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimoev::R`](R) reader structure"]
impl crate::Readable for SystimoevSpec {}
#[doc = "`write(|w| ..)` method takes [`systimoev::W`](W) writer structure"]
impl crate::Writable for SystimoevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTIMOEV to value 0"]
impl crate::Resettable for SystimoevSpec {
    const RESET_VALUE: u32 = 0;
}
