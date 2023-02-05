#[doc = "Register `MEM_CTL` reader"]
pub struct R(crate::R<MEM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CTL` writer"]
pub struct W(crate::W<MEM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CTL_SPEC>;
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
impl From<crate::W<MEM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_CLR_EN` reader - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
pub type MEM_CLR_EN_R = crate::BitReader<bool>;
#[doc = "Field `MEM_CLR_EN` writer - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
pub type MEM_CLR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEM_CTL_SPEC, bool, O>;
#[doc = "Field `MEM_BUSY` reader - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
pub type MEM_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MEM_BUSY` writer - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
pub type MEM_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEM_CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED24` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED24` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEM_CTL_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
    #[inline(always)]
    pub fn mem_clr_en(&self) -> MEM_CLR_EN_R {
        MEM_CLR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
    #[inline(always)]
    pub fn mem_busy(&self) -> MEM_BUSY_R {
        MEM_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clr_en(&mut self) -> MEM_CLR_EN_W<0> {
        MEM_CLR_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
    #[inline(always)]
    #[must_use]
    pub fn mem_busy(&mut self) -> MEM_BUSY_W<1> {
        MEM_BUSY_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<2> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Control Controls memory initialization\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_ctl](index.html) module"]
pub struct MEM_CTL_SPEC;
impl crate::RegisterSpec for MEM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_ctl::R](R) reader structure"]
impl crate::Readable for MEM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_ctl::W](W) writer structure"]
impl crate::Writable for MEM_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CTL to value 0"]
impl crate::Resettable for MEM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
