#[doc = "Register `V_B_PORCH_CFG` reader"]
pub type R = crate::R<VBPorchCfgSpec>;
#[doc = "Register `V_B_PORCH_CFG` writer"]
pub type W = crate::W<VBPorchCfgSpec>;
#[doc = "Field `V_B_PORCH_CFG` reader - This is used to specify the number of lines in frame \n\nback porch part. \n\nWhen F_SEL is 1, this value is sent in main stream \n\nattribute packet. \n\nWhen BIST_EN is 1, this bit field is used to specify \n\nthe BIST video stream format."]
pub type VBPorchCfgR = crate::FieldReader;
#[doc = "Field `V_B_PORCH_CFG` writer - This is used to specify the number of lines in frame \n\nback porch part. \n\nWhen F_SEL is 1, this value is sent in main stream \n\nattribute packet. \n\nWhen BIST_EN is 1, this bit field is used to specify \n\nthe BIST video stream format."]
pub type VBPorchCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This is used to specify the number of lines in frame \n\nback porch part. \n\nWhen F_SEL is 1, this value is sent in main stream \n\nattribute packet. \n\nWhen BIST_EN is 1, this bit field is used to specify \n\nthe BIST video stream format."]
    #[inline(always)]
    pub fn v_b_porch_cfg(&self) -> VBPorchCfgR {
        VBPorchCfgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is used to specify the number of lines in frame \n\nback porch part. \n\nWhen F_SEL is 1, this value is sent in main stream \n\nattribute packet. \n\nWhen BIST_EN is 1, this bit field is used to specify \n\nthe BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn v_b_porch_cfg(&mut self) -> VBPorchCfgW<VBPorchCfgSpec> {
        VBPorchCfgW::new(self, 0)
    }
}
#[doc = "Vertical Back Porch Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_b_porch_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_b_porch_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBPorchCfgSpec;
impl crate::RegisterSpec for VBPorchCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`v_b_porch_cfg::R`](R) reader structure"]
impl crate::Readable for VBPorchCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`v_b_porch_cfg::W`](W) writer structure"]
impl crate::Writable for VBPorchCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets V_B_PORCH_CFG to value 0"]
impl crate::Resettable for VBPorchCfgSpec {
    const RESET_VALUE: u32 = 0;
}
