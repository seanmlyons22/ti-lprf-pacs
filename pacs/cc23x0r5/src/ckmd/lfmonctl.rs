#[doc = "Register `LFMONCTL` reader"]
pub type R = crate::R<LfmonctlSpec>;
#[doc = "Register `LFMONCTL` writer"]
pub type W = crate::W<LfmonctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Enable LFMONITOR. Enable only after a LF clock source has been selected, enabled and is stable. If LFMONITOR detects a clock loss, the system will be reset."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Enable LFMONITOR. Enable only after a LF clock source has been selected, enabled and is stable. If LFMONITOR detects a clock loss, the system will be reset."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable LFMONITOR. Enable only after a LF clock source has been selected, enabled and is stable. If LFMONITOR detects a clock loss, the system will be reset."]
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
Enable LFMONITOR. Enable only after a LF clock source has been selected, enabled and is stable. If LFMONITOR detects a clock loss, the system will be reset."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<LfmonctlSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Low-frequency clock-monitor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfmonctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfmonctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfmonctlSpec;
impl crate::RegisterSpec for LfmonctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfmonctl::R`](R) reader structure"]
impl crate::Readable for LfmonctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfmonctl::W`](W) writer structure"]
impl crate::Writable for LfmonctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFMONCTL to value 0"]
impl crate::Resettable for LfmonctlSpec {
    const RESET_VALUE: u32 = 0;
}
