#[doc = "Register `TIMEBIT` reader"]
pub type R = crate::R<TimebitSpec>;
#[doc = "Register `TIMEBIT` writer"]
pub type W = crate::W<TimebitSpec>;
#[doc = "15:0\\]
The corresponding bit will have value '1' rest should be '0'. If more than one bit is asserted, output is \"or\" of all the bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Val {
    #[doc = "32768: Bit17 is forwarded to the event fabric."]
    Bit17 = 32768,
    #[doc = "16384: Bit16 is forwarded to the event fabric."]
    Bit16 = 16384,
    #[doc = "8192: Bit15 is forwarded to the event fabric."]
    Bit15 = 8192,
    #[doc = "4096: Bit14 is forwarded to the event fabric."]
    Bit14 = 4096,
    #[doc = "2048: Bit13 is forwarded to the event fabric."]
    Bit13 = 2048,
    #[doc = "1024: Bit12 is forwarded to the event fabric."]
    Bit12 = 1024,
    #[doc = "512: Bit11 is forwarded to the event fabric."]
    Bit11 = 512,
    #[doc = "256: Bit10 is forwarded to the event fabric."]
    Bit10 = 256,
    #[doc = "128: Bit9 is forwarded to the event fabric."]
    Bit9 = 128,
    #[doc = "64: Bit8 is forwarded to the event fabric."]
    Bit8 = 64,
    #[doc = "32: Bit7 is forwarded to the event fabric."]
    Bit7 = 32,
    #[doc = "16: Bit6 is forwarded to the event fabric."]
    Bit6 = 16,
    #[doc = "8: Bit5 is forwarded to the event fabric."]
    Bit5 = 8,
    #[doc = "4: Bit4 is forwarded to the event fabric."]
    Bit4 = 4,
    #[doc = "2: Bit3 is forwarded to the event fabric."]
    Bit3 = 2,
    #[doc = "1: Bit2 is forwarded to the event fabric."]
    Bit2 = 1,
    #[doc = "0: No bit is forwarded to the event fabric."]
    Nobit = 0,
}
impl From<Val> for u16 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u16;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - 15:0\\]
The corresponding bit will have value '1' rest should be '0'. If more than one bit is asserted, output is \"or\" of all the bits."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            32768 => Some(Val::Bit17),
            16384 => Some(Val::Bit16),
            8192 => Some(Val::Bit15),
            4096 => Some(Val::Bit14),
            2048 => Some(Val::Bit13),
            1024 => Some(Val::Bit12),
            512 => Some(Val::Bit11),
            256 => Some(Val::Bit10),
            128 => Some(Val::Bit9),
            64 => Some(Val::Bit8),
            32 => Some(Val::Bit7),
            16 => Some(Val::Bit6),
            8 => Some(Val::Bit5),
            4 => Some(Val::Bit4),
            2 => Some(Val::Bit3),
            1 => Some(Val::Bit2),
            0 => Some(Val::Nobit),
            _ => None,
        }
    }
    #[doc = "Bit17 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit17(&self) -> bool {
        *self == Val::Bit17
    }
    #[doc = "Bit16 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == Val::Bit16
    }
    #[doc = "Bit15 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit15(&self) -> bool {
        *self == Val::Bit15
    }
    #[doc = "Bit14 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit14(&self) -> bool {
        *self == Val::Bit14
    }
    #[doc = "Bit13 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit13(&self) -> bool {
        *self == Val::Bit13
    }
    #[doc = "Bit12 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit12(&self) -> bool {
        *self == Val::Bit12
    }
    #[doc = "Bit11 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == Val::Bit11
    }
    #[doc = "Bit10 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == Val::Bit10
    }
    #[doc = "Bit9 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == Val::Bit9
    }
    #[doc = "Bit8 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == Val::Bit8
    }
    #[doc = "Bit7 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == Val::Bit7
    }
    #[doc = "Bit6 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit6(&self) -> bool {
        *self == Val::Bit6
    }
    #[doc = "Bit5 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit5(&self) -> bool {
        *self == Val::Bit5
    }
    #[doc = "Bit4 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == Val::Bit4
    }
    #[doc = "Bit3 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit3(&self) -> bool {
        *self == Val::Bit3
    }
    #[doc = "Bit2 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_bit2(&self) -> bool {
        *self == Val::Bit2
    }
    #[doc = "No bit is forwarded to the event fabric."]
    #[inline(always)]
    pub fn is_nobit(&self) -> bool {
        *self == Val::Nobit
    }
}
#[doc = "Field `VAL` writer - 15:0\\]
The corresponding bit will have value '1' rest should be '0'. If more than one bit is asserted, output is \"or\" of all the bits."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Bit17 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit17(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit17)
    }
    #[doc = "Bit16 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit16)
    }
    #[doc = "Bit15 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit15(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit15)
    }
    #[doc = "Bit14 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit14(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit14)
    }
    #[doc = "Bit13 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit13(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit13)
    }
    #[doc = "Bit12 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit12(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit12)
    }
    #[doc = "Bit11 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit11)
    }
    #[doc = "Bit10 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit10)
    }
    #[doc = "Bit9 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit9)
    }
    #[doc = "Bit8 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit8)
    }
    #[doc = "Bit7 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit7)
    }
    #[doc = "Bit6 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit6(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit6)
    }
    #[doc = "Bit5 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit5(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit5)
    }
    #[doc = "Bit4 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit4)
    }
    #[doc = "Bit3 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit3(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit3)
    }
    #[doc = "Bit2 is forwarded to the event fabric."]
    #[inline(always)]
    pub fn bit2(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Bit2)
    }
    #[doc = "No bit is forwarded to the event fabric."]
    #[inline(always)]
    pub fn nobit(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Nobit)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The corresponding bit will have value '1' rest should be '0'. If more than one bit is asserted, output is \"or\" of all the bits."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The corresponding bit will have value '1' rest should be '0'. If more than one bit is asserted, output is \"or\" of all the bits."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<TimebitSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TimebitSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Systimer Bit. This Register will be used to specify which TIME bit is required by LGPT to be forwarded from SYSTIMER.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timebit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timebit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimebitSpec;
impl crate::RegisterSpec for TimebitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timebit::R`](R) reader structure"]
impl crate::Readable for TimebitSpec {}
#[doc = "`write(|w| ..)` method takes [`timebit::W`](W) writer structure"]
impl crate::Writable for TimebitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEBIT to value 0"]
impl crate::Resettable for TimebitSpec {
    const RESET_VALUE: u32 = 0;
}
