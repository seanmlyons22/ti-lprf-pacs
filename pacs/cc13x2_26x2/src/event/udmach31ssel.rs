#[doc = "Register `UDMACH31SSEL` reader"]
pub type R = crate::R<Udmach31sselSpec>;
#[doc = "Register `UDMACH31SSEL` writer"]
pub type W = crate::W<Udmach31sselSpec>;
#[doc = "31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ev {
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Ev> for u32 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u32;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ev::None
    }
}
#[doc = "Field `EV` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::None)
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Udmach31sselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach31ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach31ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach31sselSpec;
impl crate::RegisterSpec for Udmach31sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach31ssel::R`](R) reader structure"]
impl crate::Readable for Udmach31sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach31ssel::W`](W) writer structure"]
impl crate::Writable for Udmach31sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH31SSEL to value 0"]
impl crate::Resettable for Udmach31sselSpec {
    const RESET_VALUE: u32 = 0;
}
