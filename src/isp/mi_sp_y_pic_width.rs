#[doc = "Register `MI_SP_Y_PIC_WIDTH` reader"]
pub type R = crate::R<MiSpYPicWidthSpec>;
#[doc = "Register `MI_SP_Y_PIC_WIDTH` writer"]
pub type W = crate::W<MiSpYPicWidthSpec>;
#[doc = "Field `sp_y_pic_width` reader - Image width of the self picture Y component or RGB\n\npicture in pixel.\n\nFor YCbCr 4:2:x and RGB 565 the image width must\n\nbe a multiple of 2. If no line stride is used but flipping\n\nrequired, the image width must be a multiple of 8 for 4:2:x\n\nplanar or 4 for 4:4:4 planar/4:2:x semi planar. There are\n\nno restrictions for RGB 888/666.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used. Note: Image width always\n\nrefers to the picture width of the output image. This is\n\nparticularly important when rotating."]
pub type SpYPicWidthR = crate::FieldReader<u16>;
#[doc = "Field `sp_y_pic_width` writer - Image width of the self picture Y component or RGB\n\npicture in pixel.\n\nFor YCbCr 4:2:x and RGB 565 the image width must\n\nbe a multiple of 2. If no line stride is used but flipping\n\nrequired, the image width must be a multiple of 8 for 4:2:x\n\nplanar or 4 for 4:4:4 planar/4:2:x semi planar. There are\n\nno restrictions for RGB 888/666.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used. Note: Image width always\n\nrefers to the picture width of the output image. This is\n\nparticularly important when rotating."]
pub type SpYPicWidthW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Image width of the self picture Y component or RGB\n\npicture in pixel.\n\nFor YCbCr 4:2:x and RGB 565 the image width must\n\nbe a multiple of 2. If no line stride is used but flipping\n\nrequired, the image width must be a multiple of 8 for 4:2:x\n\nplanar or 4 for 4:4:4 planar/4:2:x semi planar. There are\n\nno restrictions for RGB 888/666.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used. Note: Image width always\n\nrefers to the picture width of the output image. This is\n\nparticularly important when rotating."]
    #[inline(always)]
    pub fn sp_y_pic_width(&self) -> SpYPicWidthR {
        SpYPicWidthR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Image width of the self picture Y component or RGB\n\npicture in pixel.\n\nFor YCbCr 4:2:x and RGB 565 the image width must\n\nbe a multiple of 2. If no line stride is used but flipping\n\nrequired, the image width must be a multiple of 8 for 4:2:x\n\nplanar or 4 for 4:4:4 planar/4:2:x semi planar. There are\n\nno restrictions for RGB 888/666.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used. Note: Image width always\n\nrefers to the picture width of the output image. This is\n\nparticularly important when rotating."]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_pic_width(&mut self) -> SpYPicWidthW<MiSpYPicWidthSpec> {
        SpYPicWidthW::new(self, 0)
    }
}
#[doc = "Y component image width\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture \n\ndata is sent to the self path. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_pic_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_pic_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYPicWidthSpec;
impl crate::RegisterSpec for MiSpYPicWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_pic_width::R`](R) reader structure"]
impl crate::Readable for MiSpYPicWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_y_pic_width::W`](W) writer structure"]
impl crate::Writable for MiSpYPicWidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_Y_PIC_WIDTH to value 0"]
impl crate::Resettable for MiSpYPicWidthSpec {
    const RESET_VALUE: u32 = 0;
}
