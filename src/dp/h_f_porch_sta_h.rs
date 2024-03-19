#[doc = "Register `H_F_PORCH_STA_H` reader"]
pub type R = crate::R<HFPorchStaHSpec>;
#[doc = "Register `H_F_PORCH_STA_H` writer"]
pub type W = crate::W<HFPorchStaHSpec>;
#[doc = "Field `H_F_PORCH_STA_H` reader - H_F_PORCH \\[11:8\\]
(horizon front porch) \n\nwhich is detected by video capture module. \n\nThis bit field is valid only when \n\nSTRM_VALID is high. And STRM_VALID \n\nbecomes high when two successive frames \n\nare determined as stable."]
pub type HFPorchStaHR = crate::FieldReader;
#[doc = "Field `H_F_PORCH_STA_H` writer - H_F_PORCH \\[11:8\\]
(horizon front porch) \n\nwhich is detected by video capture module. \n\nThis bit field is valid only when \n\nSTRM_VALID is high. And STRM_VALID \n\nbecomes high when two successive frames \n\nare determined as stable."]
pub type HFPorchStaHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - H_F_PORCH \\[11:8\\]
(horizon front porch) \n\nwhich is detected by video capture module. \n\nThis bit field is valid only when \n\nSTRM_VALID is high. And STRM_VALID \n\nbecomes high when two successive frames \n\nare determined as stable."]
    #[inline(always)]
    pub fn h_f_porch_sta_h(&self) -> HFPorchStaHR {
        HFPorchStaHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - H_F_PORCH \\[11:8\\]
(horizon front porch) \n\nwhich is detected by video capture module. \n\nThis bit field is valid only when \n\nSTRM_VALID is high. And STRM_VALID \n\nbecomes high when two successive frames \n\nare determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn h_f_porch_sta_h(&mut self) -> HFPorchStaHW<HFPorchStaHSpec> {
        HFPorchStaHW::new(self, 0)
    }
}
#[doc = "Horizon Front Porch Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPorchStaHSpec;
impl crate::RegisterSpec for HFPorchStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_f_porch_sta_h::R`](R) reader structure"]
impl crate::Readable for HFPorchStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`h_f_porch_sta_h::W`](W) writer structure"]
impl crate::Writable for HFPorchStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets H_F_PORCH_STA_H to value 0"]
impl crate::Resettable for HFPorchStaHSpec {
    const RESET_VALUE: u32 = 0;
}
