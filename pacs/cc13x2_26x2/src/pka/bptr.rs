#[doc = "Register `BPTR` reader"]
pub type R = crate::R<BptrSpec>;
#[doc = "Register `BPTR` writer"]
pub type W = crate::W<BptrSpec>;
#[doc = "Field `BPTR` reader - 10:0\\]
This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type BptrR = crate::FieldReader<u16>;
#[doc = "Field `BPTR` writer - 10:0\\]
This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type BptrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Set to zero on write, ignore on read"]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Set to zero on write, ignore on read"]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn bptr(&self) -> BptrR {
        BptrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    #[must_use]
    pub fn bptr(&mut self) -> BptrW<BptrSpec> {
        BptrW::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<BptrSpec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "PKA Vector B Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BptrSpec;
impl crate::RegisterSpec for BptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bptr::R`](R) reader structure"]
impl crate::Readable for BptrSpec {}
#[doc = "`write(|w| ..)` method takes [`bptr::W`](W) writer structure"]
impl crate::Writable for BptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPTR to value 0"]
impl crate::Resettable for BptrSpec {
    const RESET_VALUE: u32 = 0;
}
