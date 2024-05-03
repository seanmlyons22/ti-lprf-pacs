#[doc = "Register `DCRDR` reader"]
pub type R = crate::R<DcrdrSpec>;
#[doc = "Register `DCRDR` writer"]
pub type W = crate::W<DcrdrSpec>;
#[doc = "Field `DBGTMP` reader - 31:0\\]
Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
pub type DbgtmpR = crate::FieldReader<u32>;
#[doc = "Field `DBGTMP` writer - 31:0\\]
Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
pub type DbgtmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
    #[inline(always)]
    pub fn dbgtmp(&self) -> DbgtmpR {
        DbgtmpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Provides debug access for reading and writing the general-purpose registers, special-purpose registers, and Floating-point Extension registers"]
    #[inline(always)]
    #[must_use]
    pub fn dbgtmp(&mut self) -> DbgtmpW<DcrdrSpec> {
        DbgtmpW::new(self, 0)
    }
}
#[doc = "With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrdrSpec;
impl crate::RegisterSpec for DcrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrdr::R`](R) reader structure"]
impl crate::Readable for DcrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcrdr::W`](W) writer structure"]
impl crate::Writable for DcrdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCRDR to value 0"]
impl crate::Resettable for DcrdrSpec {
    const RESET_VALUE: u32 = 0;
}
