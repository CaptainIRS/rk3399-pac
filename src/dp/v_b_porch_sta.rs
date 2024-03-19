#[doc = "Register `V_B_PORCH_STA` reader"]
pub type R = crate::R<VBPorchStaSpec>;
#[doc = "Register `V_B_PORCH_STA` writer"]
pub type W = crate::W<VBPorchStaSpec>;
#[doc = "Field `V_B_PORCH_STA` reader - V_B_PORCH (vertical back porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type VBPorchStaR = crate::FieldReader;
#[doc = "Field `V_B_PORCH_STA` writer - V_B_PORCH (vertical back porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type VBPorchStaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - V_B_PORCH (vertical back porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    pub fn v_b_porch_sta(&self) -> VBPorchStaR {
        VBPorchStaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - V_B_PORCH (vertical back porch) which is \n\ndetected by video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn v_b_porch_sta(&mut self) -> VBPorchStaW<VBPorchStaSpec> {
        VBPorchStaW::new(self, 0)
    }
}
#[doc = "Vertical Back Porch Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_b_porch_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_b_porch_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBPorchStaSpec;
impl crate::RegisterSpec for VBPorchStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`v_b_porch_sta::R`](R) reader structure"]
impl crate::Readable for VBPorchStaSpec {}
#[doc = "`write(|w| ..)` method takes [`v_b_porch_sta::W`](W) writer structure"]
impl crate::Writable for VBPorchStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets V_B_PORCH_STA to value 0"]
impl crate::Resettable for VBPorchStaSpec {
    const RESET_VALUE: u32 = 0;
}
