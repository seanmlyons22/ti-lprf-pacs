#[doc = "Register `VTOR` reader"]
pub type R = crate::R<VtorSpec>;
#[doc = "Register `VTOR` writer"]
pub type W = crate::W<VtorSpec>;
#[doc = "Field `RESERVED0` reader - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TBLOFF` reader - 31:7\\]
Bits 31 down to 7 of the vector table base offset."]
pub type TbloffR = crate::FieldReader<u32>;
#[doc = "Field `TBLOFF` writer - 31:7\\]
Bits 31 down to 7 of the vector table base offset."]
pub type TbloffW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Bits 31 down to 7 of the vector table base offset."]
    #[inline(always)]
    pub fn tbloff(&self) -> TbloffR {
        TbloffR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<VtorSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Bits 31 down to 7 of the vector table base offset."]
    #[inline(always)]
    #[must_use]
    pub fn tbloff(&mut self) -> TbloffW<VtorSpec> {
        TbloffW::new(self, 7)
    }
}
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VtorSpec;
impl crate::RegisterSpec for VtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtor::R`](R) reader structure"]
impl crate::Readable for VtorSpec {}
#[doc = "`write(|w| ..)` method takes [`vtor::W`](W) writer structure"]
impl crate::Writable for VtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VTOR to value 0"]
impl crate::Resettable for VtorSpec {
    const RESET_VALUE: u32 = 0;
}
