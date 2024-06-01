#[doc = "Register `WUGATE` reader"]
pub type R = crate::R<WugateSpec>;
#[doc = "Register `WUGATE` writer"]
pub type W = crate::W<WugateSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
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
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<WugateSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<WugateSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wugate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wugate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WugateSpec;
impl crate::RegisterSpec for WugateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wugate::R`](R) reader structure"]
impl crate::Readable for WugateSpec {}
#[doc = "`write(|w| ..)` method takes [`wugate::W`](W) writer structure"]
impl crate::Writable for WugateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUGATE to value 0"]
impl crate::Resettable for WugateSpec {
    const RESET_VALUE: u32 = 0;
}
