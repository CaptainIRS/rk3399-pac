#[doc = "Register `DDR_PI_REG_77` reader"]
pub type R = crate::R<DdrPiReg77Spec>;
#[doc = "Register `DDR_PI_REG_77` writer"]
pub type W = crate::W<DdrPiReg77Spec>;
#[doc = "Field `PI_RDLVL_GATE_ROTATE` reader - Enables rotational chip select for interval gate training. Set to 1 for\n\nrotating CS."]
pub type PiRdlvlGateRotateR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_ROTATE` writer - Enables rotational chip select for interval gate training. Set to 1 for\n\nrotating CS."]
pub type PiRdlvlGateRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_CS_MAP` reader - Defines the chip select map for data eye training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ndata eye training."]
pub type PiRdlvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_CS_MAP` writer - Defines the chip select map for data eye training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ndata eye training."]
pub type PiRdlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_GATE_CS_MAP` reader - Defines the chip select map for gate training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ngate training."]
pub type PiRdlvlGateCsMapR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_CS_MAP` writer - Defines the chip select map for gate training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ngate training."]
pub type PiRdlvlGateCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables rotational chip select for interval gate training. Set to 1 for\n\nrotating CS."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_rotate(&self) -> PiRdlvlGateRotateR {
        PiRdlvlGateRotateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Defines the chip select map for data eye training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ndata eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_cs_map(&self) -> PiRdlvlCsMapR {
        PiRdlvlCsMapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Defines the chip select map for gate training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ngate training."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_cs_map(&self) -> PiRdlvlGateCsMapR {
        PiRdlvlGateCsMapR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables rotational chip select for interval gate training. Set to 1 for\n\nrotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_rotate(&mut self) -> PiRdlvlGateRotateW<DdrPiReg77Spec> {
        PiRdlvlGateRotateW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Defines the chip select map for data eye training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ndata eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_cs_map(&mut self) -> PiRdlvlCsMapW<DdrPiReg77Spec> {
        PiRdlvlCsMapW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Defines the chip select map for gate training operations. Bit0\n\ncontrols cs0, bit1 controls cs1. Set each bit to 1 to enable chip for\n\ngate training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_cs_map(&mut self) -> PiRdlvlGateCsMapW<DdrPiReg77Spec> {
        PiRdlvlGateCsMapW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg77Spec;
impl crate::RegisterSpec for DdrPiReg77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_77::R`](R) reader structure"]
impl crate::Readable for DdrPiReg77Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_77::W`](W) writer structure"]
impl crate::Writable for DdrPiReg77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_77 to value 0"]
impl crate::Resettable for DdrPiReg77Spec {
    const RESET_VALUE: u32 = 0;
}
