#[doc = "Register `CLK32KCTL` reader"]
pub type R = crate::R<Clk32kctlSpec>;
#[doc = "Register `CLK32KCTL` writer"]
pub type W = crate::W<Clk32kctlSpec>;
#[doc = "Field `OE_N` reader - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (e.g. IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
pub type OeNR = crate::BitReader;
#[doc = "Field `OE_N` writer - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (e.g. IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
pub type OeNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (e.g. IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
    #[inline(always)]
    pub fn oe_n(&self) -> OeNR {
        OeNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (e.g. IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
    #[inline(always)]
    #[must_use]
    pub fn oe_n(&mut self) -> OeNW<Clk32kctlSpec> {
        OeNW::new(self, 0)
    }
}
#[doc = "SCLK_LF External Output Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk32kctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk32kctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clk32kctlSpec;
impl crate::RegisterSpec for Clk32kctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk32kctl::R`](R) reader structure"]
impl crate::Readable for Clk32kctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk32kctl::W`](W) writer structure"]
impl crate::Writable for Clk32kctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK32KCTL to value 0x01"]
impl crate::Resettable for Clk32kctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
