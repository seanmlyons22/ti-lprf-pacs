#[doc = "Register `FLASH_COORDINATE` reader"]
pub type R = crate::R<FlashCoordinateSpec>;
#[doc = "Register `FLASH_COORDINATE` writer"]
pub type W = crate::W<FlashCoordinateSpec>;
#[doc = "Field `YCOORDINATE` reader - 15:0\\]
Y coordinate of this unit on the wafer."]
pub type YcoordinateR = crate::FieldReader<u16>;
#[doc = "Field `XCOORDINATE` reader - 31:16\\]
X coordinate of this unit on the wafer."]
pub type XcoordinateR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn ycoordinate(&self) -> YcoordinateR {
        YcoordinateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn xcoordinate(&self) -> XcoordinateR {
        XcoordinateR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Flash information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_coordinate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_coordinate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCoordinateSpec;
impl crate::RegisterSpec for FlashCoordinateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_coordinate::R`](R) reader structure"]
impl crate::Readable for FlashCoordinateSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_coordinate::W`](W) writer structure"]
impl crate::Writable for FlashCoordinateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_COORDINATE to value 0"]
impl crate::Resettable for FlashCoordinateSpec {
    const RESET_VALUE: u32 = 0;
}
