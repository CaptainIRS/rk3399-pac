#[doc = "Register `H_F_PORCH_STA_L` reader"]
pub type R = crate::R<HFPorchStaLSpec>;
#[doc = "Register `H_F_PORCH_STA_L` writer"]
pub type W = crate::W<HFPorchStaLSpec>;
#[doc = "Field `H_F_PORCH_STA_L` reader - H_F_PORCH \\[7:0\\]
(horizon front porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type HFPorchStaLR = crate::FieldReader;
#[doc = "Field `H_F_PORCH_STA_L` writer - H_F_PORCH \\[7:0\\]
(horizon front porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type HFPorchStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H_F_PORCH \\[7:0\\]
(horizon front porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    pub fn h_f_porch_sta_l(&self) -> HFPorchStaLR {
        HFPorchStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H_F_PORCH \\[7:0\\]
(horizon front porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn h_f_porch_sta_l(&mut self) -> HFPorchStaLW<HFPorchStaLSpec> {
        HFPorchStaLW::new(self, 0)
    }
}
#[doc = "Horizon Front Porch Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_f_porch_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_f_porch_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPorchStaLSpec;
impl crate::RegisterSpec for HFPorchStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_f_porch_sta_l::R`](R) reader structure"]
impl crate::Readable for HFPorchStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`h_f_porch_sta_l::W`](W) writer structure"]
impl crate::Writable for HFPorchStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets H_F_PORCH_STA_L to value 0"]
impl crate::Resettable for HFPorchStaLSpec {
    const RESET_VALUE: u32 = 0;
}
