#[doc = "Register `I2SSTMPSEL0` reader"]
pub type R = crate::R<I2sstmpsel0Spec>;
#[doc = "Register `I2SSTMPSEL0` writer"]
pub type W = crate::W<I2sstmpsel0Spec>;
#[doc = "7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 95"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "121: Always asserted"]
    AlwaysActive = 121,
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            121 => Some(Ev::AlwaysActive),
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == Ev::AlwaysActive
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ev::None
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AlwaysActive)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::None)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<I2sstmpsel0Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for I2S Subscriber 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sstmpsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sstmpsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sstmpsel0Spec;
impl crate::RegisterSpec for I2sstmpsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sstmpsel0::R`](R) reader structure"]
impl crate::Readable for I2sstmpsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`i2sstmpsel0::W`](W) writer structure"]
impl crate::Writable for I2sstmpsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SSTMPSEL0 to value 0x5f"]
impl crate::Resettable for I2sstmpsel0Spec {
    const RESET_VALUE: u32 = 0x5f;
}
