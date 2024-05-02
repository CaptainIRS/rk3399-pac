#[doc = "Register `GAMMA_OUT_Y%s` reader"]
pub type R = crate::R<GammaOutYSpec>;
#[doc = "Register `GAMMA_OUT_Y%s` writer"]
pub type W = crate::W<GammaOutYSpec>;
#[doc = "Field `isp_gamma_out_y` reader - Gamma_out curve point definition y-axis (output) for\n\nall color components (red,green,blue)\n\nRESTRICTION: The difference between two Y_n\n\n(dy = Y_n - Y_n-1) is restricted to +511/-512 (10 bit\n\nsigned)!"]
pub type IspGammaOutYR = crate::FieldReader<u16>;
#[doc = "Field `isp_gamma_out_y` writer - Gamma_out curve point definition y-axis (output) for\n\nall color components (red,green,blue)\n\nRESTRICTION: The difference between two Y_n\n\n(dy = Y_n - Y_n-1) is restricted to +511/-512 (10 bit\n\nsigned)!"]
pub type IspGammaOutYW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Gamma_out curve point definition y-axis (output) for\n\nall color components (red,green,blue)\n\nRESTRICTION: The difference between two Y_n\n\n(dy = Y_n - Y_n-1) is restricted to +511/-512 (10 bit\n\nsigned)!"]
    #[inline(always)]
    pub fn isp_gamma_out_y(&self) -> IspGammaOutYR {
        IspGammaOutYR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Gamma_out curve point definition y-axis (output) for\n\nall color components (red,green,blue)\n\nRESTRICTION: The difference between two Y_n\n\n(dy = Y_n - Y_n-1) is restricted to +511/-512 (10 bit\n\nsigned)!"]
    #[inline(always)]
    #[must_use]
    pub fn isp_gamma_out_y(&mut self) -> IspGammaOutYW<GammaOutYSpec> {
        IspGammaOutYW::new(self, 0)
    }
}
#[doc = "Gamma Out Curve definition y_ n (n=0..16)\n\nNote: Reset values generate a standard gamma of 2.2. Reset values are: \n\ny_00 = 0x000, y_01 = 0x049, y_02 = 0x089, y_03 = 0x0B7, y_04 = 0x0DF, y_05 = \n\n\n\n0x11F, y_06 = 0x154, y_07 = 0x183, y_08 = 0x1AD, y_09 = 0x1F6, y_10 = 0x235, y_11 = \n\n0x26F, y_12 = 0x2D3, y_13 = 0x32A, y_14 = 0x378, y_15 = 0x3BF, y_16 = 0x3FF \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gamma_out_y::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_out_y::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaOutYSpec;
impl crate::RegisterSpec for GammaOutYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_out_y::R`](R) reader structure"]
impl crate::Readable for GammaOutYSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_out_y::W`](W) writer structure"]
impl crate::Writable for GammaOutYSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAMMA_OUT_Y%s to value 0"]
impl crate::Resettable for GammaOutYSpec {
    const RESET_VALUE: u32 = 0;
}
