#[doc = "Register `MTPR` reader"]
pub struct R(crate::R<MTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTPR` writer"]
pub struct W(crate::W<MTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTPR_SPEC>;
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
impl From<crate::W<MTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPR` reader - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
pub type TPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPR` writer - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
pub type TPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTPR_SPEC, u8, u8, 7, O>;
#[doc = "Field `TPR_7` reader - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
pub type TPR_7_R = crate::BitReader<bool>;
#[doc = "Field `TPR_7` writer - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
pub type TPR_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTPR_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTPR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
    #[inline(always)]
    pub fn tpr_7(&self) -> TPR_7_R {
        TPR_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    #[must_use]
    pub fn tpr(&mut self) -> TPR_W<0> {
        TPR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn tpr_7(&mut self) -> TPR_7_W<7> {
        TPR_7_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtpr](index.html) module"]
pub struct MTPR_SPEC;
impl crate::RegisterSpec for MTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtpr::R](R) reader structure"]
impl crate::Readable for MTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtpr::W](W) writer structure"]
impl crate::Writable for MTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTPR to value 0x01"]
impl crate::Resettable for MTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
