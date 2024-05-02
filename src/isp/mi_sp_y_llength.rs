#[doc = "Register `MI_SP_Y_LLENGTH` reader"]
pub type R = crate::R<MiSpYLlengthSpec>;
#[doc = "Register `MI_SP_Y_LLENGTH` writer"]
pub type W = crate::W<MiSpYLlengthSpec>;
#[doc = "Field `sp_y_llength` reader - or RGB picture\n\nin pixel, also known as line stride.\n\nIf no line stride is used, line length must match image\n\nwidth.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb and Cr component is\n\nassumed the same size. Note: Line length always refers to\n\nthe line length of the output image. This is particularly\n\nimportant when rotating."]
pub type SpYLlengthR = crate::FieldReader<u16>;
#[doc = "Field `sp_y_llength` writer - or RGB picture\n\nin pixel, also known as line stride.\n\nIf no line stride is used, line length must match image\n\nwidth.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb and Cr component is\n\nassumed the same size. Note: Line length always refers to\n\nthe line length of the output image. This is particularly\n\nimportant when rotating."]
pub type SpYLlengthW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - or RGB picture\n\nin pixel, also known as line stride.\n\nIf no line stride is used, line length must match image\n\nwidth.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb and Cr component is\n\nassumed the same size. Note: Line length always refers to\n\nthe line length of the output image. This is particularly\n\nimportant when rotating."]
    #[inline(always)]
    pub fn sp_y_llength(&self) -> SpYLlengthR {
        SpYLlengthR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - or RGB picture\n\nin pixel, also known as line stride.\n\nIf no line stride is used, line length must match image\n\nwidth.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4 and for RGB 565 a multiple of 2. There are no\n\nrestrictions for RGB 888/666.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb and Cr component is\n\nassumed the same size. Note: Line length always refers to\n\nthe line length of the output image. This is particularly\n\nimportant when rotating."]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_llength(&mut self) -> SpYLlengthW<MiSpYLlengthSpec> {
        SpYLlengthW::new(self, 0)
    }
}
#[doc = "Line length of self picture Y component\n\nNote: Programmed value becomes effective \n\nimmediately. So write to the register only if no picture data \n\nis sent to the self path. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_llength::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_llength::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYLlengthSpec;
impl crate::RegisterSpec for MiSpYLlengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_llength::R`](R) reader structure"]
impl crate::Readable for MiSpYLlengthSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_y_llength::W`](W) writer structure"]
impl crate::Writable for MiSpYLlengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_Y_LLENGTH to value 0"]
impl crate::Resettable for MiSpYLlengthSpec {
    const RESET_VALUE: u32 = 0;
}
