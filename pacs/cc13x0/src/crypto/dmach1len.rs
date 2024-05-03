#[doc = "Register `DMACH1LEN` reader"]
pub type R = crate::R<Dmach1lenSpec>;
#[doc = "Register `DMACH1LEN` writer"]
pub type W = crate::W<Dmach1lenSpec>;
#[doc = "Field `LEN` reader - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
pub type LenR = crate::FieldReader<u16>;
#[doc = "Field `LEN` writer - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<Dmach1lenSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Dmach1lenSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "DMA Channel 1 Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach1lenSpec;
impl crate::RegisterSpec for Dmach1lenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach1len::R`](R) reader structure"]
impl crate::Readable for Dmach1lenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmach1len::W`](W) writer structure"]
impl crate::Writable for Dmach1lenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH1LEN to value 0"]
impl crate::Resettable for Dmach1lenSpec {
    const RESET_VALUE: u32 = 0;
}
