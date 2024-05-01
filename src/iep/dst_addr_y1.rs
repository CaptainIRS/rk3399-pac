#[doc = "Register `DST_ADDR_Y1` reader"]
pub type R = crate::R<DstAddrY1Spec>;
#[doc = "Register `DST_ADDR_Y1` writer"]
pub type W = crate::W<DstAddrY1Spec>;
#[doc = "Field `DST_IMAGE_Y_MST` reader - Destination image data Y start address in Memory"]
pub type DstImageYMstR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_Y_MST` writer - Destination image data Y start address in Memory"]
pub type DstImageYMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination image data Y start address in Memory"]
    #[inline(always)]
    pub fn dst_image_y_mst(&self) -> DstImageYMstR {
        DstImageYMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination image data Y start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_y_mst(&mut self) -> DstImageYMstW<DstAddrY1Spec> {
        DstImageYMstW::new(self, 0)
    }
}
#[doc = "Start address of destination image(Y)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_y1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_y1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrY1Spec;
impl crate::RegisterSpec for DstAddrY1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_y1::R`](R) reader structure"]
impl crate::Readable for DstAddrY1Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_y1::W`](W) writer structure"]
impl crate::Writable for DstAddrY1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_Y1 to value 0"]
impl crate::Resettable for DstAddrY1Spec {
    const RESET_VALUE: u32 = 0;
}
