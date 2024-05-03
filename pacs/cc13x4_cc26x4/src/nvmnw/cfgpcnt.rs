#[doc = "Register `CFGPCNT` reader"]
pub type R = crate::R<CfgpcntSpec>;
#[doc = "Register `CFGPCNT` writer"]
pub type W = crate::W<CfgpcntSpec>;
#[doc = "0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxpcntovr {
    #[doc = "1: Use value from MAXPCNTVAL field as maximum puse count"]
    Override = 1,
    #[doc = "0: Use hard-wired (default) value for maximum pulse count"]
    Default = 0,
}
impl From<Maxpcntovr> for bool {
    #[inline(always)]
    fn from(variant: Maxpcntovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXPCNTOVR` reader - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MaxpcntovrR = crate::BitReader<Maxpcntovr>;
impl MaxpcntovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxpcntovr {
        match self.bits {
            true => Maxpcntovr::Override,
            false => Maxpcntovr::Default,
        }
    }
    #[doc = "Use value from MAXPCNTVAL field as maximum puse count"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Maxpcntovr::Override
    }
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Maxpcntovr::Default
    }
}
#[doc = "Field `MAXPCNTOVR` writer - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
pub type MaxpcntovrW<'a, REG> = crate::BitWriter<'a, REG, Maxpcntovr>;
impl<'a, REG> MaxpcntovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use value from MAXPCNTVAL field as maximum puse count"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Maxpcntovr::Override)
    }
    #[doc = "Use hard-wired (default) value for maximum pulse count"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Maxpcntovr::Default)
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxpcntval {
    #[doc = "255: Maximum value"]
    Maximum = 255,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Maxpcntval> for u8 {
    #[inline(always)]
    fn from(variant: Maxpcntval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxpcntval {
    type Ux = u8;
}
impl crate::IsEnum for Maxpcntval {}
#[doc = "Field `MAXPCNTVAL` reader - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
pub type MaxpcntvalR = crate::FieldReader<Maxpcntval>;
impl MaxpcntvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxpcntval> {
        match self.bits {
            255 => Some(Maxpcntval::Maximum),
            0 => Some(Maxpcntval::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Maxpcntval::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Maxpcntval::Minimum
    }
}
#[doc = "Field `MAXPCNTVAL` writer - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
pub type MaxpcntvalW<'a, REG> = crate::FieldWriter<'a, REG, 8, Maxpcntval>;
impl<'a, REG> MaxpcntvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Maxpcntval::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Maxpcntval::Minimum)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    pub fn maxpcntovr(&self) -> MaxpcntovrR {
        MaxpcntovrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
    #[inline(always)]
    pub fn maxpcntval(&self) -> MaxpcntvalR {
        MaxpcntvalR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Override hard-wired maximum pulse count. If MAXERSPCNTOVR is not set, then setting this value alone will override the max pulse count for both program and erase. If MAXERSPCNTOVR is set, then this bit will only control the max pulse count setting for program. By default, this bit is 0, and a hard-wired max pulse count is used."]
    #[inline(always)]
    #[must_use]
    pub fn maxpcntovr(&mut self) -> MaxpcntovrW<CfgpcntSpec> {
        MaxpcntovrW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CfgpcntSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Override maximum pulse counter with this value. If MAXPCNTOVR = 0, then this field is ignored. If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 0, then this value will be used to override the max pulse count for both program and erase. Full max value will be {4'h0, MAXPCNTVAL} . If MAXPCNTOVR = 1 and MAXERSPCNTOVR = 1, then this value will be used to override the max pulse count for program only. Full max value will be {4'h0, MAXPCNTVAL}."]
    #[inline(always)]
    #[must_use]
    pub fn maxpcntval(&mut self) -> MaxpcntvalW<CfgpcntSpec> {
        MaxpcntvalW::new(self, 4)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<CfgpcntSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgpcntSpec;
impl crate::RegisterSpec for CfgpcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgpcnt::R`](R) reader structure"]
impl crate::Readable for CfgpcntSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgpcnt::W`](W) writer structure"]
impl crate::Writable for CfgpcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGPCNT to value 0"]
impl crate::Resettable for CfgpcntSpec {
    const RESET_VALUE: u32 = 0;
}
