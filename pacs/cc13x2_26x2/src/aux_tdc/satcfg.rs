#[doc = "Register `SATCFG` reader"]
pub type R = crate::R<SatcfgSpec>;
#[doc = "Register `SATCFG` writer"]
pub type W = crate::W<SatcfgSpec>;
#[doc = "3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Limit {
    #[doc = "15: Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    R24 = 15,
    #[doc = "14: Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    R23 = 14,
    #[doc = "13: Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    R22 = 13,
    #[doc = "12: Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    R21 = 12,
    #[doc = "11: Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    R20 = 11,
    #[doc = "10: Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    R19 = 10,
    #[doc = "9: Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    R18 = 9,
    #[doc = "8: Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    R17 = 8,
    #[doc = "7: Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    R16 = 7,
    #[doc = "6: Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    R15 = 6,
    #[doc = "5: Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    R14 = 5,
    #[doc = "4: Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    R13 = 4,
    #[doc = "3: Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    R12 = 3,
}
impl From<Limit> for u8 {
    #[inline(always)]
    fn from(variant: Limit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Limit {
    type Ux = u8;
}
impl crate::IsEnum for Limit {}
#[doc = "Field `LIMIT` reader - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
pub type LimitR = crate::FieldReader<Limit>;
impl LimitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Limit> {
        match self.bits {
            15 => Some(Limit::R24),
            14 => Some(Limit::R23),
            13 => Some(Limit::R22),
            12 => Some(Limit::R21),
            11 => Some(Limit::R20),
            10 => Some(Limit::R19),
            9 => Some(Limit::R18),
            8 => Some(Limit::R17),
            7 => Some(Limit::R16),
            6 => Some(Limit::R15),
            5 => Some(Limit::R14),
            4 => Some(Limit::R13),
            3 => Some(Limit::R12),
            _ => None,
        }
    }
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    #[inline(always)]
    pub fn is_r24(&self) -> bool {
        *self == Limit::R24
    }
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    #[inline(always)]
    pub fn is_r23(&self) -> bool {
        *self == Limit::R23
    }
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    #[inline(always)]
    pub fn is_r22(&self) -> bool {
        *self == Limit::R22
    }
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    #[inline(always)]
    pub fn is_r21(&self) -> bool {
        *self == Limit::R21
    }
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    #[inline(always)]
    pub fn is_r20(&self) -> bool {
        *self == Limit::R20
    }
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    #[inline(always)]
    pub fn is_r19(&self) -> bool {
        *self == Limit::R19
    }
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    #[inline(always)]
    pub fn is_r18(&self) -> bool {
        *self == Limit::R18
    }
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    #[inline(always)]
    pub fn is_r17(&self) -> bool {
        *self == Limit::R17
    }
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    #[inline(always)]
    pub fn is_r16(&self) -> bool {
        *self == Limit::R16
    }
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    #[inline(always)]
    pub fn is_r15(&self) -> bool {
        *self == Limit::R15
    }
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    #[inline(always)]
    pub fn is_r14(&self) -> bool {
        *self == Limit::R14
    }
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    #[inline(always)]
    pub fn is_r13(&self) -> bool {
        *self == Limit::R13
    }
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    #[inline(always)]
    pub fn is_r12(&self) -> bool {
        *self == Limit::R12
    }
}
#[doc = "Field `LIMIT` writer - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 4, Limit>;
impl<'a, REG> LimitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    #[inline(always)]
    pub fn r24(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R24)
    }
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    #[inline(always)]
    pub fn r23(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R23)
    }
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    #[inline(always)]
    pub fn r22(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R22)
    }
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    #[inline(always)]
    pub fn r21(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R21)
    }
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    #[inline(always)]
    pub fn r20(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R20)
    }
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    #[inline(always)]
    pub fn r19(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R19)
    }
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    #[inline(always)]
    pub fn r18(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R18)
    }
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    #[inline(always)]
    pub fn r17(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R17)
    }
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    #[inline(always)]
    pub fn r16(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R16)
    }
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    #[inline(always)]
    pub fn r15(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R15)
    }
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    #[inline(always)]
    pub fn r14(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R14)
    }
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    #[inline(always)]
    pub fn r13(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R13)
    }
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    #[inline(always)]
    pub fn r12(self) -> &'a mut crate::W<REG> {
        self.variant(Limit::R12)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LimitW<SatcfgSpec> {
        LimitW::new(self, 0)
    }
}
#[doc = "Saturation Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`satcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`satcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatcfgSpec;
impl crate::RegisterSpec for SatcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`satcfg::R`](R) reader structure"]
impl crate::Readable for SatcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`satcfg::W`](W) writer structure"]
impl crate::Writable for SatcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATCFG to value 0x0f"]
impl crate::Resettable for SatcfgSpec {
    const RESET_VALUE: u32 = 0x0f;
}
