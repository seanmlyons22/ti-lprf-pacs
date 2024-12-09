#[doc = "Register `MEM_CTL` reader"]
pub type R = crate::R<MemCtlSpec>;
#[doc = "Register `MEM_CTL` writer"]
pub type W = crate::W<MemCtlSpec>;
#[doc = "Field `MEM_CLR_EN` reader - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
pub type MemClrEnR = crate::BitReader;
#[doc = "Field `MEM_CLR_EN` writer - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
pub type MemClrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BUSY` reader - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
pub type MemBusyR = crate::BitReader;
#[doc = "Field `RESERVED24` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
    #[inline(always)]
    pub fn mem_clr_en(&self) -> MemClrEnR {
        MemClrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
    #[inline(always)]
    pub fn mem_busy(&self) -> MemBusyR {
        MemBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clr_en(&mut self) -> MemClrEnW<MemCtlSpec> {
        MemClrEnW::new(self, 0)
    }
}
#[doc = "Memory Control Controls memory initialization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemCtlSpec;
impl crate::RegisterSpec for MemCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctl::R`](R) reader structure"]
impl crate::Readable for MemCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ctl::W`](W) writer structure"]
impl crate::Writable for MemCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CTL to value 0"]
impl crate::Resettable for MemCtlSpec {
    const RESET_VALUE: u32 = 0;
}
