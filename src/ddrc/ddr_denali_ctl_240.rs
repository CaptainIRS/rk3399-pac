#[doc = "Register `DDR_DENALI_CTL_240` reader"]
pub type R = crate::R<DdrDenaliCtl240Spec>;
#[doc = "Register `DDR_DENALI_CTL_240` writer"]
pub type W = crate::W<DdrDenaliCtl240Spec>;
#[doc = "Field `RDLVL_CS_MAP` reader - Defines the chip select map for data eye training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
pub type RdlvlCsMapR = crate::FieldReader;
#[doc = "Field `RDLVL_CS_MAP` writer - Defines the chip select map for data eye training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
pub type RdlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDLVL_GATE_CS_MAP` reader - Defines the chip select map for gate training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
pub type RdlvlGateCsMapR = crate::FieldReader;
#[doc = "Field `RDLVL_GATE_CS_MAP` writer - Defines the chip select map for gate training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
pub type RdlvlGateCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDLVL_NORM_THRESHOLD_F0` reader - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_NORM_THRESHOLD_F0` writer - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Defines the chip select map for data eye training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
    #[inline(always)]
    pub fn rdlvl_cs_map(&self) -> RdlvlCsMapR {
        RdlvlCsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Defines the chip select map for gate training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
    #[inline(always)]
    pub fn rdlvl_gate_cs_map(&self) -> RdlvlGateCsMapR {
        RdlvlGateCsMapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_norm_threshold_f0(&self) -> RdlvlNormThresholdF0R {
        RdlvlNormThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the chip select map for data eye training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_cs_map(&mut self) -> RdlvlCsMapW<DdrDenaliCtl240Spec> {
        RdlvlCsMapW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Defines the chip select map for gate training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_cs_map(&mut self) -> RdlvlGateCsMapW<DdrDenaliCtl240Spec> {
        RdlvlGateCsMapW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_norm_threshold_f0(&mut self) -> RdlvlNormThresholdF0W<DdrDenaliCtl240Spec> {
        RdlvlNormThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_240::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_240::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl240Spec;
impl crate::RegisterSpec for DdrDenaliCtl240Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_240::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl240Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_240::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl240Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_240 to value 0"]
impl crate::Resettable for DdrDenaliCtl240Spec {
    const RESET_VALUE: u32 = 0;
}
