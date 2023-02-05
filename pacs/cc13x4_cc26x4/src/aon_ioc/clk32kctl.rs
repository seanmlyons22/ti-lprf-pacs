#[doc = "Register `CLK32KCTL` reader"]
pub struct R(crate::R<CLK32KCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK32KCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK32KCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK32KCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK32KCTL` writer"]
pub struct W(crate::W<CLK32KCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK32KCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK32KCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK32KCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OE_N` reader - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (for example IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
pub type OE_N_R = crate::BitReader<bool>;
#[doc = "Field `OE_N` writer - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (for example IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
pub type OE_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK32KCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK32KCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (for example IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
    #[inline(always)]
    pub fn oe_n(&self) -> OE_N_R {
        OE_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (for example IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
    #[inline(always)]
    #[must_use]
    pub fn oe_n(&mut self) -> OE_N_W<0> {
        OE_N_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCLK_LF External Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk32kctl](index.html) module"]
pub struct CLK32KCTL_SPEC;
impl crate::RegisterSpec for CLK32KCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk32kctl::R](R) reader structure"]
impl crate::Readable for CLK32KCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk32kctl::W](W) writer structure"]
impl crate::Writable for CLK32KCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK32KCTL to value 0x01"]
impl crate::Resettable for CLK32KCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
