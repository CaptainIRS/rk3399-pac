#[doc = "Register `H_B_PORCH_CFG_L` reader"]
pub type R = crate::R<HBPorchCfgLSpec>;
#[doc = "Register `H_B_PORCH_CFG_L` writer"]
pub type W = crate::W<HBPorchCfgLSpec>;
#[doc = "Field `H_B_PORCH_CFG_L` reader - H_B_PORCH_CFG is used to specify the number of pixel in frame horizon back porch part. This register is H_B_PORCH_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type HBPorchCfgLR = crate::FieldReader;
#[doc = "Field `H_B_PORCH_CFG_L` writer - H_B_PORCH_CFG is used to specify the number of pixel in frame horizon back porch part. This register is H_B_PORCH_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type HBPorchCfgLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H_B_PORCH_CFG is used to specify the number of pixel in frame horizon back porch part. This register is H_B_PORCH_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn h_b_porch_cfg_l(&self) -> HBPorchCfgLR {
        HBPorchCfgLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H_B_PORCH_CFG is used to specify the number of pixel in frame horizon back porch part. This register is H_B_PORCH_CFG \\[7:0\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn h_b_porch_cfg_l(&mut self) -> HBPorchCfgLW<HBPorchCfgLSpec> {
        HBPorchCfgLW::new(self, 0)
    }
}
#[doc = "Horizon Back Porch Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_cfg_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_cfg_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBPorchCfgLSpec;
impl crate::RegisterSpec for HBPorchCfgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_b_porch_cfg_l::R`](R) reader structure"]
impl crate::Readable for HBPorchCfgLSpec {}
#[doc = "`write(|w| ..)` method takes [`h_b_porch_cfg_l::W`](W) writer structure"]
impl crate::Writable for HBPorchCfgLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets H_B_PORCH_CFG_L to value 0"]
impl crate::Resettable for HBPorchCfgLSpec {
    const RESET_VALUE: u32 = 0;
}
