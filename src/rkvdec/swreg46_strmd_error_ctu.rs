#[doc = "Register `SWREG46_STRMD_ERROR_CTU` reader"]
pub type R = crate::R<Swreg46StrmdErrorCtuSpec>;
#[doc = "Register `SWREG46_STRMD_ERROR_CTU` writer"]
pub type W = crate::W<Swreg46StrmdErrorCtuSpec>;
#[doc = "Field `SW_STRMD_ERROR_CTU_XOFFSET` reader - strmd error ctu xoffset\n\nstrmd error ctu xoffset\n\nfor all HEVC and H264 and VP9\n\nfor vp9, it is the value of stsw_vp9_error_ctu0_x"]
pub type SwStrmdErrorCtuXoffsetR = crate::FieldReader;
#[doc = "Field `SW_STRMD_ERROR_CTU_XOFFSET` writer - strmd error ctu xoffset\n\nstrmd error ctu xoffset\n\nfor all HEVC and H264 and VP9\n\nfor vp9, it is the value of stsw_vp9_error_ctu0_x"]
pub type SwStrmdErrorCtuXoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_STRMD_ERROR_CTU_YOFFSET` reader - strmd error ctu yoffset\n\nstrmd error ctu yoffset\n\nfor HEVC , H264 and VP9\n\nfor VP9, it is the value of stsw_vp9_error_ctu0_y"]
pub type SwStrmdErrorCtuYoffsetR = crate::FieldReader;
#[doc = "Field `SW_STRMD_ERROR_CTU_YOFFSET` writer - strmd error ctu yoffset\n\nstrmd error ctu yoffset\n\nfor HEVC , H264 and VP9\n\nfor VP9, it is the value of stsw_vp9_error_ctu0_y"]
pub type SwStrmdErrorCtuYoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_STREAMFIFO_SPACE2FULL` reader - stream fifo space to full\n\nIt is for debug use, to tell the stream fifo space to full\n\nfor HEVC , H264 and VP9"]
pub type SwStreamfifoSpace2fullR = crate::FieldReader;
#[doc = "Field `SW_STREAMFIFO_SPACE2FULL` writer - stream fifo space to full\n\nIt is for debug use, to tell the stream fifo space to full\n\nfor HEVC , H264 and VP9"]
pub type SwStreamfifoSpace2fullW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_VP9_ERROR_CTU0_EN` reader - to tell whether is any error of vp9\n\n1'b1: there is aleast a error in vp9 strmd\n\nnow ,is no for use"]
pub type SwVp9ErrorCtu0EnR = crate::BitReader;
#[doc = "Field `SW_VP9_ERROR_CTU0_EN` writer - to tell whether is any error of vp9\n\n1'b1: there is aleast a error in vp9 strmd\n\nnow ,is no for use"]
pub type SwVp9ErrorCtu0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - strmd error ctu xoffset\n\nstrmd error ctu xoffset\n\nfor all HEVC and H264 and VP9\n\nfor vp9, it is the value of stsw_vp9_error_ctu0_x"]
    #[inline(always)]
    pub fn sw_strmd_error_ctu_xoffset(&self) -> SwStrmdErrorCtuXoffsetR {
        SwStrmdErrorCtuXoffsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - strmd error ctu yoffset\n\nstrmd error ctu yoffset\n\nfor HEVC , H264 and VP9\n\nfor VP9, it is the value of stsw_vp9_error_ctu0_y"]
    #[inline(always)]
    pub fn sw_strmd_error_ctu_yoffset(&self) -> SwStrmdErrorCtuYoffsetR {
        SwStrmdErrorCtuYoffsetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - stream fifo space to full\n\nIt is for debug use, to tell the stream fifo space to full\n\nfor HEVC , H264 and VP9"]
    #[inline(always)]
    pub fn sw_streamfifo_space2full(&self) -> SwStreamfifoSpace2fullR {
        SwStreamfifoSpace2fullR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - to tell whether is any error of vp9\n\n1'b1: there is aleast a error in vp9 strmd\n\nnow ,is no for use"]
    #[inline(always)]
    pub fn sw_vp9_error_ctu0_en(&self) -> SwVp9ErrorCtu0EnR {
        SwVp9ErrorCtu0EnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - strmd error ctu xoffset\n\nstrmd error ctu xoffset\n\nfor all HEVC and H264 and VP9\n\nfor vp9, it is the value of stsw_vp9_error_ctu0_x"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strmd_error_ctu_xoffset(
        &mut self,
    ) -> SwStrmdErrorCtuXoffsetW<Swreg46StrmdErrorCtuSpec> {
        SwStrmdErrorCtuXoffsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - strmd error ctu yoffset\n\nstrmd error ctu yoffset\n\nfor HEVC , H264 and VP9\n\nfor VP9, it is the value of stsw_vp9_error_ctu0_y"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strmd_error_ctu_yoffset(
        &mut self,
    ) -> SwStrmdErrorCtuYoffsetW<Swreg46StrmdErrorCtuSpec> {
        SwStrmdErrorCtuYoffsetW::new(self, 8)
    }
    #[doc = "Bits 16:22 - stream fifo space to full\n\nIt is for debug use, to tell the stream fifo space to full\n\nfor HEVC , H264 and VP9"]
    #[inline(always)]
    #[must_use]
    pub fn sw_streamfifo_space2full(
        &mut self,
    ) -> SwStreamfifoSpace2fullW<Swreg46StrmdErrorCtuSpec> {
        SwStreamfifoSpace2fullW::new(self, 16)
    }
    #[doc = "Bit 24 - to tell whether is any error of vp9\n\n1'b1: there is aleast a error in vp9 strmd\n\nnow ,is no for use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_error_ctu0_en(&mut self) -> SwVp9ErrorCtu0EnW<Swreg46StrmdErrorCtuSpec> {
        SwVp9ErrorCtu0EnW::new(self, 24)
    }
}
#[doc = "strmd error ctu\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg46_strmd_error_ctu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg46_strmd_error_ctu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg46StrmdErrorCtuSpec;
impl crate::RegisterSpec for Swreg46StrmdErrorCtuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg46_strmd_error_ctu::R`](R) reader structure"]
impl crate::Readable for Swreg46StrmdErrorCtuSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg46_strmd_error_ctu::W`](W) writer structure"]
impl crate::Writable for Swreg46StrmdErrorCtuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG46_STRMD_ERROR_CTU to value 0x0040_0000"]
impl crate::Resettable for Swreg46StrmdErrorCtuSpec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
