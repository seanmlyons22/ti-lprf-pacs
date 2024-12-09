#[doc = "Register `LPCMPCFG` reader"]
pub type R = crate::R<LpcmpcfgSpec>;
#[doc = "Register `LPCMPCFG` writer"]
pub type W = crate::W<LpcmpcfgSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    En = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Dis = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::En,
            false => En::Dis,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
}
#[doc = "3:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Div {
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Val4 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    Val3 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Val2 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Val0 = 0,
}
impl From<Div> for u8 {
    #[inline(always)]
    fn from(variant: Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Div {
    type Ux = u8;
}
impl crate::IsEnum for Div {}
#[doc = "Field `DIV` reader - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type DivR = crate::FieldReader<Div>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Div> {
        match self.bits {
            4 => Some(Div::Val4),
            3 => Some(Div::Val3),
            2 => Some(Div::Val2),
            1 => Some(Div::Val1),
            0 => Some(Div::Val0),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Div::Val4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Div::Val3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Div::Val2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Div::Val1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Div::Val0
    }
}
#[doc = "Field `DIV` writer - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Div>;
impl<'a, REG> DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Val4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Val3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Val2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Val1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Val0)
    }
}
#[doc = "4:4\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divpath {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Pside = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Nside = 0,
}
impl From<Divpath> for bool {
    #[inline(always)]
    fn from(variant: Divpath) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVPATH` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DivpathR = crate::BitReader<Divpath>;
impl DivpathR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divpath {
        match self.bits {
            true => Divpath::Pside,
            false => Divpath::Nside,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_pside(&self) -> bool {
        *self == Divpath::Pside
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_nside(&self) -> bool {
        *self == Divpath::Nside
    }
}
#[doc = "Field `DIVPATH` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type DivpathW<'a, REG> = crate::BitWriter<'a, REG, Divpath>;
impl<'a, REG> DivpathW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pside(self) -> &'a mut crate::W<REG> {
        self.variant(Divpath::Pside)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nside(self) -> &'a mut crate::W<REG> {
        self.variant(Divpath::Nside)
    }
}
#[doc = "7:5\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hyssel {
    #[doc = "7: Internal. Only to be used through TI provided API."]
    Val7 = 7,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    Val6 = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    Val5 = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Val4 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    Val3 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Val2 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Val0 = 0,
}
impl From<Hyssel> for u8 {
    #[inline(always)]
    fn from(variant: Hyssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hyssel {
    type Ux = u8;
}
impl crate::IsEnum for Hyssel {}
#[doc = "Field `HYSSEL` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type HysselR = crate::FieldReader<Hyssel>;
impl HysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyssel {
        match self.bits {
            7 => Hyssel::Val7,
            6 => Hyssel::Val6,
            5 => Hyssel::Val5,
            4 => Hyssel::Val4,
            3 => Hyssel::Val3,
            2 => Hyssel::Val2,
            1 => Hyssel::Val1,
            0 => Hyssel::Val0,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == Hyssel::Val7
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == Hyssel::Val6
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == Hyssel::Val5
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Hyssel::Val4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Hyssel::Val3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Hyssel::Val2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Hyssel::Val1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Hyssel::Val0
    }
}
#[doc = "Field `HYSSEL` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type HysselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hyssel, crate::Safe>;
impl<'a, REG> HysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val7(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val7)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val6(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val5(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Hyssel::Val0)
    }
}
#[doc = "11:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Vdda = 8,
    #[doc = "7: Internal. Only to be used through TI provided API."]
    VaAtestA1 = 7,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    VaAtestA0 = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    VrAtestA1 = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    VrAtestA0 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    VaPadA3 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    VaPadA2 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    VaPadA1 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
impl crate::IsEnum for Psel {}
#[doc = "Field `PSEL` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Psel> {
        match self.bits {
            8 => Some(Psel::Vdda),
            7 => Some(Psel::VaAtestA1),
            6 => Some(Psel::VaAtestA0),
            5 => Some(Psel::VrAtestA1),
            4 => Some(Psel::VrAtestA0),
            3 => Some(Psel::VaPadA3),
            2 => Some(Psel::VaPadA2),
            1 => Some(Psel::VaPadA1),
            0 => Some(Psel::Open),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == Psel::Vdda
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_atest_a1(&self) -> bool {
        *self == Psel::VaAtestA1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_atest_a0(&self) -> bool {
        *self == Psel::VaAtestA0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vr_atest_a1(&self) -> bool {
        *self == Psel::VrAtestA1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vr_atest_a0(&self) -> bool {
        *self == Psel::VrAtestA0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_pad_a3(&self) -> bool {
        *self == Psel::VaPadA3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_pad_a2(&self) -> bool {
        *self == Psel::VaPadA2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_pad_a1(&self) -> bool {
        *self == Psel::VaPadA1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Psel::Open
    }
}
#[doc = "Field `PSEL` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Psel>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Vdda)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_atest_a1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VaAtestA1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_atest_a0(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VaAtestA0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vr_atest_a1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VrAtestA1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vr_atest_a0(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VrAtestA0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_pad_a3(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VaPadA3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_pad_a2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VaPadA2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_pad_a1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VaPadA1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Open)
    }
}
#[doc = "14:12\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsel {
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Vddd = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    Vdda = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    VaPadA3 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    VaPadA2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Nsel> for u8 {
    #[inline(always)]
    fn from(variant: Nsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsel {
    type Ux = u8;
}
impl crate::IsEnum for Nsel {}
#[doc = "Field `NSEL` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type NselR = crate::FieldReader<Nsel>;
impl NselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nsel> {
        match self.bits {
            4 => Some(Nsel::Vddd),
            3 => Some(Nsel::Vdda),
            2 => Some(Nsel::VaPadA3),
            1 => Some(Nsel::VaPadA2),
            0 => Some(Nsel::Open),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vddd(&self) -> bool {
        *self == Nsel::Vddd
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == Nsel::Vdda
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_pad_a3(&self) -> bool {
        *self == Nsel::VaPadA3
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_pad_a2(&self) -> bool {
        *self == Nsel::VaPadA2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Nsel::Open
    }
}
#[doc = "Field `NSEL` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type NselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nsel>;
impl<'a, REG> NselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddd(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Vddd)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Vdda)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_pad_a3(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::VaPadA3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_pad_a2(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::VaPadA2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Nsel::Open)
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type Reserved15R = crate::BitReader;
#[doc = "16:16\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edgcfg {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Fall = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Rise = 0,
}
impl From<Edgcfg> for bool {
    #[inline(always)]
    fn from(variant: Edgcfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGCFG` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type EdgcfgR = crate::BitReader<Edgcfg>;
impl EdgcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edgcfg {
        match self.bits {
            true => Edgcfg::Fall,
            false => Edgcfg::Rise,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Edgcfg::Fall
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Edgcfg::Rise
    }
}
#[doc = "Field `EDGCFG` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type EdgcfgW<'a, REG> = crate::BitWriter<'a, REG, Edgcfg>;
impl<'a, REG> EdgcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Edgcfg::Fall)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Edgcfg::Rise)
    }
}
#[doc = "17:17\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evten {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    En = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Dis = 0,
}
impl From<Evten> for bool {
    #[inline(always)]
    fn from(variant: Evten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTEN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type EvtenR = crate::BitReader<Evten>;
impl EvtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evten {
        match self.bits {
            true => Evten::En,
            false => Evten::Dis,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Evten::En
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Evten::Dis
    }
}
#[doc = "Field `EVTEN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type EvtenW<'a, REG> = crate::BitWriter<'a, REG, Evten>;
impl<'a, REG> EvtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Evten::En)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Evten::Dis)
    }
}
#[doc = "18:18\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuensb {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    En = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Dis = 0,
}
impl From<Wuensb> for bool {
    #[inline(always)]
    fn from(variant: Wuensb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUENSB` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
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
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wuensb::En
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wuensb::Dis
    }
}
#[doc = "Field `WUENSB` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type WuensbW<'a, REG> = crate::BitWriter<'a, REG, Wuensb>;
impl<'a, REG> WuensbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Wuensb::En)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Wuensb::Dis)
    }
}
#[doc = "Field `RESERVED19` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved19R = crate::BitReader;
#[doc = "20:20\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cout {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    High = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Low = 0,
}
impl From<Cout> for bool {
    #[inline(always)]
    fn from(variant: Cout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUT` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CoutR = crate::BitReader<Cout>;
impl CoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cout {
        match self.bits {
            true => Cout::High,
            false => Cout::Low,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cout::High
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cout::Low
    }
}
#[doc = "21:21\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Couten {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    En = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Dis = 0,
}
impl From<Couten> for bool {
    #[inline(always)]
    fn from(variant: Couten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUTEN` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type CoutenR = crate::BitReader<Couten>;
impl CoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Couten {
        match self.bits {
            true => Couten::En,
            false => Couten::Dis,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Couten::En
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Couten::Dis
    }
}
#[doc = "Field `COUTEN` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type CoutenW<'a, REG> = crate::BitWriter<'a, REG, Couten>;
impl<'a, REG> CoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Couten::En)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Couten::Dis)
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type Reserved22R = crate::FieldReader;
#[doc = "24:24\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evtifg {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Set = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Clr = 0,
}
impl From<Evtifg> for bool {
    #[inline(always)]
    fn from(variant: Evtifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTIFG` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type EvtifgR = crate::BitReader<Evtifg>;
impl EvtifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evtifg {
        match self.bits {
            true => Evtifg::Set,
            false => Evtifg::Clr,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Evtifg::Set
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Evtifg::Clr
    }
}
#[doc = "Field `EVTIFG` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type EvtifgW<'a, REG> = crate::BitWriter<'a, REG, Evtifg>;
impl<'a, REG> EvtifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Evtifg::Set)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Evtifg::Clr)
    }
}
#[doc = "Field `RESERVED25` reader - 27:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
#[doc = "29:28\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atestmux {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    Ibiasout = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    CompVinNeg = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Compout = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Off = 0,
}
impl From<Atestmux> for u8 {
    #[inline(always)]
    fn from(variant: Atestmux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atestmux {
    type Ux = u8;
}
impl crate::IsEnum for Atestmux {}
#[doc = "Field `ATESTMUX` reader - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type AtestmuxR = crate::FieldReader<Atestmux>;
impl AtestmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atestmux {
        match self.bits {
            3 => Atestmux::Ibiasout,
            2 => Atestmux::CompVinNeg,
            1 => Atestmux::Compout,
            0 => Atestmux::Off,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ibiasout(&self) -> bool {
        *self == Atestmux::Ibiasout
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_comp_vin_neg(&self) -> bool {
        *self == Atestmux::CompVinNeg
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_compout(&self) -> bool {
        *self == Atestmux::Compout
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Atestmux::Off
    }
}
#[doc = "Field `ATESTMUX` writer - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type AtestmuxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Atestmux, crate::Safe>;
impl<'a, REG> AtestmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiasout(self) -> &'a mut crate::W<REG> {
        self.variant(Atestmux::Ibiasout)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comp_vin_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Atestmux::CompVinNeg)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compout(self) -> &'a mut crate::W<REG> {
        self.variant(Atestmux::Compout)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Atestmux::Off)
    }
}
#[doc = "Field `HYSPOL` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type HyspolR = crate::BitReader;
#[doc = "Field `HYSPOL` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type HyspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn divpath(&self) -> DivpathR {
        DivpathR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hyssel(&self) -> HysselR {
        HysselR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nsel(&self) -> NselR {
        NselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edgcfg(&self) -> EdgcfgR {
        EdgcfgR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evten(&self) -> EvtenR {
        EvtenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wuensb(&self) -> WuensbR {
        WuensbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cout(&self) -> CoutR {
        CoutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn couten(&self) -> CoutenR {
        CoutenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evtifg(&self) -> EvtifgR {
        EvtifgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestmux(&self) -> AtestmuxR {
        AtestmuxR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hyspol(&self) -> HyspolR {
        HyspolR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<LpcmpcfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<LpcmpcfgSpec> {
        DivW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn divpath(&mut self) -> DivpathW<LpcmpcfgSpec> {
        DivpathW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hyssel(&mut self) -> HysselW<LpcmpcfgSpec> {
        HysselW::new(self, 5)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<LpcmpcfgSpec> {
        PselW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nsel(&mut self) -> NselW<LpcmpcfgSpec> {
        NselW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn edgcfg(&mut self) -> EdgcfgW<LpcmpcfgSpec> {
        EdgcfgW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn evten(&mut self) -> EvtenW<LpcmpcfgSpec> {
        EvtenW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wuensb(&mut self) -> WuensbW<LpcmpcfgSpec> {
        WuensbW::new(self, 18)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn couten(&mut self) -> CoutenW<LpcmpcfgSpec> {
        CoutenW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn evtifg(&mut self) -> EvtifgW<LpcmpcfgSpec> {
        EvtifgW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atestmux(&mut self) -> AtestmuxW<LpcmpcfgSpec> {
        AtestmuxW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hyspol(&mut self) -> HyspolW<LpcmpcfgSpec> {
        HyspolW::new(self, 30)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcmpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcmpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpcmpcfgSpec;
impl crate::RegisterSpec for LpcmpcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcmpcfg::R`](R) reader structure"]
impl crate::Readable for LpcmpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lpcmpcfg::W`](W) writer structure"]
impl crate::Writable for LpcmpcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCMPCFG to value 0"]
impl crate::Resettable for LpcmpcfgSpec {
    const RESET_VALUE: u32 = 0;
}
