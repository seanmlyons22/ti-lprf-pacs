#[doc = "Register `PER_CHK` reader"]
pub type R = crate::R<PerChkSpec>;
#[doc = "Register `PER_CHK` writer"]
pub type W = crate::W<PerChkSpec>;
#[doc = "Field `PER_ADDR` reader - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
pub type PerAddrR = crate::FieldReader<u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Parity Error Address Offset Returns the last address offset which resulted in a parity error during an SRAM read. The address offset returned is always the word-aligned address that contains the location with the parity error. For parity faults on non word-aligned accesses, CPU_SCS:BFAR.ADDRESS will hold the address of the location that resulted in parity error."]
    #[inline(always)]
    pub fn per_addr(&self) -> PerAddrR {
        PerAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Parity Error Check Parity error check results\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_chk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_chk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerChkSpec;
impl crate::RegisterSpec for PerChkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_chk::R`](R) reader structure"]
impl crate::Readable for PerChkSpec {}
#[doc = "`write(|w| ..)` method takes [`per_chk::W`](W) writer structure"]
impl crate::Writable for PerChkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER_CHK to value 0"]
impl crate::Resettable for PerChkSpec {
    const RESET_VALUE: u32 = 0;
}