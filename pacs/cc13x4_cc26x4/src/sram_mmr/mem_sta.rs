#[doc = "Register `MEM_STA` reader"]
pub type R = crate::R<MemStaSpec>;
#[doc = "Register `MEM_STA` writer"]
pub type W = crate::W<MemStaSpec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `MEM_STA` reader - 31:8\\]
Memory Instance Status This field gives the current status of each SRAM instance. When an instance is being initialized the corresponding bit is set to 1, 0 otherwise. bit\\[x\\]: 0 : Instance x is in normal mode 1 : Instance x is getting initialized"]
pub type MemStaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Memory Instance Status This field gives the current status of each SRAM instance. When an instance is being initialized the corresponding bit is set to 1, 0 otherwise. bit\\[x\\]: 0 : Instance x is in normal mode 1 : Instance x is getting initialized"]
    #[inline(always)]
    pub fn mem_sta(&self) -> MemStaR {
        MemStaR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Memory Status Controls memory initialization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemStaSpec;
impl crate::RegisterSpec for MemStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sta::R`](R) reader structure"]
impl crate::Readable for MemStaSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_sta::W`](W) writer structure"]
impl crate::Writable for MemStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_STA to value 0"]
impl crate::Resettable for MemStaSpec {
    const RESET_VALUE: u32 = 0;
}
