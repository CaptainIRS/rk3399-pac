#[doc = "Register `H_SYNC_CFG_L` reader"]
pub type R = crate::R<HSyncCfgLSpec>;
#[doc = "Register `H_SYNC_CFG_L` writer"]
pub type W = crate::W<HSyncCfgLSpec>;
#[doc = "Field `H_SYNC_CFG_L` reader - H_SYNC_CFG is used to specify the number of pixels in HSYNC period. This register is H_SYNC_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type HSyncCfgLR = crate::FieldReader;
#[doc = "Field `H_SYNC_CFG_L` writer - H_SYNC_CFG is used to specify the number of pixels in HSYNC period. This register is H_SYNC_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type HSyncCfgLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H_SYNC_CFG is used to specify the number of pixels in HSYNC period. This register is H_SYNC_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn h_sync_cfg_l(&self) -> HSyncCfgLR {
        HSyncCfgLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H_SYNC_CFG is used to specify the number of pixels in HSYNC period. This register is H_SYNC_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn h_sync_cfg_l(&mut self) -> HSyncCfgLW<HSyncCfgLSpec> {
        HSyncCfgLW::new(self, 0)
    }
}
#[doc = "Horizon Sync Width Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_sync_cfg_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_sync_cfg_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSyncCfgLSpec;
impl crate::RegisterSpec for HSyncCfgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_sync_cfg_l::R`](R) reader structure"]
impl crate::Readable for HSyncCfgLSpec {}
#[doc = "`write(|w| ..)` method takes [`h_sync_cfg_l::W`](W) writer structure"]
impl crate::Writable for HSyncCfgLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets H_SYNC_CFG_L to value 0"]
impl crate::Resettable for HSyncCfgLSpec {
    const RESET_VALUE: u32 = 0;
}
