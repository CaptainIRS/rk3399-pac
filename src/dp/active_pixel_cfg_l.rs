#[doc = "Register `ACTIVE_PIXEL_CFG_L` reader"]
pub type R = crate::R<ActivePixelCfgLSpec>;
#[doc = "Register `ACTIVE_PIXEL_CFG_L` writer"]
pub type W = crate::W<ActivePixelCfgLSpec>;
#[doc = "Field `ACTIVE_PIXEL_CFG_L` reader - ACTIVE_PIXEL_CFG is used to specify the \n\nnumber of active pixels in each line. This \n\nregister is ACTIVE_PIXEL_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type ActivePixelCfgLR = crate::FieldReader;
#[doc = "Field `ACTIVE_PIXEL_CFG_L` writer - ACTIVE_PIXEL_CFG is used to specify the \n\nnumber of active pixels in each line. This \n\nregister is ACTIVE_PIXEL_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type ActivePixelCfgLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ACTIVE_PIXEL_CFG is used to specify the \n\nnumber of active pixels in each line. This \n\nregister is ACTIVE_PIXEL_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    pub fn active_pixel_cfg_l(&self) -> ActivePixelCfgLR {
        ActivePixelCfgLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ACTIVE_PIXEL_CFG is used to specify the \n\nnumber of active pixels in each line. This \n\nregister is ACTIVE_PIXEL_CFG \\[7:0\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn active_pixel_cfg_l(&mut self) -> ActivePixelCfgLW<ActivePixelCfgLSpec> {
        ActivePixelCfgLW::new(self, 0)
    }
}
#[doc = "Active Pixel Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_cfg_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_cfg_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActivePixelCfgLSpec;
impl crate::RegisterSpec for ActivePixelCfgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_pixel_cfg_l::R`](R) reader structure"]
impl crate::Readable for ActivePixelCfgLSpec {}
#[doc = "`write(|w| ..)` method takes [`active_pixel_cfg_l::W`](W) writer structure"]
impl crate::Writable for ActivePixelCfgLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ACTIVE_PIXEL_CFG_L to value 0"]
impl crate::Resettable for ActivePixelCfgLSpec {
    const RESET_VALUE: u32 = 0;
}
