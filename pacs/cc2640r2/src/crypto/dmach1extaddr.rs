#[doc = "Register `DMACH1EXTADDR` reader"]
pub type R = crate::R<Dmach1extaddrSpec>;
#[doc = "Register `DMACH1EXTADDR` writer"]
pub type W = crate::W<Dmach1extaddrSpec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Dmach1extaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "DMA Channel 1 External Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1extaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1extaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach1extaddrSpec;
impl crate::RegisterSpec for Dmach1extaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach1extaddr::R`](R) reader structure"]
impl crate::Readable for Dmach1extaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmach1extaddr::W`](W) writer structure"]
impl crate::Writable for Dmach1extaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH1EXTADDR to value 0"]
impl crate::Resettable for Dmach1extaddrSpec {
    const RESET_VALUE: u32 = 0;
}
