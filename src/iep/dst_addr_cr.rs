#[doc = "Register `DST_ADDR_CR` reader"]
pub type R = crate::R<DstAddrCrSpec>;
#[doc = "Register `DST_ADDR_CR` writer"]
pub type W = crate::W<DstAddrCrSpec>;
#[doc = "Field `DST_IMAGE_CR_MST` reader - Destination image data CR start address in Memory"]
pub type DstImageCrMstR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_CR_MST` writer - Destination image data CR start address in Memory"]
pub type DstImageCrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination image data CR start address in Memory"]
    #[inline(always)]
    pub fn dst_image_cr_mst(&self) -> DstImageCrMstR {
        DstImageCrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination image data CR start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_cr_mst(&mut self) -> DstImageCrMstW<DstAddrCrSpec> {
        DstImageCrMstW::new(self, 0)
    }
}
#[doc = "Start address of destination image(Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrCrSpec;
impl crate::RegisterSpec for DstAddrCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_cr::R`](R) reader structure"]
impl crate::Readable for DstAddrCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_cr::W`](W) writer structure"]
impl crate::Writable for DstAddrCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_CR to value 0"]
impl crate::Resettable for DstAddrCrSpec {
    const RESET_VALUE: u32 = 0;
}
