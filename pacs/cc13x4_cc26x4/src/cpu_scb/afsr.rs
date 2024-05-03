#[doc = "Register `AFSR` reader"]
pub type R = crate::R<AfsrSpec>;
#[doc = "Register `AFSR` writer"]
pub type W = crate::W<AfsrSpec>;
#[doc = "Field `IMPDEF` reader - 31:0\\]
Implementation defined. The bits map directly onto the signal assignment to the auxiliary fault inputs. Tied to 0"]
pub type ImpdefR = crate::FieldReader<u32>;
#[doc = "Field `IMPDEF` writer - 31:0\\]
Implementation defined. The bits map directly onto the signal assignment to the auxiliary fault inputs. Tied to 0"]
pub type ImpdefW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Implementation defined. The bits map directly onto the signal assignment to the auxiliary fault inputs. Tied to 0"]
    #[inline(always)]
    pub fn impdef(&self) -> ImpdefR {
        ImpdefR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Implementation defined. The bits map directly onto the signal assignment to the auxiliary fault inputs. Tied to 0"]
    #[inline(always)]
    #[must_use]
    pub fn impdef(&mut self) -> ImpdefW<AfsrSpec> {
        ImpdefW::new(self, 0)
    }
}
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfsrSpec;
impl crate::RegisterSpec for AfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsr::R`](R) reader structure"]
impl crate::Readable for AfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`afsr::W`](W) writer structure"]
impl crate::Writable for AfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSR to value 0"]
impl crate::Resettable for AfsrSpec {
    const RESET_VALUE: u32 = 0;
}
