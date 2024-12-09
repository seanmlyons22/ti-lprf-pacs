#[doc = "Register `COMPTEST` reader"]
pub type R = crate::R<ComptestSpec>;
#[doc = "Register `COMPTEST` writer"]
pub type W = crate::W<ComptestSpec>;
#[doc = "0:0\\]
Enable test mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Enable test mode."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::Enable,
            false => En::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Enable test mode."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable test mode."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
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
Enable test mode."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ComptestSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Enable BATMON comparator test mode. Note: This register is write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comptest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comptest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComptestSpec;
impl crate::RegisterSpec for ComptestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comptest::R`](R) reader structure"]
impl crate::Readable for ComptestSpec {}
#[doc = "`write(|w| ..)` method takes [`comptest::W`](W) writer structure"]
impl crate::Writable for ComptestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPTEST to value 0"]
impl crate::Resettable for ComptestSpec {
    const RESET_VALUE: u32 = 0;
}
