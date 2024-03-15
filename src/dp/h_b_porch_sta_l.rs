#[doc = "Register `H_B_PORCH_STA_L` reader"]
pub type R = crate::R<HBPorchStaLSpec>;
#[doc = "Register `H_B_PORCH_STA_L` writer"]
pub type W = crate::W<HBPorchStaLSpec>;
#[doc = "Field `H_B_PORCH_STA_L` reader - H_B_PORCH \\[7:0\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type HBPorchStaLR = crate::FieldReader;
#[doc = "Field `H_B_PORCH_STA_L` writer - H_B_PORCH \\[7:0\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
pub type HBPorchStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H_B_PORCH \\[7:0\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn h_b_porch_sta_l(&self) -> HBPorchStaLR {
        HBPorchStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H_B_PORCH \\[7:0\\]
(horizon back porch) which is detected by video capture module. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn h_b_porch_sta_l(&mut self) -> HBPorchStaLW<HBPorchStaLSpec> {
        HBPorchStaLW::new(self, 0)
    }
}
#[doc = "Horizon Back Porch Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_b_porch_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_b_porch_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBPorchStaLSpec;
impl crate::RegisterSpec for HBPorchStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_b_porch_sta_l::R`](R) reader structure"]
impl crate::Readable for HBPorchStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`h_b_porch_sta_l::W`](W) writer structure"]
impl crate::Writable for HBPorchStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets H_B_PORCH_STA_L to value 0"]
impl crate::Resettable for HBPorchStaLSpec {
    const RESET_VALUE: u32 = 0;
}
