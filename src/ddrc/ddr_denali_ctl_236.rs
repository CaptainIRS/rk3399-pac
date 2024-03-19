#[doc = "Register `DDR_DENALI_CTL_236` reader"]
pub type R = crate::R<DdrDenaliCtl236Spec>;
#[doc = "Register `DDR_DENALI_CTL_236` writer"]
pub type W = crate::W<DdrDenaliCtl236Spec>;
#[doc = "Field `RDLVL_REQ` writer - User request to initiate data eye training. Set to 1 to trigger."]
pub type RdlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_GATE_REQ` writer - User request to initiate gate training. Set to 1 to trigger."]
pub type RdlvlGateReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_CS` reader - Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
pub type RdlvlCsR = crate::BitReader;
#[doc = "Field `RDLVL_CS` writer - Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
pub type RdlvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_SEQ_EN` reader - Specifies the pattern, format and MPR for data eye training."]
pub type RdlvlSeqEnR = crate::FieldReader;
#[doc = "Field `RDLVL_SEQ_EN` writer - Specifies the pattern, format and MPR for data eye training."]
pub type RdlvlSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 16 - Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
    #[inline(always)]
    pub fn rdlvl_cs(&self) -> RdlvlCsR {
        RdlvlCsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Specifies the pattern, format and MPR for data eye training."]
    #[inline(always)]
    pub fn rdlvl_seq_en(&self) -> RdlvlSeqEnR {
        RdlvlSeqEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - User request to initiate data eye training. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_req(&mut self) -> RdlvlReqW<DdrDenaliCtl236Spec> {
        RdlvlReqW::new(self, 0)
    }
    #[doc = "Bit 8 - User request to initiate gate training. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_req(&mut self) -> RdlvlGateReqW<DdrDenaliCtl236Spec> {
        RdlvlGateReqW::new(self, 8)
    }
    #[doc = "Bit 16 - Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_cs(&mut self) -> RdlvlCsW<DdrDenaliCtl236Spec> {
        RdlvlCsW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Specifies the pattern, format and MPR for data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_seq_en(&mut self) -> RdlvlSeqEnW<DdrDenaliCtl236Spec> {
        RdlvlSeqEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_236::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_236::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl236Spec;
impl crate::RegisterSpec for DdrDenaliCtl236Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_236::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl236Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_236::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl236Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_236 to value 0"]
impl crate::Resettable for DdrDenaliCtl236Spec {
    const RESET_VALUE: u32 = 0;
}
