#[doc = "Register `TOTAL_PIXEL_CFG_H` reader"]
pub type R = crate::R<TotalPixelCfgHSpec>;
#[doc = "Register `TOTAL_PIXEL_CFG_H` writer"]
pub type W = crate::W<TotalPixelCfgHSpec>;
#[doc = "Field `TOTAL_PIXEL_CFG_H` reader - TOTAL_PIXEL_CFG is used to specify the \n\nnumber of pixels in each line. This register is \n\nTOTAL_PIXEL_CFG \\[13:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type TotalPixelCfgHR = crate::FieldReader;
#[doc = "Field `TOTAL_PIXEL_CFG_H` writer - TOTAL_PIXEL_CFG is used to specify the \n\nnumber of pixels in each line. This register is \n\nTOTAL_PIXEL_CFG \\[13:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type TotalPixelCfgHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - TOTAL_PIXEL_CFG is used to specify the \n\nnumber of pixels in each line. This register is \n\nTOTAL_PIXEL_CFG \\[13:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    pub fn total_pixel_cfg_h(&self) -> TotalPixelCfgHR {
        TotalPixelCfgHR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TOTAL_PIXEL_CFG is used to specify the \n\nnumber of pixels in each line. This register is \n\nTOTAL_PIXEL_CFG \\[13:8\\]. \n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn total_pixel_cfg_h(&mut self) -> TotalPixelCfgHW<TotalPixelCfgHSpec> {
        TotalPixelCfgHW::new(self, 0)
    }
}
#[doc = "Total Pixel High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_pixel_cfg_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_pixel_cfg_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalPixelCfgHSpec;
impl crate::RegisterSpec for TotalPixelCfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_pixel_cfg_h::R`](R) reader structure"]
impl crate::Readable for TotalPixelCfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`total_pixel_cfg_h::W`](W) writer structure"]
impl crate::Writable for TotalPixelCfgHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets TOTAL_PIXEL_CFG_H to value 0"]
impl crate::Resettable for TotalPixelCfgHSpec {
    const RESET_VALUE: u32 = 0;
}
