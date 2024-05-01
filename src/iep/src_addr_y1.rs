#[doc = "Register `SRC_ADDR_Y1` reader"]
pub type R = crate::R<SrcAddrY1Spec>;
#[doc = "Register `SRC_ADDR_Y1` writer"]
pub type W = crate::W<SrcAddrY1Spec>;
#[doc = "Field `SRC_IMAGE_Y_MST` reader - Source image data Y start address in Memory"]
pub type SrcImageYMstR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_Y_MST` writer - Source image data Y start address in Memory"]
pub type SrcImageYMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source image data Y start address in Memory"]
    #[inline(always)]
    pub fn src_image_y_mst(&self) -> SrcImageYMstR {
        SrcImageYMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source image data Y start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_y_mst(&mut self) -> SrcImageYMstW<SrcAddrY1Spec> {
        SrcImageYMstW::new(self, 0)
    }
}
#[doc = "Start address of source image(Y)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_y1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_y1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrY1Spec;
impl crate::RegisterSpec for SrcAddrY1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_y1::R`](R) reader structure"]
impl crate::Readable for SrcAddrY1Spec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_y1::W`](W) writer structure"]
impl crate::Writable for SrcAddrY1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_Y1 to value 0"]
impl crate::Resettable for SrcAddrY1Spec {
    const RESET_VALUE: u32 = 0;
}
