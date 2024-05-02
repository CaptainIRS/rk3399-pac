#[doc = "Register `MI_SP_Y_PIC_SIZE` reader"]
pub type R = crate::R<MiSpYPicSizeSpec>;
#[doc = "Register `MI_SP_Y_PIC_SIZE` writer"]
pub type W = crate::W<MiSpYPicSizeSpec>;
#[doc = "Field `sp_y_pic_size` reader - Image size of the Y component or RGB picture in pixel\n\nwhich has to be the Y line length multiplied by the Y image\n\nheight (sp_y_llength * sp_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb and Cr\n\ncomponent is assumed half for 4:2:0 and the same size for\n\n4:2:2."]
pub type SpYPicSizeR = crate::FieldReader<u32>;
#[doc = "Field `sp_y_pic_size` writer - Image size of the Y component or RGB picture in pixel\n\nwhich has to be the Y line length multiplied by the Y image\n\nheight (sp_y_llength * sp_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb and Cr\n\ncomponent is assumed half for 4:2:0 and the same size for\n\n4:2:2."]
pub type SpYPicSizeW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Image size of the Y component or RGB picture in pixel\n\nwhich has to be the Y line length multiplied by the Y image\n\nheight (sp_y_llength * sp_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb and Cr\n\ncomponent is assumed half for 4:2:0 and the same size for\n\n4:2:2."]
    #[inline(always)]
    pub fn sp_y_pic_size(&self) -> SpYPicSizeR {
        SpYPicSizeR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Image size of the Y component or RGB picture in pixel\n\nwhich has to be the Y line length multiplied by the Y image\n\nheight (sp_y_llength * sp_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb and Cr\n\ncomponent is assumed half for 4:2:0 and the same size for\n\n4:2:2."]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_pic_size(&mut self) -> SpYPicSizeW<MiSpYPicSizeSpec> {
        SpYPicSizeW::new(self, 0)
    }
}
#[doc = "Y component image size\n\nNote: Programmed value becomes effective \n\n\n\nimmediately. So write to the register only if no picture \n\n\n\ndata is sent to the self path. \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_pic_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_pic_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYPicSizeSpec;
impl crate::RegisterSpec for MiSpYPicSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_pic_size::R`](R) reader structure"]
impl crate::Readable for MiSpYPicSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_y_pic_size::W`](W) writer structure"]
impl crate::Writable for MiSpYPicSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_Y_PIC_SIZE to value 0"]
impl crate::Resettable for MiSpYPicSizeSpec {
    const RESET_VALUE: u32 = 0;
}
