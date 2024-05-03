#[doc = "Register `CMDDATA1` reader"]
pub type R = crate::R<Cmddata1Spec>;
#[doc = "Register `CMDDATA1` writer"]
pub type W = crate::W<Cmddata1Spec>;
#[doc = "31:0\\]
A 32-bit data value is placed in this field.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Val {
    #[doc = "4294967295: Maximum value of VAL"]
    Maximum = 4294967295,
    #[doc = "0: Minimum value of VAL"]
    Minimum = 0,
}
impl From<Val> for u32 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u32;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - 31:0\\]
A 32-bit data value is placed in this field."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            4294967295 => Some(Val::Maximum),
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
#[doc = "Field `VAL` writer - 31:0\\]
A 32-bit data value is placed in this field."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
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
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A 32-bit data value is placed in this field."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A 32-bit data value is placed in this field."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Cmddata1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmddata1Spec;
impl crate::RegisterSpec for Cmddata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddata1::R`](R) reader structure"]
impl crate::Readable for Cmddata1Spec {}
#[doc = "`write(|w| ..)` method takes [`cmddata1::W`](W) writer structure"]
impl crate::Writable for Cmddata1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDDATA1 to value 0xffff_ffff"]
impl crate::Resettable for Cmddata1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
