#[doc = "Register `SRC_ADDR_YRGB` reader"]
pub type R = crate::R<SrcAddrYrgbSpec>;
#[doc = "Register `SRC_ADDR_YRGB` writer"]
pub type W = crate::W<SrcAddrYrgbSpec>;
#[doc = "Field `SRC_IMAGE_YRGB_MST` reader - Source image data YRGB start address in Memory"]
pub type SrcImageYrgbMstR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_YRGB_MST` writer - Source image data YRGB start address in Memory"]
pub type SrcImageYrgbMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source image data YRGB start address in Memory"]
    #[inline(always)]
    pub fn src_image_yrgb_mst(&self) -> SrcImageYrgbMstR {
        SrcImageYrgbMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source image data YRGB start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_yrgb_mst(&mut self) -> SrcImageYrgbMstW<SrcAddrYrgbSpec> {
        SrcImageYrgbMstW::new(self, 0)
    }
}
#[doc = "Start address of source image(Y/RGB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_yrgb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_yrgb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrYrgbSpec;
impl crate::RegisterSpec for SrcAddrYrgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_yrgb::R`](R) reader structure"]
impl crate::Readable for SrcAddrYrgbSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_yrgb::W`](W) writer structure"]
impl crate::Writable for SrcAddrYrgbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_YRGB to value 0"]
impl crate::Resettable for SrcAddrYrgbSpec {
    const RESET_VALUE: u32 = 0;
}
