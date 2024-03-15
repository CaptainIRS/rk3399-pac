#[doc = "Register `PI_REG_74` reader"]
pub type R = crate::R<PiReg74Spec>;
#[doc = "Register `PI_REG_74` writer"]
pub type W = crate::W<PiReg74Spec>;
#[doc = "Field `PI_ADDRESS_MIRRORING` reader - Indicates the chip selects that support address mirroring. Bit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable."]
pub type PiAddressMirroringR = crate::FieldReader;
#[doc = "Field `PI_ADDRESS_MIRRORING` writer - Indicates the chip selects that support address mirroring. Bit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable."]
pub type PiAddressMirroringW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_REQ` writer - Indicates user request to initiate data eye training. Set to 1 to trigger."]
pub type PiRdlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_GATE_REQ` writer - Indicates user request to initiate gate training. Set to 1 to trigger."]
pub type PiRdlvlGateReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_CS` reader - Specifies the target chip select for the data eye training operation that is initiated through the PI_REG_74.pi_rdlvl_req parameter or the gate training operation that is initiated through the PI_REG_74.pi_rdlvl_gate_req parameter."]
pub type PiRdlvlCsR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_CS` writer - Specifies the target chip select for the data eye training operation that is initiated through the PI_REG_74.pi_rdlvl_req parameter or the gate training operation that is initiated through the PI_REG_74.pi_rdlvl_gate_req parameter."]
pub type PiRdlvlCsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Indicates the chip selects that support address mirroring. Bit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn pi_address_mirroring(&self) -> PiAddressMirroringR {
        PiAddressMirroringR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 24:25 - Specifies the target chip select for the data eye training operation that is initiated through the PI_REG_74.pi_rdlvl_req parameter or the gate training operation that is initiated through the PI_REG_74.pi_rdlvl_gate_req parameter."]
    #[inline(always)]
    pub fn pi_rdlvl_cs(&self) -> PiRdlvlCsR {
        PiRdlvlCsR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Indicates the chip selects that support address mirroring. Bit0 controls cs0, bit1 controls cs1. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_address_mirroring(&mut self) -> PiAddressMirroringW<PiReg74Spec> {
        PiAddressMirroringW::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates user request to initiate data eye training. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_req(&mut self) -> PiRdlvlReqW<PiReg74Spec> {
        PiRdlvlReqW::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates user request to initiate gate training. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_req(&mut self) -> PiRdlvlGateReqW<PiReg74Spec> {
        PiRdlvlGateReqW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Specifies the target chip select for the data eye training operation that is initiated through the PI_REG_74.pi_rdlvl_req parameter or the gate training operation that is initiated through the PI_REG_74.pi_rdlvl_gate_req parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_cs(&mut self) -> PiRdlvlCsW<PiReg74Spec> {
        PiRdlvlCsW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_74::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_74::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg74Spec;
impl crate::RegisterSpec for PiReg74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_74::R`](R) reader structure"]
impl crate::Readable for PiReg74Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_74::W`](W) writer structure"]
impl crate::Writable for PiReg74Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_74 to value 0"]
impl crate::Resettable for PiReg74Spec {
    const RESET_VALUE: u32 = 0;
}
