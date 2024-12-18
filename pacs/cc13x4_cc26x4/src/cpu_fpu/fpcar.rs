#[doc = "Register `FPCAR` reader"]
pub type R = crate::R<FpcarSpec>;
#[doc = "Register `FPCAR` writer"]
pub type W = crate::W<FpcarSpec>;
#[doc = "Field `RESERVED0` reader - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `ADDRESS` reader - 31:3\\]
The location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - 31:3\\]
The location of the unpopulated floating-point register space allocated on an exception stack frame"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
The location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
The location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<FpcarSpec> {
        AddressW::new(self, 3)
    }
}
#[doc = "Holds the location of the unpopulated floating-point register space allocated on an exception stack frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcarSpec;
impl crate::RegisterSpec for FpcarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpcar::R`](R) reader structure"]
impl crate::Readable for FpcarSpec {}
#[doc = "`write(|w| ..)` method takes [`fpcar::W`](W) writer structure"]
impl crate::Writable for FpcarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCAR to value 0"]
impl crate::Resettable for FpcarSpec {
    const RESET_VALUE: u32 = 0;
}
