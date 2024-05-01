#[doc = "Register `SRC_VIR_INFO` reader"]
pub type R = crate::R<SrcVirInfoSpec>;
#[doc = "Register `SRC_VIR_INFO` writer"]
pub type W = crate::W<SrcVirInfoSpec>;
#[doc = "Field `SW_SRC_VIR_STRIDE` reader - src image virtual stride (words)"]
pub type SwSrcVirStrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_VIR_STRIDE` writer - src image virtual stride (words)"]
pub type SwSrcVirStrideW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SW_MASK_VIR_STRIDE` reader - mask image virtual stride (words)"]
pub type SwMaskVirStrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_MASK_VIR_STRIDE` writer - mask image virtual stride (words)"]
pub type SwMaskVirStrideW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:14 - src image virtual stride (words)"]
    #[inline(always)]
    pub fn sw_src_vir_stride(&self) -> SwSrcVirStrideR {
        SwSrcVirStrideR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:25 - mask image virtual stride (words)"]
    #[inline(always)]
    pub fn sw_mask_vir_stride(&self) -> SwMaskVirStrideR {
        SwMaskVirStrideR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - src image virtual stride (words)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_vir_stride(&mut self) -> SwSrcVirStrideW<SrcVirInfoSpec> {
        SwSrcVirStrideW::new(self, 0)
    }
    #[doc = "Bits 16:25 - mask image virtual stride (words)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask_vir_stride(&mut self) -> SwMaskVirStrideW<SrcVirInfoSpec> {
        SwMaskVirStrideW::new(self, 16)
    }
}
#[doc = "RGA source image virtual stride / RGA source image tile number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_vir_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_vir_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcVirInfoSpec;
impl crate::RegisterSpec for SrcVirInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_vir_info::R`](R) reader structure"]
impl crate::Readable for SrcVirInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`src_vir_info::W`](W) writer structure"]
impl crate::Writable for SrcVirInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_VIR_INFO to value 0"]
impl crate::Resettable for SrcVirInfoSpec {
    const RESET_VALUE: u32 = 0;
}
