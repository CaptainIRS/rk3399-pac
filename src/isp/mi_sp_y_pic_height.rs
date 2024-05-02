#[doc = "Register `MI_SP_Y_PIC_HEIGHT` reader"]
pub type R = crate::R<MiSpYPicHeightSpec>;
#[doc = "Register `MI_SP_Y_PIC_HEIGHT` writer"]
pub type W = crate::W<MiSpYPicHeightSpec>;
#[doc = "Field `sp_y_pic_height` reader - Image height of the y component or RGB picture in\n\npixel.\n\nIn planar and semi planar mode the image width of\n\nthe cb and cr component is assumed according to the\n\nYCbCr format, i.e. half for 4:2:0 and the same for 4:2:2\n\nand 4:4:4.\n\nNote: Image height always refers to the picture\n\nheight of the output image. This is particularly\n\nimportant when rotating."]
pub type SpYPicHeightR = crate::FieldReader<u16>;
#[doc = "Field `sp_y_pic_height` writer - Image height of the y component or RGB picture in\n\npixel.\n\nIn planar and semi planar mode the image width of\n\nthe cb and cr component is assumed according to the\n\nYCbCr format, i.e. half for 4:2:0 and the same for 4:2:2\n\nand 4:4:4.\n\nNote: Image height always refers to the picture\n\nheight of the output image. This is particularly\n\nimportant when rotating."]
pub type SpYPicHeightW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Image height of the y component or RGB picture in\n\npixel.\n\nIn planar and semi planar mode the image width of\n\nthe cb and cr component is assumed according to the\n\nYCbCr format, i.e. half for 4:2:0 and the same for 4:2:2\n\nand 4:4:4.\n\nNote: Image height always refers to the picture\n\nheight of the output image. This is particularly\n\nimportant when rotating."]
    #[inline(always)]
    pub fn sp_y_pic_height(&self) -> SpYPicHeightR {
        SpYPicHeightR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Image height of the y component or RGB picture in\n\npixel.\n\nIn planar and semi planar mode the image width of\n\nthe cb and cr component is assumed according to the\n\nYCbCr format, i.e. half for 4:2:0 and the same for 4:2:2\n\nand 4:4:4.\n\nNote: Image height always refers to the picture\n\nheight of the output image. This is particularly\n\nimportant when rotating."]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_pic_height(&mut self) -> SpYPicHeightW<MiSpYPicHeightSpec> {
        SpYPicHeightW::new(self, 0)
    }
}
#[doc = "Y component image height\n\nNote: Programmed value becomes effective \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_pic_height::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_pic_height::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYPicHeightSpec;
impl crate::RegisterSpec for MiSpYPicHeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_pic_height::R`](R) reader structure"]
impl crate::Readable for MiSpYPicHeightSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_y_pic_height::W`](W) writer structure"]
impl crate::Writable for MiSpYPicHeightSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_Y_PIC_HEIGHT to value 0"]
impl crate::Resettable for MiSpYPicHeightSpec {
    const RESET_VALUE: u32 = 0;
}
