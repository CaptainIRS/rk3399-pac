#[doc = "Register `TOTAL_PIXEL_CFG_L` reader"]
pub type R = crate::R<TotalPixelCfgLSpec>;
#[doc = "Register `TOTAL_PIXEL_CFG_L` writer"]
pub type W = crate::W<TotalPixelCfgLSpec>;
#[doc = "Field `TOTAL_PIXEL_CFG_L` reader - TOTAL_PIXEL_CFG is used to specify the number of pixels in each line. This register is TOTAL_PIXEL_CFG\\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type TotalPixelCfgLR = crate::FieldReader;
#[doc = "Field `TOTAL_PIXEL_CFG_L` writer - TOTAL_PIXEL_CFG is used to specify the number of pixels in each line. This register is TOTAL_PIXEL_CFG\\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type TotalPixelCfgLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TOTAL_PIXEL_CFG is used to specify the number of pixels in each line. This register is TOTAL_PIXEL_CFG\\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn total_pixel_cfg_l(&self) -> TotalPixelCfgLR {
        TotalPixelCfgLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TOTAL_PIXEL_CFG is used to specify the number of pixels in each line. This register is TOTAL_PIXEL_CFG\\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn total_pixel_cfg_l(&mut self) -> TotalPixelCfgLW<TotalPixelCfgLSpec> {
        TotalPixelCfgLW::new(self, 0)
    }
}
#[doc = "Total Pixel Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_cfg_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_cfg_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalPixelCfgLSpec;
impl crate::RegisterSpec for TotalPixelCfgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_pixel_cfg_l::R`](R) reader structure"]
impl crate::Readable for TotalPixelCfgLSpec {}
#[doc = "`write(|w| ..)` method takes [`total_pixel_cfg_l::W`](W) writer structure"]
impl crate::Writable for TotalPixelCfgLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets TOTAL_PIXEL_CFG_L to value 0"]
impl crate::Resettable for TotalPixelCfgLSpec {
    const RESET_VALUE: u32 = 0;
}
