#[doc = "Register `IRQTYPE` reader"]
pub type R = crate::R<IrqtypeSpec>;
#[doc = "Register `IRQTYPE` writer"]
pub type W = crate::W<IrqtypeSpec>;
#[doc = "Field `LEVEL` reader - 0:0\\]
If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LevelR = crate::BitReader;
#[doc = "Field `LEVEL` writer - 0:0\\]
If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 1) != 0)
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
If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LevelW<IrqtypeSpec> {
        LevelW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<IrqtypeSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Control Interrupt Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqtypeSpec;
impl crate::RegisterSpec for IrqtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqtype::R`](R) reader structure"]
impl crate::Readable for IrqtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`irqtype::W`](W) writer structure"]
impl crate::Writable for IrqtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQTYPE to value 0"]
impl crate::Resettable for IrqtypeSpec {
    const RESET_VALUE: u32 = 0;
}
