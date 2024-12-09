#[doc = "Register `CMDDATAINDEX` reader"]
pub type R = crate::R<CmddataindexSpec>;
#[doc = "Register `CMDDATAINDEX` writer"]
pub type W = crate::W<CmddataindexSpec>;
#[doc = "1:0\\]
Data register index\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Val {
    #[doc = "3: Maximum value of VAL"]
    Maximum = 3,
    #[doc = "0: Minimum value of VAL"]
    Minimum = 0,
}
impl From<Val> for u8 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u8;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` reader - 1:0\\]
Data register index"]
pub type ValR = crate::FieldReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Val> {
        match self.bits {
            3 => Some(Val::Maximum),
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
#[doc = "Field `VAL` writer - 1:0\\]
Data register index"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 2, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
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
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Data register index"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Data register index"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmddataindexSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddataindex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddataindex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmddataindexSpec;
impl crate::RegisterSpec for CmddataindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddataindex::R`](R) reader structure"]
impl crate::Readable for CmddataindexSpec {}
#[doc = "`write(|w| ..)` method takes [`cmddataindex::W`](W) writer structure"]
impl crate::Writable for CmddataindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDDATAINDEX to value 0"]
impl crate::Resettable for CmddataindexSpec {
    const RESET_VALUE: u32 = 0;
}
