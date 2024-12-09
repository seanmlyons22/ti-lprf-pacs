#[doc = "Register `PCSR` reader"]
pub type R = crate::R<PcsrSpec>;
#[doc = "Register `PCSR` writer"]
pub type W = crate::W<PcsrSpec>;
#[doc = "Field `EIASAMPLE` reader - 31:0\\]
The possible values of this field are: 0xFFFFFFFF One of the following is true: - The PE is halted in Debug state. - The Security Extension is implemented, the sampled instruction was executed in Secure state, and SecureNoninvasiveDebugAllowed() == FALSE. - NoninvasiveDebugAllowed() == FALSE. - DEMCR.TRCENA == 0. - The address of a recently-executed instruction is not available. Not 0xFFFFFFFF Instruction address of a recently executed instruction. Bit \\[0\\]
of the sample instruction address is 0."]
pub type EiasampleR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The possible values of this field are: 0xFFFFFFFF One of the following is true: - The PE is halted in Debug state. - The Security Extension is implemented, the sampled instruction was executed in Secure state, and SecureNoninvasiveDebugAllowed() == FALSE. - NoninvasiveDebugAllowed() == FALSE. - DEMCR.TRCENA == 0. - The address of a recently-executed instruction is not available. Not 0xFFFFFFFF Instruction address of a recently executed instruction. Bit \\[0\\]
of the sample instruction address is 0."]
    #[inline(always)]
    pub fn eiasample(&self) -> EiasampleR {
        EiasampleR::new(self.bits)
    }
}
impl W {}
#[doc = "Program Counter Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcsrSpec;
impl crate::RegisterSpec for PcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcsr::R`](R) reader structure"]
impl crate::Readable for PcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcsr::W`](W) writer structure"]
impl crate::Writable for PcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCSR to value 0"]
impl crate::Resettable for PcsrSpec {
    const RESET_VALUE: u32 = 0;
}
