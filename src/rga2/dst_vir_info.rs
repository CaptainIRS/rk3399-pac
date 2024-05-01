#[doc = "Register `DST_VIR_INFO` reader"]
pub type R = crate::R<DstVirInfoSpec>;
#[doc = "Register `DST_VIR_INFO` writer"]
pub type W = crate::W<DstVirInfoSpec>;
#[doc = "Field `SW_DST_VIR_STRIDE` reader - destination image virtual stride(words)"]
pub type SwDstVirStrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_DST_VIR_STRIDE` writer - destination image virtual stride(words)"]
pub type SwDstVirStrideW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SW_SRC1_VIR_STRIDE` reader - source image 1 virtual stride (words)"]
pub type SwSrc1VirStrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC1_VIR_STRIDE` writer - source image 1 virtual stride (words)"]
pub type SwSrc1VirStrideW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - destination image virtual stride(words)"]
    #[inline(always)]
    pub fn sw_dst_vir_stride(&self) -> SwDstVirStrideR {
        SwDstVirStrideR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - source image 1 virtual stride (words)"]
    #[inline(always)]
    pub fn sw_src1_vir_stride(&self) -> SwSrc1VirStrideR {
        SwSrc1VirStrideR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - destination image virtual stride(words)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_vir_stride(&mut self) -> SwDstVirStrideW<DstVirInfoSpec> {
        SwDstVirStrideW::new(self, 0)
    }
    #[doc = "Bits 16:27 - source image 1 virtual stride (words)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_vir_stride(&mut self) -> SwSrc1VirStrideW<DstVirInfoSpec> {
        SwSrc1VirStrideW::new(self, 16)
    }
}
#[doc = "RGA destination image virtual width/height register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_vir_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_vir_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstVirInfoSpec;
impl crate::RegisterSpec for DstVirInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_vir_info::R`](R) reader structure"]
impl crate::Readable for DstVirInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_vir_info::W`](W) writer structure"]
impl crate::Writable for DstVirInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_VIR_INFO to value 0"]
impl crate::Resettable for DstVirInfoSpec {
    const RESET_VALUE: u32 = 0;
}
