#[doc = "Register `SHUTDOWN` reader"]
pub type R = crate::R<ShutdownSpec>;
#[doc = "Register `SHUTDOWN` writer"]
pub type W = crate::W<ShutdownSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Shutdown control. 0: Do not write 0 to this bit. 1: Immediately start the process to enter shutdown mode"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Shutdown control. 0: Do not write 0 to this bit. 1: Immediately start the process to enter shutdown mode"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Shutdown control. 0: Do not write 0 to this bit. 1: Immediately start the process to enter shutdown mode"]
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
Shutdown control. 0: Do not write 0 to this bit. 1: Immediately start the process to enter shutdown mode"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ShutdownSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ShutdownSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shutdown::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shutdown::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShutdownSpec;
impl crate::RegisterSpec for ShutdownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdown::R`](R) reader structure"]
impl crate::Readable for ShutdownSpec {}
#[doc = "`write(|w| ..)` method takes [`shutdown::W`](W) writer structure"]
impl crate::Writable for ShutdownSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHUTDOWN to value 0"]
impl crate::Resettable for ShutdownSpec {
    const RESET_VALUE: u32 = 0;
}
