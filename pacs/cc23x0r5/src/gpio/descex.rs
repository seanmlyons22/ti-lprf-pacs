#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "5:0\\]
This provides the total number of DIOs supported by GPIO. The number of DIOs supprted is NUMDIO + 1\n\nValue on reset: 25"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numdio {
    #[doc = "63: Highest possible value"]
    Maximum = 63,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Numdio> for u8 {
    #[inline(always)]
    fn from(variant: Numdio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numdio {
    type Ux = u8;
}
impl crate::IsEnum for Numdio {}
#[doc = "Field `NUMDIO` reader - 5:0\\]
This provides the total number of DIOs supported by GPIO. The number of DIOs supprted is NUMDIO + 1"]
pub type NumdioR = crate::FieldReader<Numdio>;
impl NumdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numdio> {
        match self.bits {
            63 => Some(Numdio::Maximum),
            0 => Some(Numdio::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Numdio::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Numdio::Minimum
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
This provides the total number of DIOs supported by GPIO. The number of DIOs supprted is NUMDIO + 1"]
    #[inline(always)]
    pub fn numdio(&self) -> NumdioR {
        NumdioR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
#[doc = "Provide IP-specific instance information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescexSpec;
impl crate::RegisterSpec for DescexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex::R`](R) reader structure"]
impl crate::Readable for DescexSpec {}
#[doc = "`write(|w| ..)` method takes [`descex::W`](W) writer structure"]
impl crate::Writable for DescexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX to value 0x19"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0x19;
}
