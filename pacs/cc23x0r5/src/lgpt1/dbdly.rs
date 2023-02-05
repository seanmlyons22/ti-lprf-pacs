#[doc = "Register `DBDLY` reader"]
pub type R = crate::R<DbdlySpec>;
#[doc = "Register `DBDLY` writer"]
pub type W = crate::W<DbdlySpec>;
#[doc = "Field `RISEDLY` reader - 11:0\\]
Rise delay. The number of system clock periods inserted between the rise of the dead band reference signal and the rise of the output signal."]
pub type RisedlyR = crate::FieldReader<u16>;
#[doc = "Field `RISEDLY` writer - 11:0\\]
Rise delay. The number of system clock periods inserted between the rise of the dead band reference signal and the rise of the output signal."]
pub type RisedlyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `FALLDLY` reader - 27:16\\]
Fall delay. The number of system clock periods inserted between the fall of the dead band reference signal and the rise of the inverted output signal."]
pub type FalldlyR = crate::FieldReader<u16>;
#[doc = "Field `FALLDLY` writer - 27:16\\]
Fall delay. The number of system clock periods inserted between the fall of the dead band reference signal and the rise of the inverted output signal."]
pub type FalldlyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Rise delay. The number of system clock periods inserted between the rise of the dead band reference signal and the rise of the output signal."]
    #[inline(always)]
    pub fn risedly(&self) -> RisedlyR {
        RisedlyR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Fall delay. The number of system clock periods inserted between the fall of the dead band reference signal and the rise of the inverted output signal."]
    #[inline(always)]
    pub fn falldly(&self) -> FalldlyR {
        FalldlyR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Rise delay. The number of system clock periods inserted between the rise of the dead band reference signal and the rise of the output signal."]
    #[inline(always)]
    #[must_use]
    pub fn risedly(&mut self) -> RisedlyW<DbdlySpec> {
        RisedlyW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Fall delay. The number of system clock periods inserted between the fall of the dead band reference signal and the rise of the inverted output signal."]
    #[inline(always)]
    #[must_use]
    pub fn falldly(&mut self) -> FalldlyW<DbdlySpec> {
        FalldlyW::new(self, 16)
    }
}
#[doc = "Dead Band Delay This register is used to insert a dead band delay when generating complementary PWM signals. To enable dead band, on for example IO output 0, create a reference PWM signal on Output 0, then set DBCTL.IOC0 = EN. TBD: 12-bit width fall delay and rise delay may be excessive, if 8-bits are enough we can join DBDLY and DBCTL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbdlySpec;
impl crate::RegisterSpec for DbdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbdly::R`](R) reader structure"]
impl crate::Readable for DbdlySpec {}
#[doc = "`write(|w| ..)` method takes [`dbdly::W`](W) writer structure"]
impl crate::Writable for DbdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBDLY to value 0"]
impl crate::Resettable for DbdlySpec {
    const RESET_VALUE: u32 = 0;
}
