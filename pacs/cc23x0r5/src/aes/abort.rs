#[doc = "Register `ABORT` reader"]
pub type R = crate::R<AbortSpec>;
#[doc = "Register `ABORT` writer"]
pub type W = crate::W<AbortSpec>;
#[doc = "Field `ECB` writer - 0:0\\]
Electronic Codebook Write 1 to this field to abort an ongoing ECB encryption. An abort will clear TXT, BUF, DMA, AUTOCFG registers"]
pub type EcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Electronic Codebook Write 1 to this field to abort an ongoing ECB encryption. An abort will clear TXT, BUF, DMA, AUTOCFG registers"]
    #[inline(always)]
    #[must_use]
    pub fn ecb(&mut self) -> EcbW<AbortSpec> {
        EcbW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AbortSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Abort Use this register to abort current ECB encryption.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abort::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abort::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbortSpec;
impl crate::RegisterSpec for AbortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abort::R`](R) reader structure"]
impl crate::Readable for AbortSpec {}
#[doc = "`write(|w| ..)` method takes [`abort::W`](W) writer structure"]
impl crate::Writable for AbortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABORT to value 0"]
impl crate::Resettable for AbortSpec {
    const RESET_VALUE: u32 = 0;
}
