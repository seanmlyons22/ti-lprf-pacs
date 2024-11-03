#[doc = "Register `LFQUALCTL` reader"]
pub type R = crate::R<LfqualctlSpec>;
#[doc = "Register `LFQUALCTL` writer"]
pub type W = crate::W<LfqualctlSpec>;
#[doc = "Field `CONSEC` reader - 7:0\\]
Number of consecutive times the LFCLK period error has to be smaller than MAXERR to be considered \"good\". Setting this value to 0 will bypass clock qualification, and the \"good\" indicator will always be 1."]
pub type ConsecR = crate::FieldReader;
#[doc = "Field `CONSEC` writer - 7:0\\]
Number of consecutive times the LFCLK period error has to be smaller than MAXERR to be considered \"good\". Setting this value to 0 will bypass clock qualification, and the \"good\" indicator will always be 1."]
pub type ConsecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAXERR` reader - 13:8\\]
Maximum LFCLK period error. Value given in microseconds, 3 integer bits + 3 fractional bits."]
pub type MaxerrR = crate::FieldReader;
#[doc = "Field `MAXERR` writer - 13:8\\]
Maximum LFCLK period error. Value given in microseconds, 3 integer bits + 3 fractional bits."]
pub type MaxerrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved14W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of consecutive times the LFCLK period error has to be smaller than MAXERR to be considered \"good\". Setting this value to 0 will bypass clock qualification, and the \"good\" indicator will always be 1."]
    #[inline(always)]
    pub fn consec(&self) -> ConsecR {
        ConsecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Maximum LFCLK period error. Value given in microseconds, 3 integer bits + 3 fractional bits."]
    #[inline(always)]
    pub fn maxerr(&self) -> MaxerrR {
        MaxerrR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of consecutive times the LFCLK period error has to be smaller than MAXERR to be considered \"good\". Setting this value to 0 will bypass clock qualification, and the \"good\" indicator will always be 1."]
    #[inline(always)]
    #[must_use]
    pub fn consec(&mut self) -> ConsecW<LfqualctlSpec> {
        ConsecW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Maximum LFCLK period error. Value given in microseconds, 3 integer bits + 3 fractional bits."]
    #[inline(always)]
    #[must_use]
    pub fn maxerr(&mut self) -> MaxerrW<LfqualctlSpec> {
        MaxerrW::new(self, 8)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> Reserved14W<LfqualctlSpec> {
        Reserved14W::new(self, 14)
    }
}
#[doc = "Low frequency clock qualification control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfqualctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfqualctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfqualctlSpec;
impl crate::RegisterSpec for LfqualctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfqualctl::R`](R) reader structure"]
impl crate::Readable for LfqualctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfqualctl::W`](W) writer structure"]
impl crate::Writable for LfqualctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFQUALCTL to value 0x2064"]
impl crate::Resettable for LfqualctlSpec {
    const RESET_VALUE: u32 = 0x2064;
}
