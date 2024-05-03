#[doc = "Register `CMDWEPROTA` reader"]
pub type R = crate::R<CmdweprotaSpec>;
#[doc = "Register `CMDWEPROTA` writer"]
pub type W = crate::W<CmdweprotaSpec>;
#[doc = "31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase.\n\nValue on reset: 4294967295"]
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
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
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
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
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
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the flash memory will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the flash memory will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the flash memory will be protected from program and erase."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmdweprotaSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprota::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprota::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdweprotaSpec;
impl crate::RegisterSpec for CmdweprotaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdweprota::R`](R) reader structure"]
impl crate::Readable for CmdweprotaSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdweprota::W`](W) writer structure"]
impl crate::Writable for CmdweprotaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDWEPROTA to value 0xffff_ffff"]
impl crate::Resettable for CmdweprotaSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
