#[doc = "Register `BAT` reader"]
pub type R = crate::R<BatSpec>;
#[doc = "Register `BAT` writer"]
pub type W = crate::W<BatSpec>;
#[doc = "Field `FRAC` reader - 7:0\\]
Fractional part, standard binary fractional encoding. 0x00: .0V ... 0x20: 1/8 = .125V 0x40: 1/4 = .25V 0x80: 1/2 = .5V ... 0xA0: 1/2 + 1/8 = .625V ... 0xFF: Max"]
pub type FracR = crate::FieldReader;
#[doc = "Field `INT` reader - 10:8\\]
Integer part: 0x0: 0V + fractional part ... 0x3: 3V + fractional part 0x4: 4V + fractional part"]
pub type IntR = crate::FieldReader;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Fractional part, standard binary fractional encoding. 0x00: .0V ... 0x20: 1/8 = .125V 0x40: 1/4 = .25V 0x80: 1/2 = .5V ... 0xA0: 1/2 + 1/8 = .625V ... 0xFF: Max"]
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Integer part: 0x0: 0V + fractional part ... 0x3: 3V + fractional part 0x4: 4V + fractional part"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {}
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BatSpec;
impl crate::RegisterSpec for BatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bat::R`](R) reader structure"]
impl crate::Readable for BatSpec {}
#[doc = "`write(|w| ..)` method takes [`bat::W`](W) writer structure"]
impl crate::Writable for BatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAT to value 0"]
impl crate::Resettable for BatSpec {
    const RESET_VALUE: u32 = 0;
}
