#[doc = "Register `LSUCNT` reader"]
pub type R = crate::R<LsucntSpec>;
#[doc = "Register `LSUCNT` writer"]
pub type W = crate::W<LsucntSpec>;
#[doc = "Field `LSUCNT` reader - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
pub type LsucntR = crate::FieldReader;
#[doc = "Field `LSUCNT` writer - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
pub type LsucntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    pub fn lsucnt(&self) -> LsucntR {
        LsucntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    #[must_use]
    pub fn lsucnt(&mut self) -> LsucntW<LsucntSpec> {
        LsucntW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<LsucntSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsucnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsucnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsucntSpec;
impl crate::RegisterSpec for LsucntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsucnt::R`](R) reader structure"]
impl crate::Readable for LsucntSpec {}
#[doc = "`write(|w| ..)` method takes [`lsucnt::W`](W) writer structure"]
impl crate::Writable for LsucntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSUCNT to value 0"]
impl crate::Resettable for LsucntSpec {
    const RESET_VALUE: u32 = 0;
}
