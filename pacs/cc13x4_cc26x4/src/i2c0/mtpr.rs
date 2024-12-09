#[doc = "Register `MTPR` reader"]
pub type R = crate::R<MtprSpec>;
#[doc = "Register `MTPR` writer"]
pub type W = crate::W<MtprSpec>;
#[doc = "Field `TPR` reader - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
pub type TprR = crate::FieldReader;
#[doc = "Field `TPR` writer - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
pub type TprW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TPR_7` reader - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
pub type Tpr7R = crate::BitReader;
#[doc = "Field `TPR_7` writer - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
pub type Tpr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    pub fn tpr(&self) -> TprR {
        TprR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
    #[inline(always)]
    pub fn tpr_7(&self) -> Tpr7R {
        Tpr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    #[must_use]
    pub fn tpr(&mut self) -> TprW<MtprSpec> {
        TprW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn tpr_7(&mut self) -> Tpr7W<MtprSpec> {
        Tpr7W::new(self, 7)
    }
}
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtprSpec;
impl crate::RegisterSpec for MtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtpr::R`](R) reader structure"]
impl crate::Readable for MtprSpec {}
#[doc = "`write(|w| ..)` method takes [`mtpr::W`](W) writer structure"]
impl crate::Writable for MtprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTPR to value 0x01"]
impl crate::Resettable for MtprSpec {
    const RESET_VALUE: u32 = 0x01;
}
