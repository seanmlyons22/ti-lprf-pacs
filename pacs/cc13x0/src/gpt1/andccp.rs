#[doc = "Register `ANDCCP` reader"]
pub type R = crate::R<AndccpSpec>;
#[doc = "Register `ANDCCP` writer"]
pub type W = crate::W<AndccpSpec>;
#[doc = "Field `CCP_AND_EN` reader - 0:0\\]
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
pub type CcpAndEnR = crate::BitReader;
#[doc = "Field `CCP_AND_EN` writer - 0:0\\]
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
pub type CcpAndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
    #[inline(always)]
    pub fn ccp_and_en(&self) -> CcpAndEnR {
        CcpAndEnR::new((self.bits & 1) != 0)
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
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
    #[inline(always)]
    #[must_use]
    pub fn ccp_and_en(&mut self) -> CcpAndEnW<AndccpSpec> {
        CcpAndEnW::new(self, 0)
    }
}
#[doc = "Combined CCP Output This register is used to logically AND CCP output pairs for each timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`andccp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`andccp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AndccpSpec;
impl crate::RegisterSpec for AndccpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`andccp::R`](R) reader structure"]
impl crate::Readable for AndccpSpec {}
#[doc = "`write(|w| ..)` method takes [`andccp::W`](W) writer structure"]
impl crate::Writable for AndccpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANDCCP to value 0"]
impl crate::Resettable for AndccpSpec {
    const RESET_VALUE: u32 = 0;
}
