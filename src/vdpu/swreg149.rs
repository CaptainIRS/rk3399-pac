#[doc = "Register `SWREG149` reader"]
pub type R = crate::R<Swreg149Spec>;
#[doc = "Register `SWREG149` writer"]
pub type W = crate::W<Swreg149Spec>;
#[doc = "Field `MFR_REG29` reader - multi format reuse register29 except h264\n\nVP7:\n\n\\[31:2\\]
: the segmentation map value start address\n\n\\[1\\]
: enable for segmentation map update\n\n\\[0\\]
: enable for segmentation"]
pub type MfrReg29R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG29` writer - multi format reuse register29 except h264\n\nVP7:\n\n\\[31:2\\]
: the segmentation map value start address\n\n\\[1\\]
: enable for segmentation map update\n\n\\[0\\]
: enable for segmentation"]
pub type MfrReg29W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register29 except h264\n\nVP7:\n\n\\[31:2\\]
: the segmentation map value start address\n\n\\[1\\]
: enable for segmentation map update\n\n\\[0\\]
: enable for segmentation"]
    #[inline(always)]
    pub fn mfr_reg29(&self) -> MfrReg29R {
        MfrReg29R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register29 except h264\n\nVP7:\n\n\\[31:2\\]
: the segmentation map value start address\n\n\\[1\\]
: enable for segmentation map update\n\n\\[0\\]
: enable for segmentation"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg29(&mut self) -> MfrReg29W<Swreg149Spec> {
        MfrReg29W::new(self, 0)
    }
}
#[doc = "multi format reuse register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg149::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg149::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg149Spec;
impl crate::RegisterSpec for Swreg149Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg149::R`](R) reader structure"]
impl crate::Readable for Swreg149Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg149::W`](W) writer structure"]
impl crate::Writable for Swreg149Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG149 to value 0"]
impl crate::Resettable for Swreg149Spec {
    const RESET_VALUE: u32 = 0;
}
