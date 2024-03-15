#[doc = "Register `H_B_PORCH_STA_H` reader"]
pub type R = crate::R<HBPorchStaHSpec>;
#[doc = "Register `H_B_PORCH_STA_H` writer"]
pub type W = crate::W<HBPorchStaHSpec>;
#[doc = "Field `H_B_PORCH_STA_H` reader - H_B_PORCH \\[11:8\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type HBPorchStaHR = crate::FieldReader;
#[doc = "Field `H_B_PORCH_STA_H` writer - H_B_PORCH \\[11:8\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type HBPorchStaHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - H_B_PORCH \\[11:8\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn h_b_porch_sta_h(&self) -> HBPorchStaHR {
        HBPorchStaHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - H_B_PORCH \\[11:8\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn h_b_porch_sta_h(&mut self) -> HBPorchStaHW<HBPorchStaHSpec> {
        HBPorchStaHW::new(self, 0)
    }
}
#[doc = "Horizon Back Porch Status High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_sta_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_sta_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBPorchStaHSpec;
impl crate::RegisterSpec for HBPorchStaHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_b_porch_sta_h::R`](R) reader structure"]
impl crate::Readable for HBPorchStaHSpec {}
#[doc = "`write(|w| ..)` method takes [`h_b_porch_sta_h::W`](W) writer structure"]
impl crate::Writable for HBPorchStaHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets H_B_PORCH_STA_H to value 0"]
impl crate::Resettable for HBPorchStaHSpec {
    const RESET_VALUE: u32 = 0;
}
