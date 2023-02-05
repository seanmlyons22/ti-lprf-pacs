#[doc = "Register `REFCFG` reader"]
pub type R = crate::R<RefcfgSpec>;
#[doc = "Register `REFCFG` writer"]
pub type W = crate::W<RefcfgSpec>;
#[doc = "0:0\\]
Reference buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refen {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Refen> for bool {
    #[inline(always)]
    fn from(variant: Refen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFEN` reader - 0:0\\]
Reference buffer enable"]
pub type RefenR = crate::BitReader<Refen>;
impl RefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refen {
        match self.bits {
            true => Refen::Enable,
            false => Refen::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Refen::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Refen::Disable
    }
}
#[doc = "Field `REFEN` writer - 0:0\\]
Reference buffer enable"]
pub type RefenW<'a, REG> = crate::BitWriter<'a, REG, Refen>;
impl<'a, REG> RefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Refen::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Refen::Disable)
    }
}
#[doc = "1:1\\]
Configures reference buffer output voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refvsel {
    #[doc = "1: REFBUF generates 1.4V output"]
    V1p4 = 1,
    #[doc = "0: REFBUF generates 2.5V output"]
    V2p5 = 0,
}
impl From<Refvsel> for bool {
    #[inline(always)]
    fn from(variant: Refvsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFVSEL` reader - 1:1\\]
Configures reference buffer output voltage"]
pub type RefvselR = crate::BitReader<Refvsel>;
impl RefvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refvsel {
        match self.bits {
            true => Refvsel::V1p4,
            false => Refvsel::V2p5,
        }
    }
    #[doc = "REFBUF generates 1.4V output"]
    #[inline(always)]
    pub fn is_v1p4(&self) -> bool {
        *self == Refvsel::V1p4
    }
    #[doc = "REFBUF generates 2.5V output"]
    #[inline(always)]
    pub fn is_v2p5(&self) -> bool {
        *self == Refvsel::V2p5
    }
}
#[doc = "Field `REFVSEL` writer - 1:1\\]
Configures reference buffer output voltage"]
pub type RefvselW<'a, REG> = crate::BitWriter<'a, REG, Refvsel>;
impl<'a, REG> RefvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REFBUF generates 1.4V output"]
    #[inline(always)]
    pub fn v1p4(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::V1p4)
    }
    #[doc = "REFBUF generates 2.5V output"]
    #[inline(always)]
    pub fn v2p5(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::V2p5)
    }
}
#[doc = "Field `SPARE` reader - 2:2\\]
Spare bit"]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - 2:2\\]
Spare bit"]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:3\\]
Configures reference buffer bias current output value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ibprog {
    #[doc = "3: 0.67uA"]
    Val3 = 3,
    #[doc = "2: 2uA"]
    Val2 = 2,
    #[doc = "1: 0.5uA"]
    Val1 = 1,
    #[doc = "0: 1uA"]
    Val0 = 0,
}
impl From<Ibprog> for u8 {
    #[inline(always)]
    fn from(variant: Ibprog) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ibprog {
    type Ux = u8;
}
impl crate::IsEnum for Ibprog {}
#[doc = "Field `IBPROG` reader - 4:3\\]
Configures reference buffer bias current output value"]
pub type IbprogR = crate::FieldReader<Ibprog>;
impl IbprogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibprog {
        match self.bits {
            3 => Ibprog::Val3,
            2 => Ibprog::Val2,
            1 => Ibprog::Val1,
            0 => Ibprog::Val0,
            _ => unreachable!(),
        }
    }
    #[doc = "0.67uA"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Ibprog::Val3
    }
    #[doc = "2uA"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Ibprog::Val2
    }
    #[doc = "0.5uA"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Ibprog::Val1
    }
    #[doc = "1uA"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Ibprog::Val0
    }
}
#[doc = "Field `IBPROG` writer - 4:3\\]
Configures reference buffer bias current output value"]
pub type IbprogW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ibprog, crate::Safe>;
impl<'a, REG> IbprogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.67uA"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Ibprog::Val3)
    }
    #[doc = "2uA"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Ibprog::Val2)
    }
    #[doc = "0.5uA"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Ibprog::Val1)
    }
    #[doc = "1uA"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Ibprog::Val0)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reference buffer enable"]
    #[inline(always)]
    pub fn refen(&self) -> RefenR {
        RefenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Configures reference buffer output voltage"]
    #[inline(always)]
    pub fn refvsel(&self) -> RefvselR {
        RefvselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Spare bit"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Configures reference buffer bias current output value"]
    #[inline(always)]
    pub fn ibprog(&self) -> IbprogR {
        IbprogR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reference buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn refen(&mut self) -> RefenW<RefcfgSpec> {
        RefenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Configures reference buffer output voltage"]
    #[inline(always)]
    #[must_use]
    pub fn refvsel(&mut self) -> RefvselW<RefcfgSpec> {
        RefvselW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Spare bit"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<RefcfgSpec> {
        SpareW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Configures reference buffer bias current output value"]
    #[inline(always)]
    #[must_use]
    pub fn ibprog(&mut self) -> IbprogW<RefcfgSpec> {
        IbprogW::new(self, 3)
    }
}
#[doc = "Reference buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefcfgSpec;
impl crate::RegisterSpec for RefcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refcfg::R`](R) reader structure"]
impl crate::Readable for RefcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`refcfg::W`](W) writer structure"]
impl crate::Writable for RefcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REFCFG to value 0"]
impl crate::Resettable for RefcfgSpec {
    const RESET_VALUE: u32 = 0;
}
