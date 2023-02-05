#[doc = "Register `PDSTAT1` reader"]
pub struct R(crate::R<PDSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSTAT1` writer"]
pub struct W(crate::W<PDSTAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSTAT1_SPEC>;
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
impl From<crate::W<PDSTAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSTAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT1_SPEC, bool, O>;
#[doc = "Field `CPU_ON` reader - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
pub type CPU_ON_R = crate::BitReader<bool>;
#[doc = "Field `CPU_ON` writer - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
pub type CPU_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT1_SPEC, bool, O>;
#[doc = "Field `RFC_ON` reader - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
pub type RFC_ON_R = crate::BitReader<bool>;
#[doc = "Field `RFC_ON` writer - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
pub type RFC_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT1_SPEC, bool, O>;
#[doc = "Field `VIMS_MODE` reader - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
pub type VIMS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `VIMS_MODE` writer - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
pub type VIMS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT1_SPEC, bool, O>;
#[doc = "Field `BUS_ON` reader - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
pub type BUS_ON_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ON` writer - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
pub type BUS_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSTAT1_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDSTAT1_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
    #[inline(always)]
    pub fn cpu_on(&self) -> CPU_ON_R {
        CPU_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
    #[inline(always)]
    pub fn vims_mode(&self) -> VIMS_MODE_R {
        VIMS_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
    #[inline(always)]
    pub fn bus_on(&self) -> BUS_ON_R {
        BUS_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_on(&mut self) -> CPU_ON_W<1> {
        CPU_ON_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
    #[inline(always)]
    #[must_use]
    pub fn rfc_on(&mut self) -> RFC_ON_W<2> {
        RFC_ON_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
    #[inline(always)]
    #[must_use]
    pub fn vims_mode(&mut self) -> VIMS_MODE_W<3> {
        VIMS_MODE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
    #[inline(always)]
    #[must_use]
    pub fn bus_on(&mut self) -> BUS_ON_W<4> {
        BUS_ON_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Manager Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1](index.html) module"]
pub struct PDSTAT1_SPEC;
impl crate::RegisterSpec for PDSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdstat1::R](R) reader structure"]
impl crate::Readable for PDSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdstat1::W](W) writer structure"]
impl crate::Writable for PDSTAT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSTAT1 to value 0x1a"]
impl crate::Resettable for PDSTAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1a;
}
