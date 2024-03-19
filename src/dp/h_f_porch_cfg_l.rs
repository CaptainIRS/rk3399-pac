#[doc = "Register `H_F_PORCH_CFG_L` reader"]
pub type R = crate::R<HFPorchCfgLSpec>;
#[doc = "Register `H_F_PORCH_CFG_L` writer"]
pub type W = crate::W<HFPorchCfgLSpec>;
#[doc = "Field `H_F_PORCH_CFG_L` reader - H_F_PORCH_CFG is used to specify the number \n\nof pixels in frame horizon front porch part. This \n\nregister is H_F_PORCH_CFG\\[7:0\\]
\n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type HFPorchCfgLR = crate::FieldReader;
#[doc = "Field `H_F_PORCH_CFG_L` writer - H_F_PORCH_CFG is used to specify the number \n\nof pixels in frame horizon front porch part. This \n\nregister is H_F_PORCH_CFG\\[7:0\\]
\n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
pub type HFPorchCfgLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H_F_PORCH_CFG is used to specify the number \n\nof pixels in frame horizon front porch part. This \n\nregister is H_F_PORCH_CFG\\[7:0\\]
\n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    pub fn h_f_porch_cfg_l(&self) -> HFPorchCfgLR {
        HFPorchCfgLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H_F_PORCH_CFG is used to specify the number \n\nof pixels in frame horizon front porch part. This \n\nregister is H_F_PORCH_CFG\\[7:0\\]
\n\nWhen F_SEL is 1, this value is sent in main \n\nstream attribute packet. \n\nWhen BIST_EN is 1, this bit field is used to \n\nspecify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn h_f_porch_cfg_l(&mut self) -> HFPorchCfgLW<HFPorchCfgLSpec> {
        HFPorchCfgLW::new(self, 0)
    }
}
#[doc = "Horizon Front Porch Low Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_cfg_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_cfg_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPorchCfgLSpec;
impl crate::RegisterSpec for HFPorchCfgLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_f_porch_cfg_l::R`](R) reader structure"]
impl crate::Readable for HFPorchCfgLSpec {}
#[doc = "`write(|w| ..)` method takes [`h_f_porch_cfg_l::W`](W) writer structure"]
impl crate::Writable for HFPorchCfgLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets H_F_PORCH_CFG_L to value 0"]
impl crate::Resettable for HFPorchCfgLSpec {
    const RESET_VALUE: u32 = 0;
}
