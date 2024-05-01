#[doc = "Register `SWREG44_STRMD_ERROR_EN` reader"]
pub type R = crate::R<Swreg44StrmdErrorEnSpec>;
#[doc = "Register `SWREG44_STRMD_ERROR_EN` writer"]
pub type W = crate::W<Swreg44StrmdErrorEnSpec>;
#[doc = "Field `SW_STRMD_ERROR_E` reader - strmd error enable regs\n\nstrmd error enable regs\n\nin HEVC,it is called sw_cabac_error_e\n\nfor VP9, it use sw_strmd_error_e\\[3:0\\]\n\nsw_strmd_error_e\\[0\\]
is for sw_vp9_tilesize_error\n\nsw_strmd_error_e\\[1\\]
is for sw_vp9_segskip_error\n\nsw_strmd_error_e\\[2\\]
is for sw_vp9_error_init_error\n\nsw_strmd_error_e\\[3\\]
is for sw_vp9_uncpr\n\nsw_strmd_error_e\\[4\\]
is for sw_vp9_refscale_erro,\n\nsw_vp9_refscale_erro now is no use"]
pub type SwStrmdErrorER = crate::FieldReader<u32>;
#[doc = "Field `SW_STRMD_ERROR_E` writer - strmd error enable regs\n\nstrmd error enable regs\n\nin HEVC,it is called sw_cabac_error_e\n\nfor VP9, it use sw_strmd_error_e\\[3:0\\]\n\nsw_strmd_error_e\\[0\\]
is for sw_vp9_tilesize_error\n\nsw_strmd_error_e\\[1\\]
is for sw_vp9_segskip_error\n\nsw_strmd_error_e\\[2\\]
is for sw_vp9_error_init_error\n\nsw_strmd_error_e\\[3\\]
is for sw_vp9_uncpr\n\nsw_strmd_error_e\\[4\\]
is for sw_vp9_refscale_erro,\n\nsw_vp9_refscale_erro now is no use"]
pub type SwStrmdErrorEW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - strmd error enable regs\n\nstrmd error enable regs\n\nin HEVC,it is called sw_cabac_error_e\n\nfor VP9, it use sw_strmd_error_e\\[3:0\\]\n\nsw_strmd_error_e\\[0\\]
is for sw_vp9_tilesize_error\n\nsw_strmd_error_e\\[1\\]
is for sw_vp9_segskip_error\n\nsw_strmd_error_e\\[2\\]
is for sw_vp9_error_init_error\n\nsw_strmd_error_e\\[3\\]
is for sw_vp9_uncpr\n\nsw_strmd_error_e\\[4\\]
is for sw_vp9_refscale_erro,\n\nsw_vp9_refscale_erro now is no use"]
    #[inline(always)]
    pub fn sw_strmd_error_e(&self) -> SwStrmdErrorER {
        SwStrmdErrorER::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - strmd error enable regs\n\nstrmd error enable regs\n\nin HEVC,it is called sw_cabac_error_e\n\nfor VP9, it use sw_strmd_error_e\\[3:0\\]\n\nsw_strmd_error_e\\[0\\]
is for sw_vp9_tilesize_error\n\nsw_strmd_error_e\\[1\\]
is for sw_vp9_segskip_error\n\nsw_strmd_error_e\\[2\\]
is for sw_vp9_error_init_error\n\nsw_strmd_error_e\\[3\\]
is for sw_vp9_uncpr\n\nsw_strmd_error_e\\[4\\]
is for sw_vp9_refscale_erro,\n\nsw_vp9_refscale_erro now is no use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strmd_error_e(&mut self) -> SwStrmdErrorEW<Swreg44StrmdErrorEnSpec> {
        SwStrmdErrorEW::new(self, 0)
    }
}
#[doc = "cabac error enable config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg44_strmd_error_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg44_strmd_error_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg44StrmdErrorEnSpec;
impl crate::RegisterSpec for Swreg44StrmdErrorEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg44_strmd_error_en::R`](R) reader structure"]
impl crate::Readable for Swreg44StrmdErrorEnSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg44_strmd_error_en::W`](W) writer structure"]
impl crate::Writable for Swreg44StrmdErrorEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG44_STRMD_ERROR_EN to value 0"]
impl crate::Resettable for Swreg44StrmdErrorEnSpec {
    const RESET_VALUE: u32 = 0;
}
