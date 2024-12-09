#[doc = "Register `SWSTMP` reader"]
pub type R = crate::R<SwstmpSpec>;
#[doc = "Register `SWSTMP` writer"]
pub type W = crate::W<SwstmpSpec>;
#[doc = "0:0\\]
SW ready Set by SW to indicate when SW is ready. Used to measure DELTA.TIME and DELTA.SLWP. This bit is auto-cleared by HW.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrdy {
    #[doc = "1: Set SW ready time stamp"]
    Set = 1,
    #[doc = "0: No effect"]
    Noeff = 0,
}
impl From<Swrdy> for bool {
    #[inline(always)]
    fn from(variant: Swrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRDY` reader - 0:0\\]
SW ready Set by SW to indicate when SW is ready. Used to measure DELTA.TIME and DELTA.SLWP. This bit is auto-cleared by HW."]
pub type SwrdyR = crate::BitReader<Swrdy>;
impl SwrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrdy {
        match self.bits {
            true => Swrdy::Set,
            false => Swrdy::Noeff,
        }
    }
    #[doc = "Set SW ready time stamp"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Swrdy::Set
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Swrdy::Noeff
    }
}
#[doc = "Field `SWRDY` writer - 0:0\\]
SW ready Set by SW to indicate when SW is ready. Used to measure DELTA.TIME and DELTA.SLWP. This bit is auto-cleared by HW."]
pub type SwrdyW<'a, REG> = crate::BitWriter<'a, REG, Swrdy>;
impl<'a, REG> SwrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set SW ready time stamp"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Swrdy::Set)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Swrdy::Noeff)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SW ready Set by SW to indicate when SW is ready. Used to measure DELTA.TIME and DELTA.SLWP. This bit is auto-cleared by HW."]
    #[inline(always)]
    pub fn swrdy(&self) -> SwrdyR {
        SwrdyR::new((self.bits & 1) != 0)
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
SW ready Set by SW to indicate when SW is ready. Used to measure DELTA.TIME and DELTA.SLWP. This bit is auto-cleared by HW."]
    #[inline(always)]
    #[must_use]
    pub fn swrdy(&mut self) -> SwrdyW<SwstmpSpec> {
        SwrdyW::new(self, 0)
    }
}
#[doc = "SW Time Stamp Register. This register is used to set the SW time stamp for the delta time measurement.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swstmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swstmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwstmpSpec;
impl crate::RegisterSpec for SwstmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swstmp::R`](R) reader structure"]
impl crate::Readable for SwstmpSpec {}
#[doc = "`write(|w| ..)` method takes [`swstmp::W`](W) writer structure"]
impl crate::Writable for SwstmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWSTMP to value 0"]
impl crate::Resettable for SwstmpSpec {
    const RESET_VALUE: u32 = 0;
}
