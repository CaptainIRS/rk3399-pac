#[doc = "Register `V_F_PORCH_STA` reader"]
pub type R = crate::R<VFPorchStaSpec>;
#[doc = "Register `V_F_PORCH_STA` writer"]
pub type W = crate::W<VFPorchStaSpec>;
#[doc = "Field `V_F_PORCH_STA` reader - V_F_PORCH (vertical front porch) which is detected \n\nby video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type VFPorchStaR = crate::FieldReader;
#[doc = "Field `V_F_PORCH_STA` writer - V_F_PORCH (vertical front porch) which is detected \n\nby video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
pub type VFPorchStaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - V_F_PORCH (vertical front porch) which is detected \n\nby video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    pub fn v_f_porch_sta(&self) -> VFPorchStaR {
        VFPorchStaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - V_F_PORCH (vertical front porch) which is detected \n\nby video capture module. \n\nThis bit field is valid only when STRM_VALID is \n\nhigh. And STRM_VALID becomes high when two \n\nsuccessive frames are determined as stable."]
    #[inline(always)]
    #[must_use]
    pub fn v_f_porch_sta(&mut self) -> VFPorchStaW<VFPorchStaSpec> {
        VFPorchStaW::new(self, 0)
    }
}
#[doc = "Vertical Front Porch Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v_f_porch_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v_f_porch_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VFPorchStaSpec;
impl crate::RegisterSpec for VFPorchStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`v_f_porch_sta::R`](R) reader structure"]
impl crate::Readable for VFPorchStaSpec {}
#[doc = "`write(|w| ..)` method takes [`v_f_porch_sta::W`](W) writer structure"]
impl crate::Writable for VFPorchStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets V_F_PORCH_STA to value 0x01"]
impl crate::Resettable for VFPorchStaSpec {
    const RESET_VALUE: u32 = 0x01;
}
