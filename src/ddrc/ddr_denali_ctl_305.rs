#[doc = "Register `DDR_DENALI_CTL_305` reader"]
pub type R = crate::R<DdrDenaliCtl305Spec>;
#[doc = "Register `DDR_DENALI_CTL_305` writer"]
pub type W = crate::W<DdrDenaliCtl305Spec>;
#[doc = "Field `RDLVL_RESP_MASK` reader - Mask for the dfi_rdlvl_resp signal during data eye training."]
pub type RdlvlRespMaskR = crate::FieldReader;
#[doc = "Field `RDLVL_RESP_MASK` writer - Mask for the dfi_rdlvl_resp signal during data eye training."]
pub type RdlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RDLVL_EN` reader - Enable the MC data eye training module. Set to 1 to enable."]
pub type RdlvlEnR = crate::BitReader;
#[doc = "Field `RDLVL_EN` writer - Enable the MC data eye training module. Set to 1 to enable."]
pub type RdlvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_GATE_EN` reader - Enable the MC gate training module. Set to 1 to enable."]
pub type RdlvlGateEnR = crate::BitReader;
#[doc = "Field `RDLVL_GATE_EN` writer - Enable the MC gate training module. Set to 1 to enable."]
pub type RdlvlGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Mask for the dfi_rdlvl_resp signal during data eye training."]
    #[inline(always)]
    pub fn rdlvl_resp_mask(&self) -> RdlvlRespMaskR {
        RdlvlRespMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable the MC data eye training module. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_en(&self) -> RdlvlEnR {
        RdlvlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable the MC gate training module. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_gate_en(&self) -> RdlvlGateEnR {
        RdlvlGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mask for the dfi_rdlvl_resp signal during data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_resp_mask(&mut self) -> RdlvlRespMaskW<DdrDenaliCtl305Spec> {
        RdlvlRespMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable the MC data eye training module. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_en(&mut self) -> RdlvlEnW<DdrDenaliCtl305Spec> {
        RdlvlEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable the MC gate training module. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_en(&mut self) -> RdlvlGateEnW<DdrDenaliCtl305Spec> {
        RdlvlGateEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_305::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_305::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl305Spec;
impl crate::RegisterSpec for DdrDenaliCtl305Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_305::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl305Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_305::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl305Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_305 to value 0"]
impl crate::Resettable for DdrDenaliCtl305Spec {
    const RESET_VALUE: u32 = 0;
}
