#[doc = "Register `ACTIVE_PIXEL_CFG_H` reader"]
pub type R = crate::R<ActivePixelCfgHSpec>;
#[doc = "Register `ACTIVE_PIXEL_CFG_H` writer"]
pub type W = crate::W<ActivePixelCfgHSpec>;
#[doc = "Field `ACTIVE_PIXEL_CFG_H` reader - ACTIVE_PIXEL_CFG is used to specify the number of active pixels in each line. This register is ACTIVE_PIXEL_CFG \\[13:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type ActivePixelCfgHR = crate::FieldReader;
#[doc = "Field `ACTIVE_PIXEL_CFG_H` writer - ACTIVE_PIXEL_CFG is used to specify the number of active pixels in each line. This register is ACTIVE_PIXEL_CFG \\[13:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type ActivePixelCfgHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - ACTIVE_PIXEL_CFG is used to specify the number of active pixels in each line. This register is ACTIVE_PIXEL_CFG \\[13:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn active_pixel_cfg_h(&self) -> ActivePixelCfgHR {
        ActivePixelCfgHR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ACTIVE_PIXEL_CFG is used to specify the number of active pixels in each line. This register is ACTIVE_PIXEL_CFG \\[13:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn active_pixel_cfg_h(&mut self) -> ActivePixelCfgHW<ActivePixelCfgHSpec> {
        ActivePixelCfgHW::new(self, 0)
    }
}
#[doc = "Active Pixel High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_cfg_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_cfg_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActivePixelCfgHSpec;
impl crate::RegisterSpec for ActivePixelCfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_pixel_cfg_h::R`](R) reader structure"]
impl crate::Readable for ActivePixelCfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`active_pixel_cfg_h::W`](W) writer structure"]
impl crate::Writable for ActivePixelCfgHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets ACTIVE_PIXEL_CFG_H to value 0"]
impl crate::Resettable for ActivePixelCfgHSpec {
    const RESET_VALUE: u32 = 0;
}
