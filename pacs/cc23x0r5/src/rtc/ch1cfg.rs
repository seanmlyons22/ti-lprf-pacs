#[doc = "Register `CH1CFG` reader"]
pub type R = crate::R<Ch1cfgSpec>;
#[doc = "Register `CH1CFG` writer"]
pub type W = crate::W<Ch1cfgSpec>;
#[doc = "0:0\\]
Edge detect configuration for capture source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "1: Falling Edge."]
    Fall = 1,
    #[doc = "0: Rising Edge."]
    Rise = 0,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - 0:0\\]
Edge detect configuration for capture source"]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            true => Edge::Fall,
            false => Edge::Rise,
        }
    }
    #[doc = "Falling Edge."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Edge::Fall
    }
    #[doc = "Rising Edge."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Edge::Rise
    }
}
#[doc = "Field `EDGE` writer - 0:0\\]
Edge detect configuration for capture source"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling Edge."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Fall)
    }
    #[doc = "Rising Edge."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Rise)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Edge detect configuration for capture source"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
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
Edge detect configuration for capture source"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<Ch1cfgSpec> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Ch1cfgSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Channel 1 configuration register. This register can be used to select the capture edge for generating the capture event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cfgSpec;
impl crate::RegisterSpec for Ch1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cfg::R`](R) reader structure"]
impl crate::Readable for Ch1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1cfg::W`](W) writer structure"]
impl crate::Writable for Ch1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CFG to value 0"]
impl crate::Resettable for Ch1cfgSpec {
    const RESET_VALUE: u32 = 0;
}
