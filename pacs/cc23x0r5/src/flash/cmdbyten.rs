#[doc = "Register `CMDBYTEN` reader"]
pub type R = crate::R<CmdbytenSpec>;
#[doc = "Register `CMDBYTEN` writer"]
pub type W = crate::W<CmdbytenSpec>;
#[doc = "15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Val {
    #[doc = "65535: Maximum value of VAL"]
    Maximum = 65535,
    #[doc = "0: Minimum value of VAL"]
    Minimum = 0,
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
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            65535 => Some(Val::Maximum),
            0 => Some(Val::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value of VAL"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Val::Maximum
    }
    #[doc = "Minimum value of VAL"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Val::Minimum
    }
}
#[doc = "Field `VAL` writer - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Maximum value of VAL"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Maximum)
    }
    #[doc = "Minimum value of VAL"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Minimum)
    }
}
#[doc = "Field `RESERVED16` reader - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader;
#[doc = "Field `RESERVED16` writer - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED_31_18` reader - 31:18\\]
Reserved"]
pub type Reserved31_18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_31_18` writer - 31:18\\]
Reserved"]
pub type Reserved31_18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_18(&self) -> Reserved31_18R {
        Reserved31_18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Command Byte Enable value. A 1-bit per flash word byte value is placed in this register."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmdbytenSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<CmdbytenSpec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_18(&mut self) -> Reserved31_18W<CmdbytenSpec> {
        Reserved31_18W::new(self, 18)
    }
}
#[doc = "Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is written to all 0 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdbyten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdbyten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdbytenSpec;
impl crate::RegisterSpec for CmdbytenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdbyten::R`](R) reader structure"]
impl crate::Readable for CmdbytenSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdbyten::W`](W) writer structure"]
impl crate::Writable for CmdbytenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDBYTEN to value 0"]
impl crate::Resettable for CmdbytenSpec {
    const RESET_VALUE: u32 = 0;
}
