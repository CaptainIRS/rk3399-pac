#[doc = "Register `DST_ADDR_CR_FTEMP` reader"]
pub type R = crate::R<DstAddrCrFtempSpec>;
#[doc = "Register `DST_ADDR_CR_FTEMP` writer"]
pub type W = crate::W<DstAddrCrFtempSpec>;
#[doc = "Field `DST_IMAGE_CR_MST_FTEMP` reader - Fraction part destination image data CR start address"]
pub type DstImageCrMstFtempR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_CR_MST_FTEMP` writer - Fraction part destination image data CR start address"]
pub type DstImageCrMstFtempW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fraction part destination image data CR start address"]
    #[inline(always)]
    pub fn dst_image_cr_mst_ftemp(&self) -> DstImageCrMstFtempR {
        DstImageCrMstFtempR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fraction part destination image data CR start address"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_cr_mst_ftemp(&mut self) -> DstImageCrMstFtempW<DstAddrCrFtempSpec> {
        DstImageCrMstFtempW::new(self, 0)
    }
}
#[doc = "Start address of destination image(CR fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cr_ftemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cr_ftemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrCrFtempSpec;
impl crate::RegisterSpec for DstAddrCrFtempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_cr_ftemp::R`](R) reader structure"]
impl crate::Readable for DstAddrCrFtempSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_cr_ftemp::W`](W) writer structure"]
impl crate::Writable for DstAddrCrFtempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_CR_FTEMP to value 0"]
impl crate::Resettable for DstAddrCrFtempSpec {
    const RESET_VALUE: u32 = 0;
}
