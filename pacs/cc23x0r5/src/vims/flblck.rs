#[doc = "Register `FLBLCK` reader"]
pub type R = crate::R<FlblckSpec>;
#[doc = "Register `FLBLCK` writer"]
pub type W = crate::W<FlblckSpec>;
#[doc = "0:0\\]
Used to allow or block flash operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    #[doc = "1: Block"]
    Block = 1,
    #[doc = "0: Allow"]
    Allow = 0,
}
impl From<Val> for bool {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - 0:0\\]
Used to allow or block flash operation."]
pub type ValR = crate::BitReader<Val>;
impl ValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Val {
        match self.bits {
            true => Val::Block,
            false => Val::Allow,
        }
    }
    #[doc = "Block"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Val::Block
    }
    #[doc = "Allow"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == Val::Allow
    }
}
#[doc = "Field `VAL` writer - 0:0\\]
Used to allow or block flash operation."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Block"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Block)
    }
    #[doc = "Allow"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Allow)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Used to allow or block flash operation."]
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
Used to allow or block flash operation."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<FlblckSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "This register is used to block user read, write and erase operation to flash. This register is sticky when written with value 1. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flblck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flblck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlblckSpec;
impl crate::RegisterSpec for FlblckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flblck::R`](R) reader structure"]
impl crate::Readable for FlblckSpec {}
#[doc = "`write(|w| ..)` method takes [`flblck::W`](W) writer structure"]
impl crate::Writable for FlblckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLBLCK to value 0"]
impl crate::Resettable for FlblckSpec {
    const RESET_VALUE: u32 = 0;
}
