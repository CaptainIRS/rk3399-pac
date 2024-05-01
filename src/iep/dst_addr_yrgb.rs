#[doc = "Register `DST_ADDR_YRGB` reader"]
pub type R = crate::R<DstAddrYrgbSpec>;
#[doc = "Register `DST_ADDR_YRGB` writer"]
pub type W = crate::W<DstAddrYrgbSpec>;
#[doc = "Field `DST_IMAGE_YRGB_MST` reader - Destination image data YRGB start address in Memory"]
pub type DstImageYrgbMstR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_YRGB_MST` writer - Destination image data YRGB start address in Memory"]
pub type DstImageYrgbMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination image data YRGB start address in Memory"]
    #[inline(always)]
    pub fn dst_image_yrgb_mst(&self) -> DstImageYrgbMstR {
        DstImageYrgbMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination image data YRGB start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_yrgb_mst(&mut self) -> DstImageYrgbMstW<DstAddrYrgbSpec> {
        DstImageYrgbMstW::new(self, 0)
    }
}
#[doc = "Start address of destination image(Y/RGB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_yrgb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_yrgb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrYrgbSpec;
impl crate::RegisterSpec for DstAddrYrgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_yrgb::R`](R) reader structure"]
impl crate::Readable for DstAddrYrgbSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_yrgb::W`](W) writer structure"]
impl crate::Writable for DstAddrYrgbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_YRGB to value 0"]
impl crate::Resettable for DstAddrYrgbSpec {
    const RESET_VALUE: u32 = 0;
}
