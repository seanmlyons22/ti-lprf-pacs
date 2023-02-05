#[doc = "Register `CFGCMD` reader"]
pub type R = crate::R<CfgcmdSpec>;
#[doc = "Register `CFGCMD` writer"]
pub type W = crate::W<CfgcmdSpec>;
#[doc = "3:0\\]
Wait State setting for program verify, erase verify and read verify\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Waitstate {
    #[doc = "15: Maximum value"]
    Maximum = 15,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Waitstate> for u8 {
    #[inline(always)]
    fn from(variant: Waitstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Waitstate {
    type Ux = u8;
}
impl crate::IsEnum for Waitstate {}
#[doc = "Field `WAITSTATE` reader - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
pub type WaitstateR = crate::FieldReader<Waitstate>;
impl WaitstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Waitstate> {
        match self.bits {
            15 => Some(Waitstate::Maximum),
            0 => Some(Waitstate::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Waitstate::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Waitstate::Minimum
    }
}
#[doc = "Field `WAITSTATE` writer - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
pub type WaitstateW<'a, REG> = crate::FieldWriter<'a, REG, 4, Waitstate>;
impl<'a, REG> WaitstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Waitstate::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Waitstate::Minimum)
    }
}
#[doc = "Field `RESERVED4` reader - 6:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED_31_7` reader - 31:7\\]
Reserved"]
pub type Reserved31_7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_31_7` writer - 31:7\\]
Reserved"]
pub type Reserved31_7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
    #[inline(always)]
    pub fn waitstate(&self) -> WaitstateR {
        WaitstateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_7(&self) -> Reserved31_7R {
        Reserved31_7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Wait State setting for program verify, erase verify and read verify"]
    #[inline(always)]
    #[must_use]
    pub fn waitstate(&mut self) -> WaitstateW<CfgcmdSpec> {
        WaitstateW::new(self, 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_7(&mut self) -> Reserved31_7W<CfgcmdSpec> {
        Reserved31_7W::new(self, 7)
    }
}
#[doc = "Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgcmdSpec;
impl crate::RegisterSpec for CfgcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgcmd::R`](R) reader structure"]
impl crate::Readable for CfgcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgcmd::W`](W) writer structure"]
impl crate::Writable for CfgcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGCMD to value 0x02"]
impl crate::Resettable for CfgcmdSpec {
    const RESET_VALUE: u32 = 0x02;
}
