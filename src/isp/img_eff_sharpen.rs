#[doc = "Register `IMG_EFF_SHARPEN` reader"]
pub type R = crate::R<ImgEffSharpenSpec>;
#[doc = "Register `IMG_EFF_SHARPEN` writer"]
pub type W = crate::W<ImgEffSharpenSpec>;
#[doc = "Field `coring_thr` reader - Threshold for coring function. This value is used to\n\navoid amplifying noise too much by suppressing\n\nsharpening for small gradients. Higher value means less\n\nsharpening for smooth edges. Threshold zero means no\n\ncoring, so all gradients are treated the same. Threshold\n\n255 means nearly no sharpening. An absolute value for\n\nthe highpass signal is defined here. The highpass signal is\n\ntruncated at the defined level."]
pub type CoringThrR = crate::FieldReader;
#[doc = "Field `coring_thr` writer - Threshold for coring function. This value is used to\n\navoid amplifying noise too much by suppressing\n\nsharpening for small gradients. Higher value means less\n\nsharpening for smooth edges. Threshold zero means no\n\ncoring, so all gradients are treated the same. Threshold\n\n255 means nearly no sharpening. An absolute value for\n\nthe highpass signal is defined here. The highpass signal is\n\ntruncated at the defined level."]
pub type CoringThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sharp_factor` reader - 6Bit Factor for sharpening function. Value range is\n\nfrom 0x0 to 0x3F. High value means strong sharpening.\n\nThe resulting factors are for example:\n\n0x00 = 0 (no sharpen effect like bypass) 0x01 = 0.25\n\n0x02 = 0.5\n\n0x03 = 0.75\n\n0x04 = 1.0\n\n0x05 = 1.25\n\n0x06 = 1.5\n\n0x08 = 2.0\n\n0x0A = 2.5\n\n0x0C = 3.0\n\n0x10 = 4.0\n\n0x18 = 6.0\n\n0x20 = 8.0\n\n0x30 = 12.0\n\n0x3F = 15.75"]
pub type SharpFactorR = crate::FieldReader;
#[doc = "Field `sharp_factor` writer - 6Bit Factor for sharpening function. Value range is\n\nfrom 0x0 to 0x3F. High value means strong sharpening.\n\nThe resulting factors are for example:\n\n0x00 = 0 (no sharpen effect like bypass) 0x01 = 0.25\n\n0x02 = 0.5\n\n0x03 = 0.75\n\n0x04 = 1.0\n\n0x05 = 1.25\n\n0x06 = 1.5\n\n0x08 = 2.0\n\n0x0A = 2.5\n\n0x0C = 3.0\n\n0x10 = 4.0\n\n0x18 = 6.0\n\n0x20 = 8.0\n\n0x30 = 12.0\n\n0x3F = 15.75"]
pub type SharpFactorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - Threshold for coring function. This value is used to\n\navoid amplifying noise too much by suppressing\n\nsharpening for small gradients. Higher value means less\n\nsharpening for smooth edges. Threshold zero means no\n\ncoring, so all gradients are treated the same. Threshold\n\n255 means nearly no sharpening. An absolute value for\n\nthe highpass signal is defined here. The highpass signal is\n\ntruncated at the defined level."]
    #[inline(always)]
    pub fn coring_thr(&self) -> CoringThrR {
        CoringThrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 6Bit Factor for sharpening function. Value range is\n\nfrom 0x0 to 0x3F. High value means strong sharpening.\n\nThe resulting factors are for example:\n\n0x00 = 0 (no sharpen effect like bypass) 0x01 = 0.25\n\n0x02 = 0.5\n\n0x03 = 0.75\n\n0x04 = 1.0\n\n0x05 = 1.25\n\n0x06 = 1.5\n\n0x08 = 2.0\n\n0x0A = 2.5\n\n0x0C = 3.0\n\n0x10 = 4.0\n\n0x18 = 6.0\n\n0x20 = 8.0\n\n0x30 = 12.0\n\n0x3F = 15.75"]
    #[inline(always)]
    pub fn sharp_factor(&self) -> SharpFactorR {
        SharpFactorR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for coring function. This value is used to\n\navoid amplifying noise too much by suppressing\n\nsharpening for small gradients. Higher value means less\n\nsharpening for smooth edges. Threshold zero means no\n\ncoring, so all gradients are treated the same. Threshold\n\n255 means nearly no sharpening. An absolute value for\n\nthe highpass signal is defined here. The highpass signal is\n\ntruncated at the defined level."]
    #[inline(always)]
    #[must_use]
    pub fn coring_thr(&mut self) -> CoringThrW<ImgEffSharpenSpec> {
        CoringThrW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 6Bit Factor for sharpening function. Value range is\n\nfrom 0x0 to 0x3F. High value means strong sharpening.\n\nThe resulting factors are for example:\n\n0x00 = 0 (no sharpen effect like bypass) 0x01 = 0.25\n\n0x02 = 0.5\n\n0x03 = 0.75\n\n0x04 = 1.0\n\n0x05 = 1.25\n\n0x06 = 1.5\n\n0x08 = 2.0\n\n0x0A = 2.5\n\n0x0C = 3.0\n\n0x10 = 4.0\n\n0x18 = 6.0\n\n0x20 = 8.0\n\n0x30 = 12.0\n\n0x3F = 15.75"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_factor(&mut self) -> SharpFactorW<ImgEffSharpenSpec> {
        SharpFactorW::new(self, 8)
    }
}
#[doc = "Factor and threshold for sharpen effect\n\nNote: For the sharpening effect the convolution mask must be set to the values \\[-1 -1 -1; \n\n-1 8 -1; -1 -1 -1\\]. \n\n\n\nThe convolution mask for sharpening is defined by the values sket_coef_xx in registers \n\nIMG_EFF_MAT_3 through IMG_EFF_MAT_5. Sketch effect and sharpening effect share the \n\nsame mask registers. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_sharpen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_sharpen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffSharpenSpec;
impl crate::RegisterSpec for ImgEffSharpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_sharpen::R`](R) reader structure"]
impl crate::Readable for ImgEffSharpenSpec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_sharpen::W`](W) writer structure"]
impl crate::Writable for ImgEffSharpenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_SHARPEN to value 0"]
impl crate::Resettable for ImgEffSharpenSpec {
    const RESET_VALUE: u32 = 0;
}
