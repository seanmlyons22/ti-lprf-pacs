#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Register `LSR` writer"]
pub type W = crate::W<LsrSpec>;
#[doc = "Field `PRESENT` reader - 0:0\\]
Indicates that a lock mechanism exists for this component."]
pub type PresentR = crate::BitReader;
#[doc = "Field `ACCESS` reader - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
pub type AccessR = crate::BitReader;
#[doc = "Field `BYTEACC` reader - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
pub type ByteaccR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock mechanism exists for this component."]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
    #[inline(always)]
    pub fn byteacc(&self) -> ByteaccR {
        ByteaccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {}
#[doc = "Lock Status Use this register to enable write accesses to the Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`write(|w| ..)` method takes [`lsr::W`](W) writer structure"]
impl crate::Writable for LsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSR to value 0x03"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0x03;
}
