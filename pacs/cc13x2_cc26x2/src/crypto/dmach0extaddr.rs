#[doc = "Register `DMACH0EXTADDR` reader"]
pub type R = crate::R<Dmach0extaddrSpec>;
#[doc = "Register `DMACH0EXTADDR` writer"]
pub type W = crate::W<Dmach0extaddrSpec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface. Note: The crypto DMA copies out upto 3 bytes until it hits a word boundary, thus the address need not be word aligned."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface. Note: The crypto DMA copies out upto 3 bytes until it hits a word boundary, thus the address need not be word aligned."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface. Note: The crypto DMA copies out upto 3 bytes until it hits a word boundary, thus the address need not be word aligned."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface. Note: The crypto DMA copies out upto 3 bytes until it hits a word boundary, thus the address need not be word aligned."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Dmach0extaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel 0 External Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0extaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0extaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach0extaddrSpec;
impl crate::RegisterSpec for Dmach0extaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach0extaddr::R`](R) reader structure"]
impl crate::Readable for Dmach0extaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmach0extaddr::W`](W) writer structure"]
impl crate::Writable for Dmach0extaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH0EXTADDR to value 0"]
impl crate::Resettable for Dmach0extaddrSpec {
    const RESET_VALUE: u32 = 0;
}
