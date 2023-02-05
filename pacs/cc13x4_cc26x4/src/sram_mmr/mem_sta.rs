#[doc = "Register `MEM_STA` reader"]
pub struct R(crate::R<MEM_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_STA` writer"]
pub struct W(crate::W<MEM_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_STA_SPEC>;
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
impl From<crate::W<MEM_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEM_STA_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEM_STA` reader - 31:8\\]
Memory Instance Status This field gives the current status of each SRAM instance. When an instance is being initialized the corresponding bit is set to 1, 0 otherwise. bit\\[x\\]: 0 : Instance x is in normal mode 1 : Instance x is getting initialized"]
pub type MEM_STA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEM_STA` writer - 31:8\\]
Memory Instance Status This field gives the current status of each SRAM instance. When an instance is being initialized the corresponding bit is set to 1, 0 otherwise. bit\\[x\\]: 0 : Instance x is in normal mode 1 : Instance x is getting initialized"]
pub type MEM_STA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEM_STA_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Memory Instance Status This field gives the current status of each SRAM instance. When an instance is being initialized the corresponding bit is set to 1, 0 otherwise. bit\\[x\\]: 0 : Instance x is in normal mode 1 : Instance x is getting initialized"]
    #[inline(always)]
    pub fn mem_sta(&self) -> MEM_STA_R {
        MEM_STA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Memory Instance Status This field gives the current status of each SRAM instance. When an instance is being initialized the corresponding bit is set to 1, 0 otherwise. bit\\[x\\]: 0 : Instance x is in normal mode 1 : Instance x is getting initialized"]
    #[inline(always)]
    #[must_use]
    pub fn mem_sta(&mut self) -> MEM_STA_W<8> {
        MEM_STA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Status Controls memory initialization\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_sta](index.html) module"]
pub struct MEM_STA_SPEC;
impl crate::RegisterSpec for MEM_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_sta::R](R) reader structure"]
impl crate::Readable for MEM_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_sta::W](W) writer structure"]
impl crate::Writable for MEM_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_STA to value 0"]
impl crate::Resettable for MEM_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
