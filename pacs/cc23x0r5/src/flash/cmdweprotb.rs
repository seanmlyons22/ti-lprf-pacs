#[doc = "Register `CMDWEPROTB` reader"]
pub type R = crate::R<CmdweprotbSpec>;
#[doc = "Register `CMDWEPROTB` writer"]
pub type W = crate::W<CmdweprotbSpec>;
#[doc = "27:0\\]
Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register.\n\nValue on reset: 268435455"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Val {
    #[doc = "268435455: Maximum value of VAL"]
    Maximum = 268435455,
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
#[doc = "Field `VAL` reader - 27:0\\]
Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            268435455 => Some(Val::Maximum),
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
#[doc = "Field `VAL` writer - 27:0\\]
Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 28, Val>;
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
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Each bit protects a group of 8 sectors. When a bit is 1, the associated 8 sectors in the flash will be protected from program and erase. A maximum of 256 sectors can be protected with this register."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmdweprotbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprotb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprotb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdweprotbSpec;
impl crate::RegisterSpec for CmdweprotbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdweprotb::R`](R) reader structure"]
impl crate::Readable for CmdweprotbSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdweprotb::W`](W) writer structure"]
impl crate::Writable for CmdweprotbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDWEPROTB to value 0x0fff_ffff"]
impl crate::Resettable for CmdweprotbSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
