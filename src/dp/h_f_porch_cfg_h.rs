#[doc = "Register `H_F_PORCH_CFG_H` reader"]
pub type R = crate::R<HFPorchCfgHSpec>;
#[doc = "Register `H_F_PORCH_CFG_H` writer"]
pub type W = crate::W<HFPorchCfgHSpec>;
#[doc = "Field `H_F_PORCH_CFG_H` reader - H_F_PORCH_CFG is used to specify the number of pixels in frame horizon front porch part. This register is H_F_PORCH_CFG \\[11:8\\]
When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type HFPorchCfgHR = crate::FieldReader;
#[doc = "Field `H_F_PORCH_CFG_H` writer - H_F_PORCH_CFG is used to specify the number of pixels in frame horizon front porch part. This register is H_F_PORCH_CFG \\[11:8\\]
When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type HFPorchCfgHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - H_F_PORCH_CFG is used to specify the number of pixels in frame horizon front porch part. This register is H_F_PORCH_CFG \\[11:8\\]
When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn h_f_porch_cfg_h(&self) -> HFPorchCfgHR {
        HFPorchCfgHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - H_F_PORCH_CFG is used to specify the number of pixels in frame horizon front porch part. This register is H_F_PORCH_CFG \\[11:8\\]
When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn h_f_porch_cfg_h(&mut self) -> HFPorchCfgHW<HFPorchCfgHSpec> {
        HFPorchCfgHW::new(self, 0)
    }
}
#[doc = "Horizon Front Porch High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_cfg_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_cfg_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPorchCfgHSpec;
impl crate::RegisterSpec for HFPorchCfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_f_porch_cfg_h::R`](R) reader structure"]
impl crate::Readable for HFPorchCfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`h_f_porch_cfg_h::W`](W) writer structure"]
impl crate::Writable for HFPorchCfgHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets H_F_PORCH_CFG_H to value 0"]
impl crate::Resettable for HFPorchCfgHSpec {
    const RESET_VALUE: u32 = 0;
}
