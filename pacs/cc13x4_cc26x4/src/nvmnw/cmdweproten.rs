#[doc = "Register `CMDWEPROTEN` reader"]
pub type R = crate::R<CmdweprotenSpec>;
#[doc = "Register `CMDWEPROTEN` writer"]
pub type W = crate::W<CmdweprotenSpec>;
#[doc = "0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the engr region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the engr region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the engr region will be protected from program and erase.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "1: Maximum value of VAL"]
    Maximum = 1,
    #[doc = "0: Minimum value of VAL"]
    Minimum = 0,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the engr region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the engr region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the engr region will be protected from program and erase."]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            true => Val::Maximum,
            false => Val::Minimum,
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
#[doc = "Field `VAL` writer - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the engr region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the engr region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the engr region will be protected from program and erase."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
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
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the engr region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the engr region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the engr region will be protected from program and erase."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Each bit protects 1 sector. bit \\[0\\]:When 1, sector 0 of the engr region will be protected from program and erase. bit \\[1\\]:When 1, sector 1 of the engr region will be protected from program and erase. : : bit \\[31\\]:When 1, sector 31 of the engr region will be protected from program and erase."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmdweprotenSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweproten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweproten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdweprotenSpec;
impl crate::RegisterSpec for CmdweprotenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdweproten::R`](R) reader structure"]
impl crate::Readable for CmdweprotenSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdweproten::W`](W) writer structure"]
impl crate::Writable for CmdweprotenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDWEPROTEN to value 0x01"]
impl crate::Resettable for CmdweprotenSpec {
    const RESET_VALUE: u32 = 0x01;
}
