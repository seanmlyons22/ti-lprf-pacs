#[doc = "Register `SWRESET` reader"]
pub type R = crate::R<SwresetSpec>;
#[doc = "Register `SWRESET` writer"]
pub type W = crate::W<SwresetSpec>;
#[doc = "Field `RESET` reader - 0:0\\]
Write '1' to soft reset , reset will be low for 4-5 clock cycles. Poll to 0 for reset to be completed."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 0:0\\]
Write '1' to soft reset , reset will be low for 4-5 clock cycles. Poll to 0 for reset to be completed."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write '1' to soft reset , reset will be low for 4-5 clock cycles. Poll to 0 for reset to be completed."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
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
Write '1' to soft reset , reset will be low for 4-5 clock cycles. Poll to 0 for reset to be completed."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<SwresetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "SW Reset Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwresetSpec;
impl crate::RegisterSpec for SwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreset::R`](R) reader structure"]
impl crate::Readable for SwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`swreset::W`](W) writer structure"]
impl crate::Writable for SwresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRESET to value 0"]
impl crate::Resettable for SwresetSpec {
    const RESET_VALUE: u32 = 0;
}
