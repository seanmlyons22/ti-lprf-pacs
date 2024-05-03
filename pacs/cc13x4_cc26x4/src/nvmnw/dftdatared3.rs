#[doc = "Register `DFTDATARED3` reader"]
pub type R = crate::R<Dftdatared3Spec>;
#[doc = "Register `DFTDATARED3` writer"]
pub type W = crate::W<Dftdatared3Spec>;
#[doc = "Field `VAL` reader - 3:0\\]
Data for redundant bits"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - 3:0\\]
Data for redundant bits"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Data for redundant bits"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Data for redundant bits"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Dftdatared3Spec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Dftdatared3Spec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftdatared3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftdatared3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dftdatared3Spec;
impl crate::RegisterSpec for Dftdatared3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftdatared3::R`](R) reader structure"]
impl crate::Readable for Dftdatared3Spec {}
#[doc = "`write(|w| ..)` method takes [`dftdatared3::W`](W) writer structure"]
impl crate::Writable for Dftdatared3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTDATARED3 to value 0x0f"]
impl crate::Resettable for Dftdatared3Spec {
    const RESET_VALUE: u32 = 0x0f;
}
